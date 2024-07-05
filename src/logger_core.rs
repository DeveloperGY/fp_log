use crate::log;

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::path;
use std::sync::{Mutex, OnceLock, RwLock};

/// The error type for a [`LoggerCore`]
pub enum LoggerCoreError {
    /// An error during the creation of a [`LoggerCore`]
    LogInitializationFailure(Option<Box<dyn error::Error>>),

    /// An error when writing to a log via a [`LoggerCore`]
    LogWriteFailure(Option<Box<dyn error::Error>>),
}

impl fmt::Debug for LoggerCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for LoggerCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            Self::LogInitializationFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to initialize log\n\t- {}", reason)
            }
            Self::LogWriteFailure(reason) => {
                let reason = match reason {
                    Some(reason) => reason.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to write to log\n\t- {}", reason)
            }
        };

        write!(f, "{}", string_representation)
    }
}

impl error::Error for LoggerCoreError {}

// NOTE: This could probably be moved to the LoggerCore constructor
/// The global cache of [`log::Log`]s, built for use in a multithreaded context
static LOG_FILES: OnceLock<RwLock<HashMap<path::PathBuf, Mutex<log::Log>>>> = OnceLock::new();

/// The core functionality of a logger, can be used to create custom loggers outside of fp_log
pub struct LoggerCore {
    filepath: path::PathBuf,
}

impl LoggerCore {
    /// Creates a new [`LoggerCore`]
    pub fn new(filepath: impl AsRef<path::Path>) -> Result<Self, LoggerCoreError> {
        let filepath = filepath.as_ref().to_path_buf();

        // Initialize the global log files hashmap if it isnt already
        let log_files = LOG_FILES.get_or_init(|| RwLock::new(HashMap::new()));

        // Check to see if the log already exists
        let log_exists = log_files
            .read()
            .map_err(|err| LoggerCoreError::LogInitializationFailure(Some(Box::new(err))))?
            .contains_key(&filepath);

        // If the log doesnt already exist, create it
        if !log_exists {
            let log = log::Log::new(&filepath)
                .map_err(|err| LoggerCoreError::LogInitializationFailure(Some(Box::new(err))))?;

            log_files
                .write()
                .map_err(|err| LoggerCoreError::LogInitializationFailure(Some(Box::new(err))))?
                .insert(filepath.clone(), Mutex::new(log));
        }

        Ok(Self { filepath })
    }

    /// Logs a single message to the [`log::Log`] referenced by the [`LoggerCore`]
    pub fn log(&self, msg: impl AsRef<str>) -> Result<(), LoggerCoreError> {
        LOG_FILES
            .get()
            .ok_or_else(|| {
                LoggerCoreError::LogWriteFailure(Some(
                    "global log container not initialized".into(),
                ))
            })?
            .read()
            .map_err(|_| {
                LoggerCoreError::LogWriteFailure(Some("global log container was poisoned".into()))
            })?
            .get(&self.filepath)
            .ok_or_else(|| LoggerCoreError::LogWriteFailure(Some("log doesnt exist".into())))?
            .lock()
            .map_err(|_| LoggerCoreError::LogWriteFailure(Some("log was poisoned".into())))?
            .log(msg)
            .map_err(|err| LoggerCoreError::LogWriteFailure(Some(Box::new(err))))
    }

    /// Logs multiple messages to the [`log::Log`] referenced by the [`LoggerCore`], ensuring they
    /// are grouped together in the log destination
    pub fn log_many(&self, msgs: &[impl AsRef<str>]) -> Result<(), LoggerCoreError> {
        LOG_FILES
            .get()
            .ok_or_else(|| {
                LoggerCoreError::LogWriteFailure(Some(
                    "global log container not initialized".into(),
                ))
            })?
            .read()
            .map_err(|_| {
                LoggerCoreError::LogWriteFailure(Some("global log container was poisoned".into()))
            })?
            .get(&self.filepath)
            .ok_or_else(|| LoggerCoreError::LogWriteFailure(Some("log doesnt exist".into())))?
            .lock()
            .map_err(|_| LoggerCoreError::LogWriteFailure(Some("log was poisoned".into())))?
            .log_many(msgs)
            .map_err(|err| LoggerCoreError::LogWriteFailure(Some(Box::new(err))))
    }
}
