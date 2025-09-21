use chrono::Local;

const LOG_TIME_PREFIX_TEMPLATE: &'static str = "[%d/%m/%Y %H:%M:%S]";
const MIGRATIONS_TIME_PREFIX_TEMPLATE: &'static str = "%Y-%m-%d-%H-%M-%S-%f";

pub fn migrations_time_string() -> String {
    Local::now()
        .format(MIGRATIONS_TIME_PREFIX_TEMPLATE)
        .to_string()
}

pub fn log_time_string() -> String {
    Local::now().format(LOG_TIME_PREFIX_TEMPLATE).to_string()
}
