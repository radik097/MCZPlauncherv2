use std::path::PathBuf;
use tracing_subscriber::fmt::time::SystemTime;
use tracing_subscriber::prelude::*;

/// Initialize file-based logging for the application.
/// Creates a logs directory and writes logs to a timestamped file.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create logs directory if it doesn't exist
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("MCZPlauncher")
        .join("logs");
    
    std::fs::create_dir_all(&log_dir)?;

    // Create a file appender with daily rotation
    let file_appender = tracing_appender::rolling::daily(&log_dir, "mczlauncher.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Set up the subscriber with both console and file output
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(SystemTime::default())
                .with_target(true)
                .with_level(true)
                .with_file(true)
                .with_line_number(true)
                .with_writer(non_blocking),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(SystemTime::default())
                .with_target(false)
                .with_level(true)
                .compact(),
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("MCZ Launcher initialized - logs directory: {:?}", log_dir);
    Ok(())
}
