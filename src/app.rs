use crate::content;

/// Active tab in the portfolio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    About,
    Projects,
    Skills,
    Contact,
}

impl Tab {
    pub const ALL: [Tab; 4] = [Tab::About, Tab::Projects, Tab::Skills, Tab::Contact];

    pub fn label(&self) -> &'static str {
        match self {
            Tab::About => "About",
            Tab::Projects => "Projects",
            Tab::Skills => "Skills",
            Tab::Contact => "Contact",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::About => 0,
            Tab::Projects => 1,
            Tab::Skills => 2,
            Tab::Contact => 3,
        }
    }

    pub fn from_index(i: usize) -> Option<Tab> {
        Tab::ALL.get(i).copied()
    }
}

/// Intro animation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntroPhase {
    /// Typewriter effect — reveals chars_shown characters of the banner.
    Typing { chars_shown: usize },
    /// Brief pause after typing finishes before showing the full UI.
    Pause { ticks_remaining: u8 },
    /// Animation done — show normal UI.
    Done,
}

/// Application state for a single SSH client session.
pub struct App {
    pub tab: Tab,
    pub should_quit: bool,
    pub scroll_offset: usize,
    pub intro: IntroPhase,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::About,
            should_quit: false,
            scroll_offset: 0,
            intro: IntroPhase::Typing { chars_shown: 0 },
        }
    }

    pub fn next_tab(&mut self) {
        let idx = self.tab.index();
        let next = (idx + 1) % Tab::ALL.len();
        self.tab = Tab::ALL[next];
        self.scroll_offset = 0;
    }

    pub fn prev_tab(&mut self) {
        let idx = self.tab.index();
        let prev = if idx == 0 {
            Tab::ALL.len() - 1
        } else {
            idx - 1
        };
        self.tab = Tab::ALL[prev];
        self.scroll_offset = 0;
    }

    pub fn go_to_tab(&mut self, idx: usize) {
        if let Some(tab) = Tab::from_index(idx) {
            self.tab = tab;
            self.scroll_offset = 0;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // ── Scrolling ──────────────────────────────────────────────

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, content_height: usize, viewport_height: usize) {
        if content_height > viewport_height {
            let max = content_height - viewport_height;
            if self.scroll_offset < max {
                self.scroll_offset += 1;
            }
        }
    }

    /// Returns the total number of content lines for the current tab.
    pub fn content_line_count(&self) -> usize {
        match self.tab {
            Tab::About => content::ABOUT_LINES.len(),
            Tab::Projects => content::total_project_lines(),
            Tab::Skills => {
                let mut lines = 0;
                for (i, _group) in content::SKILLS.iter().enumerate() {
                    if i > 0 {
                        lines += 1; // blank separator
                    }
                    lines += 1; // group header
                    lines += 1; // items line
                }
                lines
            }
            Tab::Contact => {
                // intro + blank + entries + blank + outro
                1 + 1 + content::CONTACT_ENTRIES.len() + 1 + 1
            }
        }
    }

    // ── Intro animation ────────────────────────────────────────

    /// Advance the typewriter animation by `chars` characters.
    /// Returns `true` if the state changed (needs re-render).
    pub fn advance_intro(&mut self, chars: usize) -> bool {
        match self.intro {
            IntroPhase::Typing { chars_shown } => {
                let total = content::banner_char_count();
                let next = (chars_shown + chars).min(total);
                if next >= total {
                    self.intro = IntroPhase::Pause { ticks_remaining: 8 };
                } else {
                    self.intro = IntroPhase::Typing { chars_shown: next };
                }
                true
            }
            IntroPhase::Pause { ticks_remaining } => {
                if ticks_remaining <= 1 {
                    self.intro = IntroPhase::Done;
                } else {
                    self.intro = IntroPhase::Pause {
                        ticks_remaining: ticks_remaining - 1,
                    };
                }
                true
            }
            IntroPhase::Done => false,
        }
    }

    /// Skip the intro animation immediately.
    pub fn skip_intro(&mut self) {
        self.intro = IntroPhase::Done;
    }

    pub fn intro_done(&self) -> bool {
        matches!(self.intro, IntroPhase::Done)
    }
}
