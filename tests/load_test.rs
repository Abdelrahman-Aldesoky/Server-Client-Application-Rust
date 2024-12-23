//! Load Testing Suite
//! This suite tests system performance and stability:
//! 1. Rapid-fire concurrent requests
//!    - Tests how system handles burst traffic
//!    - Verifies connection pooling effectiveness
//!    - Ensures no request failures under load
//!
//! 2. Large payload handling
//!    - Tests memory management
//!    - Verifies buffer handling
//!    - Ensures consistent performance with large data

use embedded_recruitment_task::proto::calculator::Operation;
use tokio::time::{timeout, Duration};
use common::TestContext;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

mod common;

// Test rapid-fire mixed service requests
// Purpose:
// - Verify system handles mixed workloads
// - Test service switching overhead
// - Validate connection pooling under load
#[tokio::test]
async fn test_rapid_fire_requests() {
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    // Atomic counter for thread-safe success tracking
    let success_count = Arc::new(AtomicUsize::new(0));
    let total_requests = 100;
    
    // Create concurrent request tasks
    let handles: Vec<_> = (0..total_requests).map(|i| {
        let client = ctx.client.clone();  // Share client connection
        let counter = success_count.clone();
        
        tokio::spawn(async move {
            // Alternate between echo and calculator services
            // Tests service switching overhead and connection reuse
            if i % 2 == 0 {
                // Echo service test
                let msg = format!("rapid {}", i);
                timeout(Duration::from_secs(2), client.echo().echo(msg))
                    .await.expect("Timeout").expect("Echo failed");
            } else {
                // Calculator service test
                timeout(
                    Duration::from_secs(2),
                    client.calculator().calculate(i as f64, 2.0, Operation::Multiply)
                ).await.expect("Timeout").expect("Calculate failed");
            }
            // Track successful completion
            counter.fetch_add(1, Ordering::SeqCst);
        })
    }).collect();

    // Wait for all requests to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all requests succeeded
    assert_eq!(success_count.load(Ordering::SeqCst), total_requests);
}

// Test handling of large messages in parallel
// Purpose:
// - Verify memory management with large payloads
// - Test concurrent large message handling
// - Validate system stability under memory pressure
#[tokio::test]
async fn test_parallel_large_messages() {
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    let large_msg = "A".repeat(100_000);
    
    let handles: Vec<_> = (0..5).map(|_| {
        let client = ctx.client.clone();
        let msg = large_msg.clone();
        tokio::spawn(async move {
            for _ in 0..10 {
                timeout(Duration::from_secs(5), client.echo().echo(msg.clone()))
                    .await.expect("Timeout").expect("Echo failed");
            }
        })
    }).collect();

    for handle in handles {
        handle.await.unwrap();
    }
}
