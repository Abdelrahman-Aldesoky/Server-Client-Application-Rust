//! Protocol Buffer Module
//! This module contains the generated Rust code from our .proto files.
//! 
//! Key aspects:
//! 1. Generated code using tonic::include_proto! macro
//! 2. Separate modules for each service to maintain clean organization
//! 3. Automatic code generation from .proto definitions

// Include generated code for echo service
// tonic::include_proto! macro processes the proto file at compile time
// and generates all necessary Rust types, traits, and implementations
pub mod echo {
    tonic::include_proto!("echo");  // Generates from echo.proto
}

// Include generated code for calculator service
// The generated code includes:
// - Request/Response message structs
// - Service traits for client and server
// - Helper types and conversions
pub mod calculator {
    tonic::include_proto!("calculator");  // Generates from calculator.proto
}
