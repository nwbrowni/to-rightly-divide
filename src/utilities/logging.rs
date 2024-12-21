use core::str;
use std::{io::Write, fs::OpenOptions};

pub const GENERAL: Logger = Logger { file: "information.log", level: LoggerLevel::Informational };
pub const ERROR: Logger = Logger { file: "error.log", level: LoggerLevel::Informational };

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum LoggerLevel {
    Debug,
    Informational,
    Warning,
    Error,
}

pub struct Logger<'a> {
    file: &'a str,
    level: LoggerLevel,
}

impl<'a> Logger<'a> {
    pub fn file(&self) -> String {
        self.file.to_string()
    }

    pub fn level(&self) -> LoggerLevel {
        self.level
    }

    fn log<'b>(&self, message: &'b str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.file)
            .unwrap();
        match file.write(message.as_bytes()) {
            Ok(_) => {
                match file.write("\n".as_bytes()) {
                    Ok(_) => {
                        return
                    },
                    Err(_) => return,
                }
            },
            Err(_) => return,
        }
    }

    pub fn debug<'b>(&self, message: &'b str) {
        if self.level <= LoggerLevel::Debug { self.log(("DEBUG|".to_string() + message).as_str()) }
    }

    pub fn information<'b>(&self, message: &'b str) {
        if self.level <= LoggerLevel::Informational { self.log(("INFO|".to_string() + message).as_str()) }
    }

    pub fn warning<'b>(&self, message: &'b str) {
        if self.level <= LoggerLevel::Warning { self.log(("WARNING|".to_string() + message).as_str()) }
    }

    pub fn error<'b>(&self, message: &'b str) {
        if self.level <= LoggerLevel::Error { self.log(("ERROR|".to_string() + message).as_str()) }
    }
}