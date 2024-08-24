use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Log {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl From<LevelFilter> for Log {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::OFF => Log::Off,
            LevelFilter::ERROR => Log::Error,
            LevelFilter::WARN => Log::Warn,
            LevelFilter::INFO => Log::Info,
            LevelFilter::DEBUG => Log::Debug,
            LevelFilter::TRACE => Log::Trace,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn from_level_filter() {
        let from_off: Log = Log::from(LevelFilter::OFF);
        let from_error: Log = Log::from(LevelFilter::ERROR);
        let from_warn: Log = Log::from(LevelFilter::WARN);
        let from_info: Log = Log::from(LevelFilter::INFO);
        let from_debug: Log = Log::from(LevelFilter::DEBUG);
        let from_trace: Log = Log::from(LevelFilter::TRACE);

        assert_eq!(from_off, Log::Off);
        assert_eq!(from_error, Log::Error);
        assert_eq!(from_warn, Log::Warn);
        assert_eq!(from_info, Log::Info);
        assert_eq!(from_debug, Log::Debug);
        assert_eq!(from_trace, Log::Trace);
    }

    #[tokio::test]
    async fn into_level_filter() {
        let into_off: Log = LevelFilter::OFF.into();
        let into_error: Log = LevelFilter::ERROR.into();
        let into_warn: Log = LevelFilter::WARN.into();
        let into_info: Log = LevelFilter::INFO.into();
        let into_debug: Log = LevelFilter::DEBUG.into();
        let into_trace: Log = LevelFilter::TRACE.into();

        assert_eq!(into_off, Log::Off);
        assert_eq!(into_error, Log::Error);
        assert_eq!(into_warn, Log::Warn);
        assert_eq!(into_info, Log::Info);
        assert_eq!(into_debug, Log::Debug);
        assert_eq!(into_trace, Log::Trace);
    }
}
