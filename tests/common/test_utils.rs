//! Test Utilities Implementation
//! This module provides the core testing infrastructure that enables:
//! 1. Parallel test execution through dynamic port allocation
//! 2. Isolated test environments for each test
//! 3. Automatic resource cleanup
//! 4. Simplified test setup and teardown
//! 5. Connection management

use std::sync::atomic::{AtomicU16, Ordering};
use tokio::{sync::oneshot, time::{sleep, Duration}};
use tonic::Status;
use embedded_recruitment_task::{GrpcClient, GrpcServer};

// Global atomic counter for port allocation
// - Starts at 50000 to avoid system-reserved ports
// - Atomic operations ensure thread-safe incrementation
// - Each test gets a unique port to avoid conflicts
static NEXT_PORT: AtomicU16 = AtomicU16::new(50000);

// TestContext: Main test harness that provides isolated test environments
// - Manages server lifecycle
// - Handles client connections
// - Ensures proper cleanup
pub struct TestContext {
    // Optional shutdown sender allows for graceful server shutdown
    // None after shutdown is triggered (taken)
    shutdown: Option<oneshot::Sender<()>>,
    // Client instance shared across test operations
    // Clone trait allows for multiple references
    pub client: GrpcClient,
}

impl TestContext {
    // Creates a complete test environment with running server and connected client
    // Returns Result to propagate setup failures to test
    pub async fn setup() -> Result<Self, Status> {
        // Atomically get and increment port number
        // SeqCst ordering ensures sequential consistency across threads
        let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);
        let addr = format!("[::1]:{}", port);

        // Build and configure server instance
        let (server, shutdown) = GrpcServer::builder()
            .address(addr.clone())
            .build()?;

        // Spawn server in separate task to not block test execution
        // Server runs until shutdown signal is received
        tokio::spawn(async move {
            if let Err(e) = server.serve().await {
                eprintln!("Test server error: {}", e);
            }
        });

        // Brief delay to ensure server is ready
        // Prevents race conditions with immediate client connections
        sleep(Duration::from_millis(100)).await;

        // Create and connect client to server
        let client = GrpcClient::builder(format!("http://{}", addr))?
            .connect()?;

        Ok(Self { 
            shutdown: Some(shutdown),
            client 
        })
    }
}

// Drop implementation ensures cleanup happens even if test panics
// This prevents resource leaks and hanging servers
impl Drop for TestContext {
    fn drop(&mut self) {
        // Take ownership of shutdown sender and trigger server shutdown
        // take() ensures shutdown happens only once
        if let Some(shutdown) = self.shutdown.take() {
            shutdown.send(()).ok(); // Ignore send errors during cleanup
        }
    }
}
