//! Calculator Service Implementation
//! This file implements a gRPC calculator service that supports basic arithmetic operations.
//! It demonstrates:
//! 1. Error handling best practices
//! 2. Pattern matching in Rust
//! 3. Input validation
//! 4. Unit testing async code

use tonic::{Request, Response, Status, Code};
// Import generated Protocol Buffer code
// CalculatorService: The trait we need to implement
// CalculateRequest/Response: The message types for our RPC
// Operation: Enum defining supported mathematical operations
use crate::proto::calculator::calculator_service_server::CalculatorService;
use crate::proto::calculator::{CalculateRequest, CalculateResponse, Operation};

// CalculatorServer is our service implementation
// #[derive(Debug, Default)] automatically implements:
// - Debug: for debugging output formatting
// - Default: allows creating new instances with default values
#[derive(Debug, Default)]
pub struct CalculatorServer {}

// tonic::async_trait allows us to use async functions in trait implementations
// This is needed because Rust's native traits don't support async functions yet
#[tonic::async_trait]
impl CalculatorService for CalculatorServer {
    async fn calculate(
        &self,
        request: Request<CalculateRequest>,
    ) -> Result<Response<CalculateResponse>, Status> {
        // Extract the actual request data from the gRPC request wrapper
        let req = request.into_inner();

        // Pattern matching in Rust - a powerful way to handle different cases
        // The '?' operator at the end propagates any Err returned from the match
        let result = match req.operation() {
            // Basic arithmetic operations
            Operation::Add => Ok(req.first_number + req.second_number),
            Operation::Subtract => Ok(req.first_number - req.second_number),
            Operation::Multiply => Ok(req.first_number * req.second_number),
            Operation::Divide => {
                // Division needs special handling for division by zero
                // This is a common source of runtime errors that we validate
                if req.second_number == 0.0 {
                    Err(Status::new(
                        Code::InvalidArgument,
                        "division by zero is not allowed"
                    ))
                } else {
                    Ok(req.first_number / req.second_number)
                }
            }
        }?;  // The ? operator unwraps Ok values and returns Err values

        // Construct and return the successful response
        Ok(Response::new(CalculateResponse {
            result,
        }))
    }
}

// Test module for our calculator service
// cfg(test) ensures this code only compiles when running tests
#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;

    // tokio::test is used because our functions are async
    // It sets up the tokio runtime for each test
    #[tokio::test]
    async fn test_calculator_operations() {
        // Create a new instance of our service
        let service = CalculatorServer::default();

        // Test addition operation
        // This demonstrates the "happy path" where everything works
        let response = service.calculate(Request::new(CalculateRequest {
            first_number: 5.0,
            second_number: 3.0,
            operation: Operation::Add.into(),
        })).await.unwrap();
        assert_eq!(response.into_inner().result, 8.0);

        // Test division by zero
        // This demonstrates error handling
        let err = service.calculate(Request::new(CalculateRequest {
            first_number: 5.0,
            second_number: 0.0,
            operation: Operation::Divide.into(),
        })).await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }
}
