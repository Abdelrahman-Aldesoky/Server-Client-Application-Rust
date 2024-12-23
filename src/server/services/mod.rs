//! This module organizes all the gRPC service implementations for our server.
//! A mod.rs file in Rust is commonly used as the entry point for a module,
//! declaring its submodules and re-exporting items we want to make public.

// Declare submodules containing our service implementations
mod calculator;
mod echo;

// Re-export the service structs so they can be used by other modules
// The pub(crate) means these are only visible within our crate
pub(crate) use calculator::CalculatorServer;
pub(crate) use echo::EchoServer;
