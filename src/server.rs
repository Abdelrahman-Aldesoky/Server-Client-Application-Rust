use crate::message::{client_message, server_message, ServerMessage};
use log::{error, info};
use prost::Message;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

// Client struct represents a single connected client
// Each client has its own TCP stream for communication
struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream }
    }

    // handle(): Processes incoming client messages
    // - Uses a 1024-byte buffer for reading messages
    // - Handles both EchoMessage and AddRequest message types
    // - Returns Ok(()) on success, or appropriate error on failure
    pub fn handle(&mut self) -> io::Result<()> {
        // Increased buffer size to handle larger messages
        let mut buffer = vec![0; 1024];
        
        // Enhanced message reading with proper error handling
        match self.stream.read(&mut buffer) {
            // Connection closed by client
            Ok(0) => {
                info!("Client disconnected normally");
                return Ok(());
            }
            // Successfully read n bytes
            Ok(n) => {
                // Attempt to decode the protobuf message
                if let Ok(client_msg) = crate::message::ClientMessage::decode(&buffer[..n]) {
                    match client_msg.message {
                        // Echo handler: simply sends back the same message
                        Some(client_message::Message::EchoMessage(echo)) => {
                            let response = ServerMessage {
                                message: Some(server_message::Message::EchoMessage(echo)),
                            };
                            self.send_response(response)?;
                        }
                        // Add handler: computes sum and sends back result
                        Some(client_message::Message::AddRequest(add)) => {
                            let result = add.a + add.b;
                            let response = ServerMessage {
                                message: Some(server_message::Message::AddResponse(
                                    crate::message::AddResponse { result },
                                )),
                            };
                            self.send_response(response)?;
                        }
                        None => {
                            error!("Received empty message");
                        }
                    }
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // send_response(): Helper method to encode and send responses
    // - Handles protobuf encoding
    // - Ensures complete message transmission with flush
    fn send_response(&mut self, response: ServerMessage) -> io::Result<()> {
        let mut buf = Vec::new();
        response.encode(&mut buf).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Encoding error: {}", e))
        })?;
        self.stream.write_all(&buf)?;
        self.stream.flush()
    }
}

// Server struct now includes thread handling capabilities
pub struct Server {
    listener: TcpListener,
    is_running: Arc<AtomicBool>,
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>, // Wrapped in Arc<Mutex>
}

impl Server {
    // new(): Creates a non-blocking server instance
    // - Sets up TCP listener in non-blocking mode
    // - Initializes thread management structures
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        // Non-blocking mode allows server to check shutdown signal
        listener.set_nonblocking(true)?;
        
        Ok(Server {
            listener,
            is_running: Arc::new(AtomicBool::new(false)),
            handles: Arc::new(Mutex::new(Vec::new())),
        })
    }

    // run(): Main server loop with improved concurrency
    // - Handles each client in a separate thread
    // - Implements non-blocking accept with proper error handling
    // - Includes graceful shutdown mechanism
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst);
        info!("Server running on {}", self.listener.local_addr()?);

        while self.is_running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);
                    
                    // Create new thread for each client
                    let is_running = Arc::clone(&self.is_running);
                    let mut client = Client::new(stream);
                    
                    // Store the handle instead of just spawning the thread
                    let handle = thread::spawn(move || {
                        // Client handling loop
                        while is_running.load(Ordering::SeqCst) {
                            match client.handle() {
                                Ok(()) => {}
                                // Handle non-blocking wait
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    thread::sleep(Duration::from_millis(10));
                                    continue;
                                }
                                // Handle other errors (disconnection, etc.)
                                Err(e) => {
                                    error!("Client handling error: {}", e);
                                    break;
                                }
                            }
                        }
                    });

                    // Safely store the handle
                    if let Ok(mut handles) = self.handles.lock() {
                        handles.push(handle);
                    }
                }
                // Non-blocking accept loop
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }

        Ok(())
    }

    // stop(): Graceful shutdown implementation
    // - Sets shutdown flag for all threads
    // - Allows time for cleanup
    pub fn stop(&self) {
        info!("Stopping server...");
        self.is_running.store(false, Ordering::SeqCst);
        
        // Safely join all threads
        if let Ok(mut handles) = self.handles.lock() {
            while let Some(handle) = handles.pop() {
                if let Err(e) = handle.join() {
                    error!("Error joining thread: {:?}", e);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(500));
    }
}

// Drop implementation ensures server cleanup on destruction
impl Drop for Server {
    fn drop(&mut self) {
        if self.is_running.load(Ordering::SeqCst) {
            self.stop();
        }
    }
}
