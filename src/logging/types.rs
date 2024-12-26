//! Logging Types
//! This file defines the logging components and their configurations.

use tracing_subscriber::filter::LevelFilter;

/// Enum representing different components of the application
#[derive(Debug, Clone, Copy)]
pub enum Component {
    Server,
    Client,
    Test,
}

impl Component {
    /// Get the logging configuration for the component
    /// 
    /// # Returns
    /// * `(&'static str, LevelFilter)` - A tuple containing the component name and log level filter.
    pub(crate) fn config(&self) -> (&'static str, LevelFilter) {
        match self {
            Component::Server => ("server", LevelFilter::INFO),
            Component::Client => ("client", LevelFilter::INFO),
            Component::Test  => ("test",   LevelFilter::TRACE),
        }
    }
}