//! Connection Stress Testing Suite
//! 
//! Purpose:
//! - Validate server behavior under extreme connection load
//! - Test connection pooling and resource management
//! - Verify service stability with mixed operations
//! - Ensure graceful handling of concurrent requests
//!
//! Test Strategy:
//! 1. Create many concurrent clients (1000)
//! 2. Each client performs multiple operations (10)
//! 3. Mix different operation types (echo, calculate, large payloads)
//! 4. Track successful operations using atomic counter
//! 5. Verify all operations complete successfully

// Imports for async operations, atomic counters, and timeouts
use embedded_recruitment_task::proto::calculator::Operation;
use tokio::time::{timeout, Duration};
use common::TestContext;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

mod common;

// Test configuration constants
const CONCURRENT_CLIENTS: usize = 1000;   // Simulates high concurrent load
const OPERATIONS_PER_CLIENT: usize = 10;  // Multiple operations per client for sustained load
const TIMEOUT_DURATION: Duration = Duration::from_secs(10);  // Maximum time for any operation

#[tokio::test]
async fn test_massive_concurrent_load() {
    // Initialize test environment
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    
    // Atomic counter for tracking successful operations
    // Using atomic operations for thread-safe counting
    let success_count = Arc::new(AtomicUsize::new(0));
    let expected_total = CONCURRENT_CLIENTS * OPERATIONS_PER_CLIENT;
    
    // Create concurrent client tasks
    let handles: Vec<_> = (0..CONCURRENT_CLIENTS).map(|client_id| {
        // Clone references for the async task
        let client = ctx.client.clone();
        let counter = success_count.clone();
        
        // Spawn individual client task
        tokio::spawn(async move {
            // Each client performs multiple operations
            for op_id in 0..OPERATIONS_PER_CLIENT {
                // Rotate through different operation types
                match op_id % 3 {
                    0 => {
                        // Simple echo operation
                        let msg = format!("client_{}_op_{}", client_id, op_id);
                        timeout(TIMEOUT_DURATION, client.echo().echo(msg))
                            .await.expect("Timeout").expect("Echo failed");
                    },
                    1 => {
                        // Calculator operation
                        timeout(
                            TIMEOUT_DURATION,
                            client.calculator().calculate(
                                client_id as f64,
                                op_id as f64,
                                Operation::Add
                            )
                        ).await.expect("Timeout").expect("Calculate failed");
                    },
                    _ => {
                        // Large message echo operation
                        let msg = format!("large_{}_{}", client_id, "X".repeat(1000));
                        timeout(TIMEOUT_DURATION, client.echo().echo(msg))
                            .await.expect("Timeout").expect("Large message failed");
                    }
                }
                // Increment success counter atomically
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
    }).collect();

    // Wait for all client tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all operations completed successfully
    let final_count = success_count.load(Ordering::SeqCst);
    assert_eq!(
        final_count, 
        expected_total,
        "Expected {} operations but got {}", 
        expected_total, 
        final_count
    );
}
