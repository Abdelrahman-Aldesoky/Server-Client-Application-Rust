//! Logging Setup
//! This file provides the setup functions for initializing logging.

use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use super::types::Component;
use std::sync::{Once, Mutex};

// Ensure logging is initialized only once
static INIT_LOGGER: Once = Once::new();
static LOGGER_MUTEX: Mutex<()> = Mutex::new(());

/// Initialize logging for the specified component
/// 
/// # Arguments
/// * `component` - The component for which to initialize logging.
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure.
pub(crate) fn init_logging(component: Component) -> Result<(), Box<dyn std::error::Error>> {
    INIT_LOGGER.call_once(|| {
        let _lock = LOGGER_MUTEX.lock().unwrap();
        let (name, level) = component.config();
        
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::NEVER)
            .filename_prefix(name)
            .build("logs").expect("Failed to create file appender");

        fmt::Subscriber::builder()
            .with_ansi(false)
            .with_target(false)
            .with_writer(file_appender)
            .with_env_filter(EnvFilter::from_default_env().add_directive(level.into()))
            .try_init()
            .expect("Failed to initialize logger");

        tracing::info!("Initialized logging for {:?}", component);
    });

    Ok(())
}