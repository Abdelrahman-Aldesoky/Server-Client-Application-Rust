//! Calculator Service Integration Tests
//! This test suite verifies:
//! 1. Mathematical correctness of all operations
//! 2. Handling of edge cases and special numbers
//! 3. Error conditions and validation
//! 4. Floating-point precision requirements
//! 5. Timeout handling for operations

use embedded_recruitment_task::proto::calculator::Operation;
use tonic::Code;
use tokio::time::{timeout, Duration};
use common::TestContext;

mod common;

// Comprehensive test of all calculator operations
// Tests various number combinations:
// - Regular integers
// - Floating point numbers
// - Very large numbers (1e10)
// - Very small numbers (1e-10)
// - Negative numbers
// - Mixed sign operations
#[tokio::test]
async fn test_basic_operations() {
    // Initialize test environment
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    let mut calculator = ctx.client.calculator();

    // Test cases designed to verify:
    // - Basic arithmetic correctness
    // - Floating point precision
    // - Edge case handling
    // - Number range support
    let test_cases: Vec<(&str, f64, f64, Operation, Result<f64, Code>)> = vec![
        // Basic arithmetic with regular numbers
        ("Addition", 10.0, 5.0, Operation::Add, Ok(15.0)),
        ("Subtraction", 10.0, 5.0, Operation::Subtract, Ok(5.0)),
        ("Multiplication", 10.0, 5.0, Operation::Multiply, Ok(50.0)),
        ("Division", 10.0, 5.0, Operation::Divide, Ok(2.0)),
        
        // Edge cases with extreme numbers
        ("Large Numbers", 1e10, 2.0, Operation::Multiply, Ok(2e10)),
        ("Small Numbers", 1e-10, 2.0, Operation::Multiply, Ok(2e-10)),
        
        // Sign handling
        ("Negative Numbers", -10.0, -5.0, Operation::Add, Ok(-15.0)),
        ("Mixed Signs", -10.0, 5.0, Operation::Multiply, Ok(-50.0)),
        
        // Precision test
        ("Near Zero", 1e-15, 1e15, Operation::Multiply, Ok(1.0)),
    ];

    // Execute each test case with timeout protection
    for (name, first, second, op, expected) in test_cases {
        let result = timeout(
            Duration::from_secs(5),
            calculator.calculate(first, second, op)
        ).await
            .expect(&format!("{} timed out", name))
            .expect(&format!("{} failed", name));

        // Verify results with floating-point tolerance
        match expected {
            Ok(expected_val) => assert!((result - expected_val).abs() < 1e-10),
            Err(_) => panic!("Unexpected error for {}", name),
        }
    }
}

// Test error handling scenarios
// Focuses on invalid operations and error responses
#[tokio::test]
async fn test_error_cases() {
    // Initialize test environment
    let ctx = TestContext::setup().await.expect("Failed to setup test context");
    let mut calculator = ctx.client.calculator();

    // Error test cases
    // Each case should result in an InvalidArgument status
    let test_cases = vec![
        // Classic division by zero
        ("Division by Zero", 10.0, 0.0, Operation::Divide),
        // Edge case: 0/0 is undefined
        ("Zero Division Zero", 0.0, 0.0, Operation::Divide),
    ];

    // Verify each error case
    for (name, first, second, op) in test_cases {
        let err = timeout(
            Duration::from_secs(5),
            calculator.calculate(first, second, op)
        ).await
            .expect(&format!("{} timed out", name))
            .unwrap_err();
        
        // All division by zero cases should return InvalidArgument
        assert_eq!(err.code(), Code::InvalidArgument);
    }
}
