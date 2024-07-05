use std::error;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path;

pub enum LogError {
    LogFileCreationError(Option<Box<dyn error::Error>>),
    LogFileWriteError(Option<Box<dyn error::Error>>),
}

impl LogError {
    pub fn to_string(&self) -> String {
        match self {
            Self::LogFileCreationError(reason) => {
                let reason = match reason {
                    Some(err) => err.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to create log file ({})", reason)
            }
            Self::LogFileWriteError(reason) => {
                let reason = match reason {
                    Some(err) => err.to_string(),
                    None => "unspecified reason".to_string(),
                };

                format!("failed to write to log file ({})", reason)
            }
        }
    }
}

impl fmt::Debug for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl error::Error for LogError {}

pub struct Log {
    file: fs::File,
}

impl Log {
    pub fn new(filepath: impl AsRef<path::Path>) -> Result<Self, LogError> {
        let filepath = filepath.as_ref().to_path_buf();

        let file = fs::File::create(&filepath)
            .map_err(|err| LogError::LogFileCreationError(Some(Box::new(err))))?;

        Ok(Self { file })
    }

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

    pub fn log(&mut self, msg: impl AsRef<str>) -> Result<(), LogError> {
        self.basic_log(msg, true)
    }

    pub fn log_many(&mut self, msgs: &[impl AsRef<str>]) -> Result<(), LogError> {
        for i in 0..msgs.len() {
            let is_last_message = i == msgs.len() - 1;

            self.basic_log(msgs[i].as_ref(), is_last_message)?;
        }

        Ok(())
    }
}
