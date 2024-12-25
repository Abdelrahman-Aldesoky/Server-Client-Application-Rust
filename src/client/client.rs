//! Core gRPC Client Implementation
//! This file implements the client-side Builder pattern and channel management.
//! Key concepts demonstrated:
//! 1. Builder pattern for configuration
//! 2. Channel management for gRPC connections
//! 3. Error handling with Status
//! 4. Clean API design with impl AsRef<str>

use tonic::{transport::{Channel, Endpoint}, Status};
use tracing::{info};

// Builder struct for configuring the client
// Clone allows us to create copies of the builder
#[derive(Clone)]
pub struct GrpcClientBuilder {
    endpoint: Endpoint,  // Configured but not yet connected endpoint
}

// Main client struct that holds the active channel
#[derive(Clone)]
pub struct GrpcClient {
    channel: Channel,  // Active gRPC channel
}

// Builder implementation with fluent API
impl GrpcClientBuilder {
    // Create a new builder from an address string
    // AsRef<str> allows flexible string types (String, &str, etc.)
    pub fn new(addr: impl AsRef<str>) -> Result<Self, Status> {
        let endpoint = Endpoint::from_shared(addr.as_ref().to_string())
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Self { endpoint })
    }

    // Connect and build the final client
    // Uses lazy connection - only connects when first used
    pub fn connect(self) -> Result<GrpcClient, Status> {
        // Initialize logging for client
        crate::logging::init_client()
            .map_err(|e| Status::internal(format!("Failed to initialize logging: {}", e)))?;
        
        info!("Connecting to gRPC server at {}", self.endpoint.uri());
        let channel = self.endpoint.connect_lazy();
        info!("Successfully connected to gRPC server at {}", self.endpoint.uri());
        Ok(GrpcClient { channel })
    }
}

// Main client implementation
impl GrpcClient {
    // Entry point for client configuration
    pub fn builder(addr: impl AsRef<str>) -> Result<GrpcClientBuilder, Status> {
        GrpcClientBuilder::new(addr)
    }

    // Internal method to share the channel with service implementations
    pub(crate) fn get_channel(&self) -> Channel {
        self.channel.clone()
    }
}
