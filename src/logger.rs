use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

use super::args::LogLevel;
use env_logger::{Builder, Target};
use lazy_static::lazy_static;
use log::LevelFilter;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
}

/// `Logger` is a struct that contains a reference to a string, a `LogLevel` enum, and an optional
/// `File`.
///
/// Properties:
///
/// * `stack`: The name of the stack that the logger is associated with.
/// * `log_level`: This is the log level that the logger will use.
/// * `log_file`: This is the file that the logger will write to.
pub struct Logger {
    stack: &'static str,
    log_level: LogLevel,
    log_file: Option<File>,
}

pub fn set_stack(stack: &'static str) {
    let mut logger = LOGGER.lock().unwrap();
    logger.stack = stack;
}

pub fn set_verbosity(level: LogLevel) {
    let mut logger = LOGGER.lock().unwrap();
    logger.log_level = level;
}

pub fn set_logfile(log_file: &Option<String>) -> Result<(), std::io::Error> {
    let mut logger = LOGGER.lock().unwrap();
    logger.log_file = match log_file {
        Some(path) => Some(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)?,
        ),
        None => None,
    };
    Ok(())
}

/// This function sets up the logger
pub fn init() {
    let logger = LOGGER.lock().unwrap();
    let verbosity = match logger.log_level {
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Error => LevelFilter::Error,
    };
    Builder::new()
        .filter(Some(logger.stack), verbosity)
        .target(Target::Stdout)
        .init();
}

/// Writes a log message to a file
///
/// Arguments:
///
/// * `level`: The log level of the message.
/// * `message`: The message to log.
fn log_to_file(logger: &mut Logger, level: LogLevel, message: &impl Display) {
    if let Some(ref mut file) = logger.log_file {
        let level = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO ",
            LogLevel::Warn => "WARN ",
            LogLevel::Error => "ERROR",
        };
        file.write_all(
            format!(
                "[{:?} {} {}] {}\n",
                chrono::offset::Local::now(),
                level,
                logger.stack,
                message
            )
            .as_bytes(),
        )
        .unwrap();
    }
}

/// Logs a message if the log level is set to debug.
///
/// Arguments:
///
/// * `message`: message to be logged
pub fn debug(message: impl Display) {
    let mut logger = LOGGER.lock().unwrap();
    if logger.log_level == LogLevel::Debug {
        log_to_file(&mut logger, LogLevel::Debug, &message);
    }
    log::debug!(target: logger.stack, "{}", message);
}
/// Logs a message if the log level is set to info.
///
/// Arguments:
///
/// * `message`: message to be logged
pub fn info(message: impl Display) {
    let mut logger = LOGGER.lock().unwrap();
    if logger.log_level <= LogLevel::Info {
        log_to_file(&mut logger, LogLevel::Info, &message);
    }
    log::info!(target: logger.stack, "{}", message);
}
/// Logs a message if the log level is set to warning.
///
/// Arguments:
///
/// * `message`: message to be logged
pub fn warn(message: impl Display) {
    let mut logger = LOGGER.lock().unwrap();
    if logger.log_level <= LogLevel::Warn {
        log_to_file(&mut logger, LogLevel::Warn, &message);
    }
    log::warn!(target: logger.stack, "{}", message);
}
/// Logs a message if the log level is set to error.
///
/// Arguments:
///
/// * `message`: message to be logged
pub fn error(message: impl Display) {
    let mut logger = LOGGER.lock().unwrap();
    if logger.log_level <= LogLevel::Error {
        log_to_file(&mut logger, LogLevel::Error, &message);
    }
    log::error!(target: logger.stack, "{}", message);
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            stack: "unknown",
            log_level: LogLevel::Info,
            log_file: None,
        }
    }
}
