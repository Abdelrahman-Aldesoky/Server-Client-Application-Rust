//! Message Integrity Testing Suite
//! This file verifies message handling integrity under concurrent load:
//! 1. Tests ordering and content preservation with shared connections
//! 2. Tests message integrity across separate connections
//! 3. Validates concurrent message handling
//! 4. Ensures no message corruption under load

// Import utilities for async operations and synchronization
use tokio::time::{timeout, Duration};
use tokio::sync::Mutex;  // Async mutex for thread-safe state
use std::sync::Arc;      // Reference counting for shared ownership
use common::TestContext;

mod common;

// Test configuration
const TOTAL_MESSAGES: usize = 1000;  // Number of messages to send concurrently
const TIMEOUT_DURATION: Duration = Duration::from_secs(5);  // Maximum time per operation

// Test connection pooling behavior
// Verifies that multiple concurrent requests using the same connection:
// - Maintain message integrity
// - Don't interfere with each other
// - Complete successfully
#[tokio::test]
async fn test_message_integrity_connection_pool() {
    // Set up test environment
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    // Thread-safe vector to store received messages
    let received_messages = Arc::new(Mutex::new(Vec::new()));
    
    // Create concurrent tasks for each message
    let handles: Vec<_> = (0..TOTAL_MESSAGES).map(|i| {
        let client = ctx.client.clone();  // Clone the client (cheap, shares connection)
        let messages = received_messages.clone();  // Clone Arc for shared access
        
        // Spawn async task for concurrent execution
        tokio::spawn(async move {
            // Format message with padding for consistent ordering
            let msg = format!("pooled_msg_{:04}", i);
            // Send message with timeout
            let response = timeout(
                TIMEOUT_DURATION,
                client.echo().echo(msg.clone())
            ).await
                .expect("Timeout")  // Handle timeout error
                .expect("Echo failed");  // Handle echo error
            
            // Store result with original index for ordering verification
            messages.lock().await.push((i, response));
        })
    }).collect();

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify results
    let messages = received_messages.lock().await;
    // Check no messages were lost
    assert_eq!(messages.len(), TOTAL_MESSAGES, "Messages were lost in connection pool test");
    
    // Verify message integrity and ordering
    for (i, msg) in messages.iter() {
        assert_eq!(
            msg, 
            &format!("pooled_msg_{:04}", i),
            "Message corruption in pool at index {}", i
        );
    }
}

// Test separate connections for each request
// Verifies that creating new connections for each request:
// - Doesn't cause resource exhaustion
// - Maintains message integrity
// - Handles concurrent connection creation
#[tokio::test]
async fn test_message_integrity_separate_connections() {
    let _ctx = TestContext::setup().await.expect("Failed to setup test context");
    let received_messages = Arc::new(Mutex::new(Vec::new()));
    
    let handles: Vec<_> = (0..TOTAL_MESSAGES).map(|i| {
        let messages = received_messages.clone();
        
        tokio::spawn(async move {
            // Create new test context for each request
            let new_ctx = TestContext::setup()
                .await
                .expect("Failed to create new test context");

            let msg = format!("separate_msg_{:04}", i);
            let response = timeout(
                TIMEOUT_DURATION,
                new_ctx.client.echo().echo(msg.clone())
            ).await
                .expect("Timeout")
                .expect("Echo failed");
                
            messages.lock().await.push((i, response));
        })
    }).collect();

    for handle in handles {
        handle.await.unwrap();
    }

    let messages = received_messages.lock().await;
    assert_eq!(messages.len(), TOTAL_MESSAGES, "Messages were lost in separate connections test");
    
    for (i, msg) in messages.iter() {
        assert_eq!(
            msg, 
            &format!("separate_msg_{:04}", i),
            "Message corruption in separate connection at index {}", i
        );
    }
}
