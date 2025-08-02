use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

static GLOBAL_VERBOSE: AtomicBool = AtomicBool::new(false);

pub struct Logger {}

impl Logger {
    pub fn verbose_mode() {
        GLOBAL_VERBOSE.store(true, Ordering::Relaxed);
    }

    pub fn ok(text: String) {
        println!("[{}]: {}", "OK".bright_green(), text);
    }

    pub fn error(text: String) {
        println!("[{}]: {}", "ERROR".bright_red(), text);
    }

    pub fn process(text: String) {
        if GLOBAL_VERBOSE.load(Ordering::Relaxed) {
            println!("[{}]: {}", "PROCESS".bright_purple(), text);
        }
    }

    pub fn info(text: String) {
        println!("[{}]: {}", "INFO".bright_cyan(), text);
    }
}
