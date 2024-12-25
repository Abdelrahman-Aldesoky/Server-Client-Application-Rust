use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use super::types::Component;
use std::sync::{Once, Mutex};

static INIT_LOGGER: Once = Once::new();
static LOGGER_MUTEX: Mutex<()> = Mutex::new(());

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