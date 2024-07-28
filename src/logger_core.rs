/// The core functionality of a logger, can be used to create custom loggers outside of fp_log
pub struct LoggerCore;

impl LoggerCore {
    /// Creates a new [`LoggerCore`]
    pub fn new() -> Self {
        Self
    }

    /// Logs a single message to the log destination
    pub fn log(
        &self,
        destination: &mut impl std::io::Write,
        log_message: impl AsRef<[u8]>,
    ) -> std::io::Result<()> {
        destination.write_all(log_message.as_ref())?;
        destination.flush()
    }

    /// Logs multiple messages to the log destination, ensuring they
    /// are grouped together in the log destination
    pub fn log_many(
        &self,
        destination: &mut impl std::io::Write,
        log_messages: &[impl AsRef<[u8]>],
    ) -> std::io::Result<()> {
        let concatenated_messages: Vec<u8> = log_messages
            .iter()
            .map(|e| e.as_ref())
            .flatten()
            .map(|e| *e)
            .collect();

        self.log(destination, concatenated_messages)
    }
}
