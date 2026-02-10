// ── ASCII Art Banner ───────────────────────────────────────────
//
// Compact "slant"-style banner for the header.
// Each line is a &str so the renderer can reveal them character by
// character during the intro animation.

pub const BANNER: &[&str] = &[
    r" __   __                _      _    ",
    r" \ \ / /_ _ _ __  _ __ (_) ___| | __",
    r"  \ V / _` | '_ \| '_ \| |/ __| |/ /",
    r"   | | (_| | | | | | | | | (__|   < ",
    r"   |_|\__,_|_| |_|_| |_|_|\___|_|\_\",
];

/// Total number of characters in the banner (for the typewriter animation).
pub fn banner_char_count() -> usize {
    BANNER.iter().map(|l| l.len()).sum::<usize>() + BANNER.len() // +newlines
}

// ── About ──────────────────────────────────────────────────────

pub const ABOUT_LINES: &[&str] = &[
    "Hi there \u{1F44B}, I'm Yannick!",
    "",
    "I'm a freelance dev based in France, obsessed with",
    "building things and learning new stuff.",
    "",
    "If an idea gets stuck in my head, I'll probably end",
    "up building it.",
    "",
    "My go-to stack is React Native, Expo, TypeScript,",
    "and Next.js. I spend most of my time in mobile and",
    "web development but I'm always curious about new",
    "tools and technologies.",
    "",
    "Currently shipping Doku \u{2013} a French learning app",
    "with graded stories and a cute cat mascot \u{1F431}",
    "",
    "I believe in learning by doing, always building",
    "something, and improving along the way.",
    "",
    "Open to freelance opportunities \u{2013} hello@yannickh.dev",
];

// ── Projects ───────────────────────────────────────────────────

pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tech: &'static str,
    pub url: &'static str,
}

pub struct ProjectCategory {
    pub name: &'static str,
    pub projects: &'static [Project],
}

pub const PROJECT_CATEGORIES: &[ProjectCategory] = &[
    ProjectCategory {
        name: "Language Learning",
        projects: &[
            Project {
                name: "Doku",
                description: "iOS app for learning French through graded stories",
                tech: "React Native, Expo, TypeScript",
                url: "https://learnfrenchwithdoku.app",
            },
            Project {
                name: "kanidachi",
                description: "WaniKani client for Android and iOS",
                tech: "React Native, Expo, TypeScript",
                url: "https://github.com/YannickHerrero/kanidachi",
            },
            Project {
                name: "yomu",
                description: "iOS Japanese reading assistant with offline dictionary and SRS",
                tech: "React Native, TypeScript",
                url: "https://github.com/YannickHerrero/Yomu",
            },
        ],
    },
    ProjectCategory {
        name: "Media & Streaming",
        projects: &[
            Project {
                name: "mira",
                description: "Cross-platform streaming app for movies and TV shows",
                tech: "React Native, TypeScript",
                url: "https://github.com/YannickHerrero/mira",
            },
            Project {
                name: "miru",
                description: "Terminal-native anime streaming CLI with Anilist + Real-Debrid",
                tech: "Rust",
                url: "https://github.com/YannickHerrero/miru",
            },
        ],
    },
    ProjectCategory {
        name: "Terminal Fun",
        projects: &[
            Project {
                name: "Solaris",
                description: "Terminal idle game — harness the cosmos to generate energy",
                tech: "Rust, ratatui",
                url: "https://github.com/YannickHerrero/Solaris",
            },
            Project {
                name: "Balatrust",
                description: "A terminal-based Balatro clone",
                tech: "Rust",
                url: "https://github.com/YannickHerrero/Balatrust",
            },
            Project {
                name: "kanitomo",
                description: "Terminal mini-game collection with your pet crab companion",
                tech: "Rust",
                url: "https://github.com/YannickHerrero/kanitomo",
            },
        ],
    },
    ProjectCategory {
        name: "Tools & Productivity",
        projects: &[
            Project {
                name: "mtools",
                description: "Unified toolkit for work management and developer utilities",
                tech: "TypeScript",
                url: "https://github.com/YannickHerrero/mtools",
            },
            Project {
                name: "motionflow",
                description: "Pipeline for generating short-form French educational videos",
                tech: "TypeScript, AI",
                url: "https://github.com/YannickHerrero/motionflow",
            },
            Project {
                name: "life",
                description: "Personal habit tracking for learning, nutrition, and sport",
                tech: "TypeScript",
                url: "https://github.com/YannickHerrero/life",
            },
        ],
    },
    ProjectCategory {
        name: "Web & Config",
        projects: &[
            Project {
                name: "yannickh.dev",
                description: "Personal portfolio and project showcase",
                tech: "Next.js, TypeScript, Vercel",
                url: "https://yannickh.dev",
            },
            Project {
                name: "ssh-yannickh.dev",
                description: "This SSH portfolio you're looking at right now",
                tech: "Rust, russh, ratatui, Fly.io",
                url: "https://github.com/YannickHerrero/ssh-yannickh.dev",
            },
            Project {
                name: "windot",
                description: "Windows/WSL dotfiles with tiling WM and custom status bar",
                tech: "JavaScript, PowerShell",
                url: "https://github.com/YannickHerrero/windot",
            },
            Project {
                name: "chocofi-config",
                description: "ZMK firmware config for Corne/Chocofi split keyboard",
                tech: "ZMK, Devicetree",
                url: "https://github.com/YannickHerrero/chocofi-config",
            },
        ],
    },
];

