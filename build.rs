//! Build Script for Protocol Buffer Compilation
//! This file runs during the build process to:
//! 1. Generate Rust code from .proto files
//! 2. Integrate generated code with tonic gRPC framework
//! 3. Ensure protocol definitions are up-to-date

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile echo service proto file
    // This generates:
    // - Request/response structs
    // - Client stubs
    // - Server traits
    tonic_build::compile_protos("src/proto/echo.proto")?;

    // Compile calculator service proto file
    // Generated code will be placed in target directory
    // and included in the final build
    tonic_build::compile_protos("src/proto/calculator.proto")?;
    
    // Return success or propagate any compilation errors
    Ok(())
}
