# Project Overhaul: Modern Async gRPC Implementation

## Project Structure

## Commit 1: Initial Project Restructure

### File Structure Changes
1. **Module Organization**
   - Moved `client.rs` from `tests/` to the `src/` directory.
   - Updated module declarations in `lib.rs`.
   - Fixed import paths to use crate-relative paths.

2. **Import Path Updates**
   - Changed from using external crate paths to internal crate paths.
   - Updated `client_test.rs` to import `Client` from the main crate.
   - Removed redundant `mod client` declaration from `client_test.rs`.

### Summary
These changes improve the project structure by properly separating implementation from tests.

*Note: This is the first of many small, incremental improvements.*

## Commit 2: Fix Client Port Variable Type and Rename Test File

### Changes
1. **Port Variable Type Change**
   - Changed the port variable type in the `Client` struct from `u32` to `u16`.
   - This better matches TCP/UDP port number specifications (0-65535).
   - It is more memory efficient for port number storage.

2. **Test File Reorganization**
   - Renamed `client_tests.rs` to `original_integration_tests.rs`.
   - Preserved original task tests as a reference implementation.
   - Prepared the codebase for new test files with additional cases.
   - Better distinguished between provided and new test cases.

## Commit 3: Complete Architectural Overhaul - Modern Async gRPC Implementation 

### Project Structure

      .
├── src
│   ├── client
│   │   ├── client.rs
│   │   ├── mod.rs
│   │   └── services
│   │       ├── calculator.rs
│   │       ├── echo.rs
│   │       └── mod.rs
│   ├── lib.rs
│   ├── proto
│   │   ├── calculator.proto
│   │   ├── echo.proto
│   │   └── mod.rs
│   │── server
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── services
│   │       ├── calculator.rs
│   │       ├── echo.rs
│   │       └── mod.rs
├── tests
│   ├── calculator_test.rs
│   ├── common
│   │   ├── mod.rs
│   │   └── test_utils.rs
│   ├── connection_stress_test.rs
│   ├── echo_test.rs
│   ├── load_test.rs
│   └── message_integrity_test.rs
├── Cargo.lock
├── Cargo.toml
├── README.md
├── SOLUTION.md
├── SOLUTION_2.md
└── build.rs


### Changes and Improvements

1. **Transition to Tonic:**
   - **Commit 2:**
     - Custom TCP server and client implementation.
     - Manual handling of connections, message encoding/decoding, and error handling.
     - Limited concurrency handling.
   - **Commit 3:**
     - Replaced with Tonic for gRPC framework.
     - Tonic provides built-in support for async operations, automatic connection pooling, retries, and exponential backoff.
     - Leveraged Tonic's features such as lazy connections, automatic retries, connection timeouts, and disconnection of idle clients to save resources.

