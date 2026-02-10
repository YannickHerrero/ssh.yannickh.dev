/// Active tab in the portfolio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    About,
    Projects,
    Contact,
}

impl Tab {
    pub const ALL: [Tab; 3] = [Tab::About, Tab::Projects, Tab::Contact];

    pub fn label(&self) -> &'static str {
        match self {
            Tab::About => "About",
            Tab::Projects => "Projects",
            Tab::Contact => "Contact",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::About => 0,
            Tab::Projects => 1,
            Tab::Contact => 2,
        }
    }
}

/// A project entry for the Projects tab.
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub tech: &'static str,
}

/// Application state for a single SSH client session.
pub struct App {
    pub tab: Tab,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::About,
            should_quit: false,
        }
    }

    pub fn next_tab(&mut self) {
        let idx = self.tab.index();
        let next = (idx + 1) % Tab::ALL.len();
        self.tab = Tab::ALL[next];
    }

    pub fn prev_tab(&mut self) {
        let idx = self.tab.index();
        let prev = if idx == 0 {
            Tab::ALL.len() - 1
        } else {
            idx - 1
        };
        self.tab = Tab::ALL[prev];
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // ── Content data ───────────────────────────────────────────

    pub fn about_text(&self) -> &'static str {
        concat!(
            "Hey, I'm Yannick Herrero.\n",
            "\n",
            "I'm a software engineer who enjoys building things\n",
            "that live on the web — and apparently also in your terminal.\n",
            "\n",
            "I care about clean code, good developer experience,\n",
            "and shipping software that actually works.\n",
            "\n",
            "This SSH portfolio was built with Rust, using russh\n",
            "and ratatui. Feel free to look around.",
        )
    }

    pub fn projects(&self) -> Vec<Project> {
        vec![
            Project {
                name: "yannickh.dev",
                description: "My personal website & portfolio",
                url: "https://yannickh.dev",
                tech: "Next.js, TypeScript, Vercel",
            },
            Project {
                name: "ssh-yannickh.dev",
                description: "This SSH portfolio you're looking at right now",
                url: "https://github.com/yannickh/ssh-yannickh.dev",
                tech: "Rust, russh, ratatui, Fly.io",
            },
        ]
    }

    pub fn contact_text(&self) -> &'static str {
        concat!(
            "Want to get in touch? Here's where you can find me:\n",
            "\n",
            "  Web       https://yannickh.dev\n",
            "  GitHub    https://github.com/yannickh\n",
            "  Email     hello@yannickh.dev\n",
            "\n",
            "Feel free to reach out — I'm always happy to chat.",
        )
    }
}
