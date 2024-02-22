use std::time::Duration;

use anstream::println;
use color_print::cformat;
use indicatif::ProgressStyle;
use inquire::{Confirm, MultiSelect, Select};

#[derive(Clone, Debug)]
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

#[derive(Clone)]
pub struct Terminal {
    debug: u8,
    #[cfg(windows)]
    modern_win: bool,
    current_spinner: Option<indicatif::ProgressBar>,
    current_progress: Option<indicatif::ProgressBar>,
}

impl Terminal {
    fn new(debug: u8) -> Self {
        #[cfg(windows)]
        let modern_win = super::win_term::check_windows_vt_support();

        Self {
            debug,
            #[cfg(windows)]
            modern_win,
            current_spinner: None,
            current_progress: None,
        }
    }

    /// Check if we in debug mode
    pub fn is_debug(&self) -> bool {
        self.debug > 0
    }

    /// Log info to terminal
    pub fn info(&self, msg: &str) {
        println!("{}", cformat!(" [<cyan,strong>INFO</cyan,strong>] {}", msg))
    }

    /// Log warning to terminal
    pub fn warn(&self, msg: &str) {
        println!(
            "{}",
            cformat!(" [<yellow,strong>WARN</yellow,strong>] {}", msg)
        )
    }

    /// Log error to terminal
    pub fn error(&self, msg: &str) {
        println!("{}", cformat!("[<red,strong>ERROR</red,strong>] {}", msg))
    }

    /// Log to terminal
    pub fn log(&self, msg: &str) {
        if self.debug >= 1 {
            println!(
                "{}",
                cformat!("  [<magenta,strong>LOG</magenta,strong>] {}", msg)
            )
        }
    }

    // pub fn trace(&self, msg: &str) {
    //     if self.debug >= 2 {
    //         println!("{}", cformat!("[<blue,strong>TRACE</blue,strong>] {}", msg))
    //     }
    // }

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

    /// Do a multiple choice prompt
    pub fn select(&self, prompt: &str, choices: Vec<ConsoleChoice>) -> Option<Vec<ConsoleChoice>> {
        let choice = MultiSelect::new(prompt, choices).prompt_skippable();

        match choice {
            Ok(choice) => choice,
            Err(_) => None,
        }
    }

    fn make_spinner(&self) -> indicatif::ProgressBar {
        let spinner = indicatif::ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(120));
        spinner.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "⠋",
                    "⠙",
                    "⠹",
                    "⠸",
                    "⠼",
                    "⠴",
                    "⠦",
                    "⠧",
                    "⠇",
                    "⠏",
                    &cformat!(" [<cyan,strong>INFO</cyan,strong>]"),
                ]),
        );
        spinner
    }

    /// Do a status spinner
    pub fn status(&mut self, prompt: String) {
        match self.current_spinner.as_mut() {
            Some(spinner) => {
                spinner.set_message(prompt);
            }
            None => {
                let spinner = self.make_spinner();
                spinner.set_message(prompt);
                self.current_spinner = Some(spinner);
            }
        }
    }

    /// Stop the current spinner
    // pub fn stop_status(&mut self) {
    //     match self.current_spinner.as_mut() {
    //         Some(spinner) => {
    //             spinner.finish();
    //             self.current_spinner = None;
    //         }
    //         None => {}
    //     }
    // }

    /// Stop the current spinner with a message
    pub fn stop_status_msg(&mut self, msg: String) {
        if let Some(spinner) = self.current_spinner.as_mut() {
            spinner.finish_with_message(msg);
            self.current_spinner = None;
        }
    }

    fn make_progress(&self, len: u64, message: Option<String>) -> indicatif::ProgressBar {
        let progress = indicatif::ProgressBar::new(len);
        progress.enable_steady_tick(Duration::from_millis(120));
        progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.blue} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        let message = message.unwrap_or("Processing".to_string());
        progress.set_message(message);
        progress
    }

    /// Do a progress bar
    pub fn progress(&mut self, init_len: u64, incr: u64, message: Option<String>) {
        match self.current_progress.as_mut() {
            Some(progress) => {
                progress.inc(incr);
                if let Some(message) = message {
                    progress.set_message(message);
                }
            }
            None => {
                let progress = self.make_progress(init_len, message);
                self.current_progress = Some(progress);
            }
        }
    }

    /// Stop the current progress bar
    pub fn stop_progress(&mut self, message: Option<String>) {
        if let Some(progress) = self.current_progress.as_mut() {
            match message {
                Some(message) => progress.finish_with_message(message),
                None => progress.finish(),
            }
            self.current_progress = None;
        }
    }

    /// Is the terminal modern?
    ///
    /// Assume yes if not on Windows
    pub fn is_modern(&self) -> bool {
        #[cfg(windows)]
        {
            self.modern_win
        }
        #[cfg(not(windows))]
        {
            true
        }
    }
}

/// Get the root console instance
pub fn get_console(debug: u8) -> Terminal {
    Terminal::new(debug)
}

pub(crate) mod macros {
    /// Create a clickable link/text in terminal
    ///
    /// Ref: [`GitHub Gist`](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda)
    macro_rules! linkify {
        ($url:expr, $text:expr) => {
            match supports_hyperlinks::on(supports_hyperlinks::Stream::Stdout) {
                true => format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", $url, $text),
                false => match $crate::win_term::check_windows_vt_support() {
                    true => format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", $url, $text),
                    false => $text.to_string(),
                },
            }
        };
        ($url:expr) => {
            linkify!($url, $url)
        };
    }

    pub(crate) use linkify;
}
