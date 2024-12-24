//! gRPC Client Binary
//! This is a command-line client that demonstrates:
//! 1. Connecting to the gRPC server
//! 2. Using multiple services (echo and calculator)
//! 3. Making async RPC calls
//! 4. Error handling with Result

// Import our client type from the main library
use embedded_recruitment_task::GrpcClient;

// Configure async runtime and provide error handling
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize and connect the client to our server
    let client = GrpcClient::builder("http://[::1]:12345")?
        .connect()?;

    // Get service handles for both available services
    let mut echo = client.echo();
    let mut calc = client.calculator();
    
    // Demonstrate echo service functionality
    let response = echo.echo("Hello OpenTier :)").await?;
    println!("Echo response: {}", response);
    
    // Demonstrate calculator service functionality with addition
    let result = calc.calculate(2.0, 3.0, embedded_recruitment_task::proto::calculator::Operation::Add).await?;
    println!("Calculator response: 2 + 3 = {}", result);
    
    Ok(())
}