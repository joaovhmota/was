use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Local;
use colored::{ColoredString, Colorize};

static IS_VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn enable_verbose() {
    IS_VERBOSE.store(true, Ordering::Relaxed);
}

fn print_pretty_log<T: AsRef<str>>(log_type: ColoredString, message: T) {
    let log_time = Local::now();
    let formatted_log_time = log_time
        .format("[%d/%m/%Y %H:%M:%S]")
        .to_string()
        .bright_black();
    let message_reference = message.as_ref();

    println!("{formatted_log_time} {log_type}: {message_reference}");
}

pub fn info<T: AsRef<str>>(message: T) {
    if !IS_VERBOSE.load(Ordering::Relaxed) {
        return;
    }

    print_pretty_log("INFO".bright_purple(), message);
}

pub fn ok<T: AsRef<str>>(message: T) {
    print_pretty_log("OK".bright_green(), message);
}

pub fn warn<T: AsRef<str>>(message: T) {
    print_pretty_log("WARNING".bright_yellow(), message);
}

pub fn error<T: AsRef<str>>(message: T) {
    print_pretty_log("ERROR".bright_red(), message);
}
