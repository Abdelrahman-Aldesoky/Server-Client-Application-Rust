//! Service Client Implementations
//! This module organizes the client-side service implementations:
//! - calculator: Calculator service client
//! - echo: Echo service client
//!
//! We re-export the main types and the Operation enum for easier access

mod calculator;
mod echo;

// Re-export service clients and common types
pub use calculator::CalculatorService;
pub use echo::EchoService;
// Re-export Operation enum for calculator service
pub use crate::proto::calculator::Operation;