2. **Improved Project Structure:**
   - **Commit 2:**
     - Flat structure with server and client code mixed together.
     - Limited modularity and separation of concerns.
   - **Commit 3:**
     - Organized into clear modules: `client`, `server`, and `proto`.
     - Separated service implementations into individual files for better maintainability.
     - Utilized Rust's module system to create a clean and modular codebase.
     - Followed SOLID principles and DRY (Don't Repeat Yourself) practices for better code maintainability and scalability.

3. **Client and Server Enhancements:**
   - **Commit 2:**
     - Basic TCP client and server with limited functionality.
     - Manual handling of message types and error conditions.
     - No retries, timeouts, or connection pooling.
   - **Commit 3:**
     - Implemented gRPC services for Echo and Calculator functionalities.
     - Used Tonic's generated client and server code for type-safe and efficient communication.
     - Added comprehensive error handling and validation for client and server operations.
     - Automatic connection pooling, retries, and exponential backoff provided by Tonic.
     - Lazy connections and disconnection of idle clients to save resources.

4. **Testing Improvements:**
   - **Commit 2:**
     - Limited test suite with only 5 tests.
     - Sequential test execution using `serial_test` crate.
   - **Commit 3:**
     - Transitioned to a comprehensive suite of unit and integration tests.
     - Added unit tests for each service file, ensuring thorough coverage.
     - Developed extensive integration tests in the `tests` folder, covering various scenarios such as connection stress, load handling, message integrity, and more.
     - Total of 11 integration tests.
     - Testing now uses atomic dynamic port allocation for tests to run concurrently, ensuring isolated test environments and avoiding port conflicts.

5. **Concurrency and Scalability:**
   - **Commit 2:**
     - Basic concurrency handling with manual thread management.
     - Limited scalability and performance.
   - **Commit 3:**
     - Leveraged Tonic's multi-threaded runtime for handling multiple clients concurrently.
     - Implemented proper synchronization mechanisms to ensure thread safety.
     - Used async/await syntax for non-blocking operations, improving performance and responsiveness.
     - Scalable architecture capable of handling high concurrent loads efficiently.

6. **Builder Pattern:**
   - **Commit 2:**
     - No builder pattern for client and server configuration.
   - **Commit 3:**
     - Introduced the Builder pattern for flexible and configurable client and server creation.
     - Simplified the setup and configuration of gRPC services.

7. **Protocol Buffers:**
   - **Commit 2:**
     - Used `prost` for Protocol Buffers implementation.
     - Manual handling of message encoding/decoding.
   - **Commit 3:**
     - Defined message types and services using Protocol Buffers (`calculator.proto` and `echo.proto`).
     - Used `prost` for efficient serialization and deserialization of messages.

#### Linux Environment Setup
Run these commands before executing cargo test:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Protocol Buffers compiler
sudo apt update
sudo apt install -y protobuf-compiler

# Install OpenSSL dev libraries
sudo apt install -y pkg-config libssl-dev

# Verify installations
rustc --version
protoc --version
```

Then execute:
```bash
cargo test
```

### Benefits

1. **Scalability:**
   - The new architecture can handle multiple clients concurrently without blocking.
   - Efficient resource management with Tonic's async runtime and connection pooling.

2. **Maintainability:**
   - Clear separation of concerns with modular code structure.
   - Easy to extend and modify individual components without affecting the entire system.

3. **Reliability:**
   - Comprehensive error handling and validation ensure robust and predictable behavior.
   - Graceful shutdown and resource cleanup prevent memory leaks and resource exhaustion.

4. **Performance:**
   - Non-blocking async operations improve throughput and reduce latency.
   - Efficient serialization with `prost` and gRPC's binary protocol.

5. **Future-Proofing:**
   - The architecture is designed to be easily extendable with new features like TLS/SSL, metrics, and more.
   - Tonic provides a solid foundation for building production-ready systems.

6. **Testing:**
   - Improved test coverage with async tests.
   - Reliable and consistent test results with proper synchronization and error handling.
   - Testing now uses atomic dynamic port allocation for tests to run concurrently, ensuring isolated test environments and avoiding port conflicts.

7. **Flexibility:**
   - The current structure is organized as a library but can easily be separated into a separate client and server implementation as executables.

### Future Improvements

1. **Logging:**
   - Implement comprehensive logging for better observability and debugging.
   - Use an async logging framework like `tracing` or `log` to capture detailed logs.
   - Configure log levels (info, debug, error) and output formats.
   - Ensure logs are structured and can be easily integrated with monitoring tools.
   - The scalable and maintainable architecture makes it easy to add logging without significant changes to the codebase.
   - Async logging is preferred as synchronous logging caused issues during testing, leading to its removal from the current implementation.

2. **Enhanced Security:**
   - Add TLS/SSL support for encrypted communications.
   - Implement authentication mechanisms and rate limiting for clients.

3. **Monitoring and Metrics:**
   - Add performance metrics collection and health checks.
   - Monitor system resources and log connection statistics.

4. **Configuration Management:**
   - Make server parameters configurable through config files.
   - Support dynamic configuration updates without restart and environment-specific configurations.

### Summary

The new implementation provides a robust, scalable, and maintainable architecture for the gRPC server and client.
By leveraging Tonic, we have achieved significant improvements in concurrency, performance, and reliability. The project is now well-structured, future-proof, and maybe ready for production use.