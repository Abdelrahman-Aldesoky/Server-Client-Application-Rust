//! Server Module Organization
//! This is the root module for the gRPC server implementation.
//! It demonstrates Rust's module system and clean API design:
//! 
//! Key components:
//! - server: Contains the main GrpcServer implementation with Builder pattern
//! - services: Contains individual service implementations (Calculator, Echo)
//!
//! The pub use statement provides a clean public API by re-exporting
//! the GrpcServer type at the module level, following the facade pattern.

// Internal modules that make up our server implementation
mod server;
mod services;

// Re-export the main server type for cleaner external usage
// This allows users to just use `use crate::server::GrpcServer`
// instead of `use crate::server::server::GrpcServer`
pub use server::GrpcServer;