mod log;
pub mod logger_core;

use std::error;
use std::fmt;
use std::path;

/// Error type for an [`FPLogger`]
pub enum FPLoggerError {
    /// Error during the creation of an [`FPLogger`]
    CreationFailure(Option<Box<dyn error::Error>>),

    /// Error while writing to a log
    LogWriteFailure(Option<Box<dyn error::Error>>),
}

impl fmt::Debug for FPLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for FPLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            Self::CreationFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to create fplogger\n\t- {}", reason)
            }
            Self::LogWriteFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to log message\n\t- {}", reason)
            }
        };

        write!(f, "{}", string_representation)
    }
}

impl error::Error for FPLoggerError {}

/// A basic logger provided by fp_log
pub struct FPLogger {
    core: logger_core::LoggerCore,
}

impl FPLogger {
    /// Creates a new [`FPLogger`]
    pub fn new(filepath: impl AsRef<path::Path>) -> Result<Self, FPLoggerError> {
        let core = logger_core::LoggerCore::new(filepath)
            .map_err(|err| FPLoggerError::CreationFailure(Some(Box::new(err))))?;

        Ok(Self { core })
    }

    /// Logs an info message to the log destination referred to by the [`FPLogger`]
    pub fn info(&self, msg: impl AsRef<str>) -> Result<(), FPLoggerError> {
        self.core
            .log(format!("[INFO] {}\n", msg.as_ref()))
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }

    /// Logs many info messages to the log destination referred to by the [`FPLogger`]
    pub fn info_many(&self, msgs: &[impl AsRef<str>]) -> Result<(), FPLoggerError> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!("[INFO] {}\n", msg.as_ref()))
            .collect();

        self.core
            .log_many(&msgs)
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }
    /// Logs an error message to the log destination referred to by the [`FPLogger`]
    pub fn err(&self, msg: impl AsRef<str>) -> Result<(), FPLoggerError> {
        self.core
            .log(format!(" [ERR] {}\n", msg.as_ref()))
            .map_err(|err| FPLoggerError::LogWriteFailure(Some(Box::new(err))))
    }
    /// Logs multiple error messages to the log destination referred to by the [`FPLogger`]
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
