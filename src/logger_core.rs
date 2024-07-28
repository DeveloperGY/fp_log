/// The core functionality of a logger, can be used to create custom loggers outside of fp_log
pub struct LoggerCore;

impl LoggerCore {
    /// Creates a new [`LoggerCore`]
    pub fn new() -> Self {
        Self
    }

    /// Logs a single message to the [`log::Log`] referenced by the [`LoggerCore`]
    pub fn log(
        &self,
        dest: &mut impl std::io::Write,
        msg: impl AsRef<[u8]>,
    ) -> std::io::Result<()> {
        dest.write_all(msg.as_ref())?;
        dest.flush()
    }

    /// Logs multiple messages to the [`log::Log`] referenced by the [`LoggerCore`], ensuring they
    /// are grouped together in the log destination
    pub fn log_many(
        &self,
        dest: &mut impl std::io::Write,
        msgs: &[impl AsRef<[u8]>],
    ) -> std::io::Result<()> {
        let bytes: Vec<u8> = msgs
            .iter()
            .map(|e| e.as_ref())
            .flatten()
            .map(|e| *e)
            .collect();

        self.log(dest, bytes)
    }
}
