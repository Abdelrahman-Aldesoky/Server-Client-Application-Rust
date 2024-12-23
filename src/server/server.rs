//! Main gRPC Server Implementation
//! This file demonstrates several advanced Rust patterns:
//! 1. Builder Pattern: For flexible server configuration
//! 2. Shutdown handling using tokio channels
//! 3. Error handling with Status
//! 4. Service registration and lifecycle management

// Import required dependencies
// tonic: The gRPC framework we're using
// tokio: For async runtime and utilities
use tonic::{transport::Server, Status, Code};
use tokio::sync::oneshot;  // Channel for shutdown signal
// Import our service implementations
use crate::proto::echo::echo_service_server::EchoServiceServer;
use crate::proto::calculator::calculator_service_server::CalculatorServiceServer;
use super::services::{EchoServer, CalculatorServer};

// Builder pattern implementation
// This allows flexible configuration of server parameters
#[derive(Default)]
pub struct GrpcServerBuilder {
    addr: Option<String>,  // Server address is optional during building
}

// The actual server struct that will be built
pub struct GrpcServer {
    addr: String,  // Server address (required for running)
    shutdown: oneshot::Receiver<()>,  // Channel for graceful shutdown
}

// Builder implementation
impl GrpcServerBuilder {
    // Create a new builder instance
    pub fn new() -> Self {
        Self::default()
    }

    // Set the server address
    // Uses generic Into<String> to accept different string types
    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.addr = Some(addr.into());
        self
    }

    // Finalize the server configuration
    // Returns both the server and a shutdown signal sender
    pub fn build(self) -> Result<(GrpcServer, oneshot::Sender<()>), Status> {
        // Ensure address was provided
        let addr = self.addr.ok_or_else(|| Status::new(
            Code::InvalidArgument,
            "Server address must be provided"
        ))?;

        // Create shutdown channel
        let (tx, rx) = oneshot::channel();
        
        Ok((GrpcServer {
            addr,
            shutdown: rx,
        }, tx))
    }
}

// Main server implementation
impl GrpcServer {
    // Create a new builder (entry point for server configuration)
    pub fn builder() -> GrpcServerBuilder {
        GrpcServerBuilder::new()
    }

    // Start the server and run until shutdown signal
    pub async fn serve(self) -> Result<(), Status> {
        // Parse the address string into a socket address
        let addr = self.addr.parse()
            .map_err(|_| Status::new(
                Code::InvalidArgument,
                "invalid server address format"
            ))?;

        // Configure and start the server
        Server::builder()
            // Register our services
            .add_service(EchoServiceServer::new(EchoServer::default()))
            .add_service(CalculatorServiceServer::new(CalculatorServer::default()))
            // Start serving with shutdown handler
            .serve_with_shutdown(addr, async { 
                self.shutdown.await.ok(); 
            })
            .await
            .map_err(|e| Status::new(
                Code::Internal,
                format!("server error: {}", e)
            ))
    }
}
