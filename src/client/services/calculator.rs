//! Calculator Service Client Implementation
//! This file demonstrates:
//! 1. Ergonomic API design for client usage
//! 2. Early validation before making RPC calls
//! 3. Error handling and status code mapping

use tonic::{Request, Status, Code};
use tracing::{info, error};
// Import the generated client and message types
use crate::proto::calculator::{
    calculator_service_client::CalculatorServiceClient,
    CalculateRequest, Operation,
};
use super::super::client::GrpcClient;

// Client-side service wrapper
// Clone allows creating multiple instances from one
#[derive(Clone)]
pub struct CalculatorService {
    // Hold the generated client with transport channel
    client: CalculatorServiceClient<tonic::transport::Channel>,
}

// Extension trait implementation for GrpcClient
impl GrpcClient {
    // Convenient method to create calculator service
    pub fn calculator(&self) -> CalculatorService {
        // Create new client using the shared channel
        CalculatorService {
            client: CalculatorServiceClient::new(self.get_channel())
        }
    }
}

// Main service implementation
impl CalculatorService {
    // High-level calculate method that handles all operations
    // Takes f64 for numbers and Operation enum for operation type
    pub async fn calculate(&mut self, first: f64, second: f64, operation: Operation) -> Result<f64, Status> {
        // Early validation for division by zero
        // Better to fail fast before making network call
        if matches!(operation, Operation::Divide) && second == 0.0 {
            return Err(Status::new(
                Code::InvalidArgument,
                "division by zero is not allowed"
            ));
        }

        info!("Sending calculate request: {} {:?} {}", first, operation, second);
        // Create and send the gRPC request
        let request = Request::new(CalculateRequest {
            first_number: first,
            second_number: second,
            operation: operation.into(),
        });

        // Handle different types of responses and errors
        match self.client.calculate(request).await {
            Ok(response) => {
                let result = response.into_inner().result;
                info!("Received calculate response: {}", result);
                Ok(result)
            },
            Err(status) if status.code() == Code::Unavailable => {
                error!("Service temporarily unavailable");
                Err(Status::new(
                    Code::Unavailable,
                    "service temporarily unavailable"
                ))
            }
            Err(e) => {
                error!("Calculate request failed: {}", e);
                Err(e)
            },
        }
    }
}

// Tests that checks if the second operand is zero that is not allowed
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculator_validation() {
        let client = GrpcClient::builder("http://[::1]:50051")
            .unwrap()
            .connect()
            .unwrap();
        
        let mut calc = client.calculator();
        
        let err = calc.calculate(10.0, 0.0, Operation::Divide).await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
        assert!(err.message().contains("division by zero"));
    }
}
