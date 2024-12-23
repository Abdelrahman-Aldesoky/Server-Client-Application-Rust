//! Root Library Module
//! This is the entry point for the library crate.
//! It provides:
//! 1. Module organization
//! 2. Public API exports
//! 3. Main types accessibility

// Module declarations
pub mod proto;     // Generated Protocol Buffer code
pub mod client;    // Client-side implementation
pub mod server;    // Server-side implementation

// Re-export main types for easier access
// This allows users to access these types directly from the crate root
// Example: use crate_name::GrpcServer instead of crate_name::server::GrpcServer
pub use server::GrpcServer;    // Main server type with builder pattern
pub use client::GrpcClient;    // Main client type with builder pattern