//! gRPC Server Binary
//! This is the main executable for running the gRPC server.
//! It demonstrates:
//! 1. Simple server setup using builder pattern
//! 2. Error handling with Result
//! 3. Async runtime configuration with tokio

// Import our server type from the main library
use embedded_recruitment_task::GrpcServer;

// Configure async runtime and provide error handling
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize server using builder pattern
    // _shutdown is a channel sender we could use to gracefully shutdown the server
    let (server, _shutdown) = GrpcServer::builder()
        .address("[::1]:12345")  // IPv6 loopback address and port
        .build()?;
        
    // Log server startup information
    println!("Server listening on [::1]:12345");
    
    // Start the server and await completion or error
    server.serve().await?;
    Ok(())
}