use std::collections::HashMap;
use std::sync::Arc;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use russh::keys::ssh_key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use tokio::sync::Mutex;

use crate::app::App;
use crate::terminal::TerminalHandle;
use crate::ui;

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

/// Per-client state: a ratatui terminal and the app model.
type ClientState = (SshTerminal, App);

/// SSH server that serves the portfolio TUI to each connected client.
#[derive(Clone)]
pub struct AppServer {
    clients: Arc<Mutex<HashMap<usize, ClientState>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    /// Start the SSH server on the given address.
    pub async fn run(
        &mut self,
        config: Arc<russh::server::Config>,
        addr: (&str, u16),
    ) -> Result<(), anyhow::Error> {
        self.run_on_address(config, addr).await?;
        Ok(())
    }

    /// Re-render the TUI for a specific client.
    async fn render_client(&self, id: usize) {
        let mut clients = self.clients.lock().await;
        if let Some((terminal, app)) = clients.get_mut(&id) {
            let _ = terminal.draw(|f| {
                ui::render(app, f);
            });
        }
    }
}

impl Server for AppServer {
    type Handler = Self;

    fn new_client(&mut self, peer: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        log::info!(
            "New client connection (id={}) from {:?}",
            self.id,
            peer
        );
        s
    }
}

impl Handler for AppServer {
    type Error = anyhow::Error;

    /// Accept all connections without authentication.
    async fn auth_none(&mut self, _user: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    /// Also accept any public key (fallback for clients that try pubkey first).
    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    /// Client opens an SSH session channel — create the terminal + app.
    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let terminal_handle =
            TerminalHandle::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };
        let terminal = Terminal::with_options(backend, options)?;
        let app = App::new();

        self.clients.lock().await.insert(self.id, (terminal, app));

        Ok(true)
    }

    /// Client requests a PTY — capture the terminal dimensions and do the
    /// initial render.
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, _)) = clients.get_mut(&self.id) {
                terminal.resize(rect)?;
            }
        }

        // Initial render
        self.render_client(self.id).await;

        session.channel_success(channel)?;
        Ok(())
    }

    /// Client resized their terminal window.
    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, _)) = clients.get_mut(&self.id) {
                terminal.resize(rect)?;
            }
        }

        self.render_client(self.id).await;
        Ok(())
    }

    /// Client sent data (keypresses as raw bytes).
    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut should_quit = false;
        let mut needs_render = false;

        {
            let mut clients = self.clients.lock().await;
            if let Some((_, app)) = clients.get_mut(&self.id) {
                match data {
                    // 'q' or Ctrl-C — quit
                    b"q" | b"Q" | b"\x03" => {
                        app.quit();
                        should_quit = true;
                    }
                    // Right arrow (\x1b[C) or Tab (\t)
                    b"\x1b[C" | b"\t" => {
                        app.next_tab();
                        needs_render = true;
                    }
                    // Left arrow (\x1b[D) or Shift-Tab (\x1b[Z)
                    b"\x1b[D" | b"\x1b[Z" => {
                        app.prev_tab();
                        needs_render = true;
                    }
                    // '1', '2', '3' — jump to tab directly
                    b"1" => {
                        app.tab = crate::app::Tab::About;
                        needs_render = true;
                    }
                    b"2" => {
                        app.tab = crate::app::Tab::Projects;
                        needs_render = true;
                    }
                    b"3" => {
                        app.tab = crate::app::Tab::Contact;
                        needs_render = true;
                    }
                    _ => {
                        // Ignore unknown input
                    }
                }
            }
        }

        if should_quit {
            self.clients.lock().await.remove(&self.id);
            session.close(channel)?;
        } else if needs_render {
            self.render_client(self.id).await;
        }

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            clients.lock().await.remove(&id);
        });
    }
}