/// Flat count of all projects across categories.
pub fn total_project_lines() -> usize {
    let mut lines = 0;
    for (i, cat) in PROJECT_CATEGORIES.iter().enumerate() {
        if i > 0 {
            lines += 1; // blank separator between categories
        }
        lines += 1; // category header
        lines += 1; // blank after header
        for (j, _) in cat.projects.iter().enumerate() {
            if j > 0 {
                lines += 1; // blank between projects
            }
            lines += 4; // name, desc, tech, url
        }
    }
    lines
}

/// Total number of projects across all categories.
pub fn total_project_count() -> usize {
    PROJECT_CATEGORIES
        .iter()
        .map(|cat| cat.projects.len())
        .sum()
}

/// Map a flat project index (0-based) to the corresponding category and project.
/// Returns `None` if the index is out of range.
pub fn get_project_by_flat_index(
    idx: usize,
) -> Option<(&'static ProjectCategory, &'static Project)> {
    let mut remaining = idx;
    for cat in PROJECT_CATEGORIES {
        if remaining < cat.projects.len() {
            return Some((cat, &cat.projects[remaining]));
        }
        remaining -= cat.projects.len();
    }
    None
}

// ── Skills ─────────────────────────────────────────────────────

pub struct SkillGroup {
    pub name: &'static str,
    pub items: &'static [&'static str],
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        name: "Languages",
        items: &["Rust", "TypeScript", "JavaScript", "Python", "Lua", "CSS"],
    },
    SkillGroup {
        name: "Mobile",
        items: &["React Native", "Expo"],
    },
    SkillGroup {
        name: "Web",
        items: &["Next.js", "Astro", "Svelte", "React"],
    },
    SkillGroup {
        name: "Terminal / TUI",
        items: &["ratatui", "crossterm", "russh"],
    },
    SkillGroup {
        name: "Infrastructure",
        items: &["Docker", "Fly.io", "Vercel", "GitHub Actions"],
    },
    SkillGroup {
        name: "Other",
        items: &["ZMK firmware", "Base16 theming", "WSL / Hyprland"],
    },
];

// ── Contact ────────────────────────────────────────────────────

pub struct ContactEntry {
    pub label: &'static str,
    pub value: &'static str,
}

pub const CONTACT_ENTRIES: &[ContactEntry] = &[
    ContactEntry {
        label: "Web",
        value: "https://yannickh.dev",
    },
    ContactEntry {
        label: "GitHub",
        value: "https://github.com/YannickHerrero",
    },
    ContactEntry {
        label: "Email",
        value: "hello@yannickh.dev",
    },
    ContactEntry {
        label: "Doku",
        value: "https://learnfrenchwithdoku.app",
    },
];

pub const CONTACT_OUTRO: &str = "Open to freelance opportunities — feel free to reach out.";
