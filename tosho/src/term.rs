use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use anstream::println;
use color_print::cformat;
use indicatif::ProgressStyle;
use inquire::{Confirm, MultiSelect, Select};

pub(crate) static IS_WIN_VT_SUPPORTED: LazyLock<bool> = LazyLock::new(|| {
    if ::supports_hyperlinks::on(::supports_hyperlinks::Stream::Stdout) {
        true
    } else {
        crate::win_term::check_windows_vt_support()
    }
});

#[derive(Clone, Debug)]
pub struct ConsoleChoice {
    /// The name of the choice (also the key)
    pub name: String,
    /// The value of the choice (the value that would be shown)
    pub value: String,
}

impl ConsoleChoice {
    /// Create a new console choice
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
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
    pub fn info(&self, msg: impl AsRef<str>) {
        println!(
            "{}",
            cformat!(" [<cyan,strong>INFO</cyan,strong>] {}", msg.as_ref())
        )
    }

    /// Log warning to terminal
    pub fn warn(&self, msg: impl AsRef<str>) {
        println!(
            "{}",
            cformat!(" [<yellow,strong>WARN</yellow,strong>] {}", msg.as_ref())
        )
    }

    /// Log error to terminal
    pub fn error(&self, msg: impl AsRef<str>) {
        println!(
            "{}",
            cformat!("[<red,strong>ERROR</red,strong>] {}", msg.as_ref())
        )
    }

    /// Log to terminal
    pub fn log(&self, msg: impl AsRef<str>) {
        if self.debug >= 1 {
            println!(
                "{}",
                cformat!("  [<magenta,strong>LOG</magenta,strong>] {}", msg.as_ref())
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

        choice.unwrap_or_default()
    }

    /// Do a multiple choice prompt
    pub fn select(&self, prompt: &str, choices: Vec<ConsoleChoice>) -> Option<Vec<ConsoleChoice>> {
        let choice = MultiSelect::new(prompt, choices).prompt_skippable();

        choice.unwrap_or_default()
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

    // /// Stop the current spinner
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

    /// Make a new progress bar
    pub fn make_progress(
        &self,
        len: u64,
        message: Option<impl Into<String>>,
    ) -> indicatif::ProgressBar {
        let progress = indicatif::ProgressBar::new(len);
        progress.enable_steady_tick(std::time::Duration::from_millis(120));
        progress.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.blue} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "]),
        );
        match message {
            Some(msg) => {
                progress.set_message(msg.into());
            }
            None => {
                progress.set_message("Processing");
            }
        }
        progress
    }

    /// Make a new progress bar
    ///
    /// Similar to [`Terminal::make_progress`] but returns an [`Arc`] of the
    /// progress bar so it can be shared across threads without
    /// recreating the progress bar again.
    pub fn make_progress_arc(
        &self,
        len: u64,
        message: Option<impl Into<String>>,
    ) -> Arc<indicatif::ProgressBar> {
        let progress = self.make_progress(len, message);
        Arc::new(progress)
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

    /// Clear the terminal screen
    pub fn clear_screen(&self) {
        if self.is_modern() {
            print!("\x1B[2J\x1B[1;1H");
            print!("\x1B[3J");
            use std::io::{self, Write};
            let _ = io::stdout().flush();
        } else {
            // Fallback for older terminals - print enough newlines to "clear" screen
            for _ in 0..50 {
                println!();
            }
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
        ($url:expr_2021, $text:expr_2021) => {
            if *$crate::term::IS_WIN_VT_SUPPORTED {
                format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", $url, $text)
            } else {
                $text.to_string()
            }
        };
        ($url:expr_2021) => {
            linkify!($url, $url)
        };
    }

    pub(crate) use linkify;
}
