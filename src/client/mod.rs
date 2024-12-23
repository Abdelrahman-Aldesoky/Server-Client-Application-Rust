//! Client Module Organization
//! This module provides a clean API for the gRPC client implementation:
//! - client: Contains the core GrpcClient implementation
//! - services: Contains specific service clients (Calculator, Echo)
//!
//! The pub use statements make the main types directly available to users
//! of our library, following the facade pattern for a cleaner API.

// Declare our submodules
mod client;
mod services;

// Re-export main types for easier access
// Users can now use them directly from the crate root
pub use client::GrpcClient;
pub use services::*;  // All public items from services module