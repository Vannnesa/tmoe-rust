use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .compact(),
        )
        .init();
}
