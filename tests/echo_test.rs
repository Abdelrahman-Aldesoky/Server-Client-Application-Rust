//! Echo Service Integration Tests
//! This suite verifies the robustness of string handling:
//! 1. Basic ASCII message handling
//! 2. Complex Unicode support
//!    - Emojis
//!    - Right-to-left text
//!    - CJK characters
//!    - Mixed scripts
//! 3. Special characters and formatting
//!    - Control characters
//!    - Zero bytes
//!    - JSON-like content
//! 4. Large message handling
//! 5. Performance under various payloads

use tokio::time::{timeout, Duration};
use common::TestContext;

mod common;

// Basic functionality test
// Verifies:
// - Simple message echo works
// - Timeout mechanism functions
// - Basic client-server communication
#[tokio::test]
async fn test_echo_simple() {
    // Setup the test context
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    // Define the test message
    let test_msg = "hello";
    // Send the echo request and wait for the response with a timeout
    let response = timeout(
        Duration::from_secs(5),
        ctx.client.echo().echo(test_msg)
    ).await
        .expect("Test timed out")
        .expect("Echo request failed");
    
    // Verify that the response matches the test message
    assert_eq!(response, test_msg);
}

// Unicode handling test
// Verifies:
// - Complex character encoding preservation
// - Multi-byte character support
// - Different writing systems
// - Emoji handling
// - Unicode escape sequences
#[tokio::test]
async fn test_echo_unicode() {
    // Setup the test context
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    
    // Define the test cases with different Unicode messages
    let test_cases = vec![
        ("Emoji Test", "Hello 🌍 🚀 💻"),
        ("RTL Text", "عبدالرحمن"),
        ("CJK Text", "你好，世界"),
        ("Mixed Scripts", "Hello مرحبا 你好"),
        ("Unicode Escapes", "Hello\u{1F600}\u{1F602}"),
    ];

    // Iterate over each test case
    for (name, msg) in test_cases {
        // Send the echo request and wait for the response with a timeout
        let response = timeout(
            Duration::from_secs(5),
            ctx.client.echo().echo(msg)
        ).await
            .expect(&format!("{} timed out", name))
            .expect(&format!("{} failed", name));
        
        // Verify that the response matches the test message
        assert_eq!(response, msg, "{} failed equality check", name);
    }
}

// Special formatting test
// Verifies:
// - Control character preservation
// - Whitespace handling
// - Null byte handling
// - Multi-line text support
// - Structured text (JSON) handling
#[tokio::test]
async fn test_echo_formatting() {
    // Setup the test context
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    
    // Define the test cases with different special formatting messages
    let test_cases = vec![
        ("Control Chars", "Hello\nWorld\tTab\rReturn"),
        ("Whitespace", "Hello    World    "),
        ("Zero Bytes", "Hello\0World\0"),
        ("Mixed Format", "Line1\n  Line2\r\n\tLine3"),
        ("JSON-like", r#"{"key": "value"}"#),
    ];

    // Iterate over each test case
    for (name, msg) in test_cases {
        // Send the echo request and wait for the response with a timeout
        let response = timeout(
            Duration::from_secs(5),
            ctx.client.echo().echo(msg)
        ).await
            .expect(&format!("{} timed out", name))
            .expect(&format!("{} failed", name));
        
        // Verify that the response matches the test message
        assert_eq!(response, msg, "{} failed equality check", name);
    }
}

// Large message test
// Verifies:
// - Buffer handling for large payloads
// - Memory management
// - Performance with large strings
// - Timeout adequacy for large messages
#[tokio::test]
async fn test_echo_long_message() {
    // Setup the test context
    let ctx = TestContext::setup().await.unwrap();
    // Define a long test message
    let long_msg = "a".repeat(1000000);
    // Send the echo request and wait for the response with a timeout
    let response = timeout(
        Duration::from_secs(5),
        ctx.client.echo().echo(long_msg.clone())
    ).await
        .expect("Test timed out")
        .expect("Long message echo failed");
    // Verify that the response matches the long test message
    assert_eq!(response, long_msg);
}
