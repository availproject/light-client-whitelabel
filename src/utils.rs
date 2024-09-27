use tracing::{Level, Subscriber};
use tracing_subscriber::{fmt::format, EnvFilter, FmtSubscriber};

pub fn json_subscriber(log_level: Level) -> impl Subscriber + Send + Sync {
    FmtSubscriber::builder()
        .json()
        .with_env_filter(EnvFilter::new(format!(
            "light={log_level},avail={log_level}"
        )))
        .with_span_events(format::FmtSpan::CLOSE)
        .with_target(false)
        .finish()
}

pub fn default_subscriber(log_level: Level) -> impl Subscriber + Send + Sync {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(format!(
            "light={log_level},avail={log_level}"
        )))
        .with_span_events(format::FmtSpan::CLOSE)
        .with_target(false)
        .finish()
}
