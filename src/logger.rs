use crate::logger_core::LoggerCore;

/// A basic logger provided by fp_log
pub struct FPLogger {
    core: LoggerCore,
}

impl FPLogger {
    /// Creates a new [`FPLogger`]
    pub fn new() -> Self {
        Self {
            core: LoggerCore::new(),
        }
    }

    /// Logs a log message to the log destination
    pub fn log(&self, dest: &mut impl std::io::Write, msg: impl AsRef<str>) -> std::io::Result<()> {
        self.core.log(dest, format!("[LOG] {}\n", msg.as_ref()))
    }

    /// Logs many log messages to the log destination
    pub fn log_many(
        &self,
        dest: &mut impl std::io::Write,
        msgs: &[impl AsRef<str>],
    ) -> std::io::Result<()> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!("[LOG] {}\n", msg.as_ref()))
            .collect();

        self.core.log_many(dest, &msgs)
    }

    /// Logs an info message to the log destination
    pub fn info(
        &self,
        dest: &mut impl std::io::Write,
        msg: impl AsRef<str>,
    ) -> std::io::Result<()> {
        self.core.log(dest, format!("[INFO] {}\n", msg.as_ref()))
    }

    /// Logs many info messages to the log destination
    pub fn info_many(
        &self,
        dest: &mut impl std::io::Write,
        msgs: &[impl AsRef<str>],
    ) -> std::io::Result<()> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!("[INFO] {}\n", msg.as_ref()))
            .collect();

        self.core.log_many(dest, &msgs)
    }

    /// Logs an error message to the log destination
    pub fn err(&self, dest: &mut impl std::io::Write, msg: impl AsRef<str>) -> std::io::Result<()> {
        self.core.log(dest, format!(" [ERR] {}\n", msg.as_ref()))
    }

    /// Logs multiple error messages to the log destination
    pub fn err_many(
        &self,
        dest: &mut impl std::io::Write,
        msgs: &[impl AsRef<str>],
    ) -> std::io::Result<()> {
        let msgs: Vec<_> = msgs
            .iter()
            .map(|msg| format!(" [ERR] {}\n", msg.as_ref()))
            .collect();

        self.core.log_many(dest, &msgs)
    }
}
