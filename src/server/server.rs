//! Main gRPC Server Implementation
//! This file demonstrates several advanced Rust patterns:
//! 1. Builder Pattern: For flexible server configuration
//! 2. Shutdown handling using tokio channels
//! 3. Error handling with Status
//! 4. Service registration and lifecycle management

// Import required dependencies
// tonic: The gRPC framework we're using
// tokio: For async runtime and utilities
use tonic::{transport::Server, Status, Code, Request};
use tokio::sync::oneshot;  // Channel for shutdown signal
use tracing::{info, error};  // Import tracing for logging
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

// Define an interceptor function to log incoming connections
fn log_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    info!("Incoming connection from: {:?}", req.remote_addr());
    Ok(req)
}

// Main server implementation
impl GrpcServer {
    // Create a new builder (entry point for server configuration)
    pub fn builder() -> GrpcServerBuilder {
        GrpcServerBuilder::new()
    }

    // Start the server and run until shutdown signal
    pub async fn serve(self) -> Result<(), Status> {
        // Initialize logging for server
        crate::logging::init_server()
            .map_err(|e| Status::internal(format!("Failed to initialize logging: {}", e)))?;
        
        // Parse the address string into a socket address
        let addr = self.addr.parse()
            .map_err(|e| {
                error!("Invalid server address: {}", e);
                Status::new(Code::InvalidArgument, "invalid server address format")
            })?;

        info!("Starting gRPC server on {}", addr);

        // Create intercepted services
        let echo_service = EchoServiceServer::with_interceptor(EchoServer::default(), log_interceptor);
        let calculator_service = CalculatorServiceServer::with_interceptor(CalculatorServer::default(), log_interceptor);

        // Configure and start the server with logging interceptor
        Server::builder()
            // Register our services
            .add_service(echo_service)
            .add_service(calculator_service)
            // Start serving with shutdown handler
            .serve_with_shutdown(addr, async { 
                self.shutdown.await.ok(); 
                info!("Received shutdown signal, stopping gRPC server");
            })
            .await
            .map_err(|e| {
                error!("Server error: {}", e);
                Status::new(Code::Internal, format!("server error: {}", e))
            })
    }
}
