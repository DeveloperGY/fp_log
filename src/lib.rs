mod log;
pub mod logger_core;

use std::error;
use std::fmt;
use std::path;

pub enum FPLoggerError {
    CreationFailure(Option<Box<dyn error::Error>>),
    LogWriteFailure(Option<Box<dyn error::Error>>),
}

impl FPLoggerError {
    pub fn to_string(&self) -> String {
        match self {
            Self::CreationFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to create fplogger ({})", reason)
            }
            Self::LogWriteFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to log message ({})", reason)
            }
        }
    }
}

impl fmt::Debug for FPLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for FPLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl error::Error for FPLoggerError {}

pub struct FPLogger {
    core: logger_core::LoggerCore,
}

impl FPLogger {
    pub fn new(filepath: impl AsRef<path::Path>) -> Result<Self, FPLoggerError> {
        let core = logger_core::LoggerCore::new(filepath)
            .map_err(|err| FPLoggerError::CreationFailure(Some(Box::new(err))))?;

        Ok(Self { core })
    }

    pub fn info(&self, msg: impl AsRef<str>) -> Result<(), FPLoggerError> {
        self.core
            .log(format!("[INFO] {}\n", msg.as_ref()))
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }

    pub fn info_many(&self, msgs: &[impl AsRef<str>]) -> Result<(), FPLoggerError> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!("[INFO] {}\n", msg.as_ref()))
            .collect();

        self.core
            .log_many(&msgs)
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }

    pub fn err(&self, msg: impl AsRef<str>) -> Result<(), FPLoggerError> {
        self.core
            .log(format!(" [ERR] {}\n", msg.as_ref()))
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }

    pub fn err_many(&self, msgs: &[impl AsRef<str>]) -> Result<(), FPLoggerError> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!(" [ERR] {}\n", msg.as_ref()))
            .collect();

        self.core
            .log_many(&msgs)
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }
}
