use anstream::println;
use color_print::cformat;
use dialoguer::{theme::ColorfulTheme, Confirm};

pub struct Terminal {
    debug: u8,
}

impl Terminal {
    fn new(debug: u8) -> Self {
        Self { debug }
    }

    /// Log info to terminal
    pub fn info(&self, msg: String) {
        println!("{}", cformat!("[<c><s>INFO</></>] {}", msg))
    }

    /// Log warning to terminal
    pub fn warn(&self, msg: String) {
        println!("{}", cformat!("[<y><s>WARN</></>] {}", msg))
    }

    /// Log error to terminal
    pub fn error(&self, msg: String) {
        println!("{}", cformat!("[<r><s>ERROR</></>] {}", msg))
    }

    /// Log to terminal
    pub fn log(&self, msg: String) {
        println!("{}", cformat!("[<m><s>LOG</s></m>] {}", msg))
    }

    /// Create a new line
    pub fn enter(&self) {
        println!()
    }

    // Do a confirmation prompt
    pub fn confirm(&self, prompt: Option<&str>) -> bool {
        let prompt = prompt.unwrap_or("Are you sure?");

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(false)
            .show_default(true)
            .interact()
            .unwrap()
    }
}

/// Get the root console instance
pub fn get_console(debug: u8) -> Terminal {
    Terminal::new(debug)
}
