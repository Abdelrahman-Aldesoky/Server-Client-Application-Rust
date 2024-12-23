//! Implementation of a simple Echo gRPC service that returns the same message it receives.
//! This serves as a good example of basic gRPC service implementation in Rust.

use tonic::{Request, Response, Status, Code};
// Import the generated protobuf code for our echo service
use crate::proto::echo::echo_service_server::EchoService;
use crate::proto::echo::{EchoRequest, EchoResponse};

// Our server implementation. We use Debug and Default traits to make it easier to create instances
// Debug: Allows printing the struct for debugging
// Default: Provides a default empty constructor
#[derive(Debug, Default)]
pub struct EchoServer {}

// This attribute generates the async implementation of our service
// The async_trait is needed because Rust doesn't support async functions in traits natively yet
#[tonic::async_trait]
impl EchoService for EchoServer {
    async fn echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        // Extract the inner request data
        let req = request.into_inner();
        
        // Input validation: Ensure the message isn't empty or just whitespace
        // This is a good practice for robust service implementation
        if req.message.trim().is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "empty message is not allowed"
            ));
        }

        // Return the same message we received
        Ok(Response::new(EchoResponse {
            message: req.message,
        }))
    }
}

// Unit tests for our echo service
// The cfg(test) attribute ensures these are only compiled during testing
#[cfg(test)]
mod tests {
    use super::*;

    // We use tokio::test instead of standard test because our service is async
    #[tokio::test]
    async fn test_echo_service() {
        let service = EchoServer::default();
        
        // Test the happy path with a valid message
        let response = service.echo(Request::new(EchoRequest {
            message: "test".into()
        })).await.unwrap();
        assert_eq!(response.into_inner().message, "test");

        // Test error handling with an empty message
        let err = service.echo(Request::new(EchoRequest {
            message: "   ".into()
        })).await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }
}