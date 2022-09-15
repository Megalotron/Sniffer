use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Mutex, MutexGuard};

use env_logger::{Builder, Target};
use lazy_static::lazy_static;
use log::LevelFilter;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

#[derive(Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

pub struct Logger {
    stack: &'static str,
    log_level: LogLevel,
    log_file: Option<File>,
}

pub fn get() -> MutexGuard<'static, Logger> {
    LOGGER.lock().unwrap()
}

pub fn init() -> MutexGuard<'static, Logger> {
    lazy_static::initialize(&LOGGER);
    LOGGER.lock().unwrap()
}

impl Logger {
    fn new() -> Self {
        Logger {
            stack: "unknown",
            log_level: LogLevel::Info,
            log_file: None,
        }
    }

    pub fn stack(&mut self, stack: &'static str) -> &mut Self {
        self.stack = stack;
        self
    }

    pub fn verbosity(&mut self, level: LogLevel) -> &mut Self {
        self.log_level = level;
        self
    }

    pub fn logfile(&mut self, log_file: &Option<String>) -> Result<&mut Self, std::io::Error> {
        self.log_file = match log_file {
            Some(path) => Some(
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)?,
            ),
            None => None,
        };
        Ok(self)
    }

    pub fn run(&mut self) -> &mut Self {
        let verbosity = match self.log_level {
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        };
        Builder::new()
            .filter(None, verbosity)
            .target(Target::Stdout)
            .init();

        self
    }

    fn log_to_file(&mut self, level: LogLevel, message: &impl std::fmt::Display) {
        if let Some(ref mut file) = self.log_file {
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
                    self.stack,
                    message
                )
                .as_bytes(),
            )
            .unwrap();
        }
    }

    pub fn debug(&mut self, message: impl std::fmt::Display) {
        if self.log_level == LogLevel::Debug {
            self.log_to_file(LogLevel::Debug, &message);
        }
        log::debug!(target: self.stack, "{}", message);
    }
    pub fn info(&mut self, message: impl std::fmt::Display) {
        if self.log_level <= LogLevel::Info {
            self.log_to_file(LogLevel::Info, &message);
        }
        log::info!(target: self.stack, "{}", message);
    }
    pub fn warn(&mut self, message: impl std::fmt::Display) {
        if self.log_level <= LogLevel::Warn {
            self.log_to_file(LogLevel::Warn, &message);
        }
        log::warn!(target: self.stack, "{}", message);
    }
    pub fn error(&mut self, message: impl std::fmt::Display) {
        if self.log_level <= LogLevel::Error {
            self.log_to_file(LogLevel::Error, &message);
        }
        log::error!(target: self.stack, "{}", message);
    }
}
