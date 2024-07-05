use std::error;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path;

/// The error type for a [`Log`]
pub enum LogError {
    /// An error during the creation of a [`Log`]
    LogFileCreationError(Option<Box<dyn error::Error>>),

    /// An error while loggin to a [`Log`]
    LogFileWriteError(Option<Box<dyn error::Error>>),
}

impl fmt::Debug for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            Self::LogFileCreationError(reason) => {
                let reason = match reason {
                    Some(err) => err.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to create log file\n\t- {}", reason)
            }
            Self::LogFileWriteError(reason) => {
                let reason = match reason {
                    Some(err) => err.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to write to log file\n\t- {}", reason)
            }
        };

        write!(f, "{}", string_representation)
    }
}

impl error::Error for LogError {}

/// A struct representing a single log file
pub struct Log {
    file: fs::File,
}

impl Log {
    /// Creates a new [`Log`] from a specified file path
    pub fn new(filepath: impl AsRef<path::Path>) -> Result<Self, LogError> {
        let filepath = filepath.as_ref().to_path_buf();

        let file = fs::File::create(&filepath)
            .map_err(|err| LogError::LogFileCreationError(Some(Box::new(err))))?;

        Ok(Self { file })
    }

    /// The core logic of logging to a file
    fn basic_log(&mut self, msg: impl AsRef<str>, should_flush: bool) -> Result<(), LogError> {
        self.file
            .write_all(msg.as_ref().as_bytes())
            .map_err(|err| LogError::LogFileWriteError(Some(Box::new(err))))?;

        if should_flush {
            self.file
                .flush()
                .map_err(|err| LogError::LogFileWriteError(Some(Box::new(err))))?;
        }

        Ok(())
    }

    /// Logs a single message to the log file
    pub fn log(&mut self, msg: impl AsRef<str>) -> Result<(), LogError> {
        self.basic_log(msg, true)
    }

    /// Logs multiple messages to the log file
    pub fn log_many(&mut self, msgs: &[impl AsRef<str>]) -> Result<(), LogError> {
        for i in 0..msgs.len() {
            let is_last_message = i == msgs.len() - 1;

            self.basic_log(msgs[i].as_ref(), is_last_message)?;
        }

        Ok(())
    }
}
