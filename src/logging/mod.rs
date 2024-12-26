//! Logging Module
//! This module provides logging initialization functions for different components.

mod setup;
mod types;

pub use types::Component;
use setup::init_logging;

/// Initialize logging for the specified component
/// 
/// # Arguments
/// * `component` - The component for which to initialize logging.
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure.
#[inline]
pub fn init(component: Component) -> Result<(), Box<dyn std::error::Error>> {
    init_logging(component)
}

pub mod prelude {
    use super::*;

    /// Initialize logging for the server component
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure.
    #[inline]
    pub fn init_server() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Server)
    }
    
    /// Initialize logging for the client component
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure.
    #[inline]
    pub fn init_client() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Client)
    }
    
    /// Initialize logging for the test component
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure.
    #[inline]
    pub fn init_test() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Test)
    }
}

pub use prelude::*;