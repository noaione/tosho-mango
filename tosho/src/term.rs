use anstream::println;
use color_print::cformat;
use inquire::{Confirm, Select};

#[derive(Clone)]
pub struct ConsoleChoice {
    /// The name of the choice (also the key)
    pub name: String,
    /// The value of the choice (the value that would be shown)
    pub value: String,
}

impl std::fmt::Display for ConsoleChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Terminal {
    debug: u8,
}

impl Terminal {
    fn new(debug: u8) -> Self {
        Self { debug }
    }

    /// Log info to terminal
    pub fn info(&self, msg: &str) {
        println!("{}", cformat!("[<cyan,strong>INFO</cyan,strong>] {}", msg))
    }

    /// Log warning to terminal
    pub fn warn(&self, msg: &str) {
        println!(
            "{}",
            cformat!("[<yellow,strong>WARN</yellow,strong>] {}", msg)
        )
    }

    /// Log error to terminal
    pub fn error(&self, msg: &str) {
        println!("{}", cformat!("[<red,strong>ERROR</red,strong>] {}", msg))
    }

    /// Log to terminal
    #[allow(dead_code)]
    pub fn log(&self, msg: &str) {
        if self.debug > 1 {
            println!(
                "{}",
                cformat!("[<magenta,strong>LOG</magenta,strong>] {}", msg)
            )
        }
    }

    #[allow(dead_code)]
    pub fn trace(&self, msg: &str) {
        if self.debug > 2 {
            println!("{}", cformat!("[<blue,strong>TRACE</blue,strong>] {}", msg))
        }
    }

    /// Do a confirmation prompt
    pub fn confirm(&self, prompt: Option<&str>) -> bool {
        let prompt = prompt.unwrap_or("Are you sure?");

        Confirm::new(prompt)
            .with_default(false)
            .prompt()
            .unwrap_or(false)
    }

    /// Do a single choice prompt
    pub fn choice(&self, prompt: &str, choices: Vec<ConsoleChoice>) -> Option<ConsoleChoice> {
        let choice = Select::new(prompt, choices).prompt_skippable();

        match choice {
            Ok(choice) => choice,
            Err(_) => None,
        }
    }
}

/// Get the root console instance
pub fn get_console(debug: u8) -> Terminal {
    Terminal::new(debug)
}

/// Create a clickable link/text in terminal
///
/// Ref: [`GitHub Gist`](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda)
#[macro_export]
macro_rules! linkify {
    ($url:expr, $text:expr) => {
        match supports_hyperlinks::on(supports_hyperlinks::Stream::Stdout) {
            true => format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", $url, $text),
            false => $url.to_string(),
        }
    };
    ($url:expr) => {
        linkify!($url, $url)
    };
}
