use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use parking_lot::Mutex;
use std::time::Duration;

/// IPC Commands that can be sent between processes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcCommand {
    /// Start the background task
    StartBackground,
    /// Stop the background task
    StopBackground,
    /// Start the UI
    StartUI,
    /// Close the UI
    CloseUI,
    /// Quit everything
    QuitAll,
    /// Ping to check if process is alive
    Ping,
    /// Response to ping
    Pong,
    /// Background task status
    BackgroundStatus(bool),
    /// UI status
    UIStatus(bool),
}

/// IPC Server for receiving commands
pub struct IpcServer {
    port: u16,
    listener: Option<TcpListener>,
    running: Arc<Mutex<bool>>,
}

impl IpcServer {
    pub fn new(port: u16) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        listener.set_nonblocking(true)?;
        
        Ok(Self {
            port,
            listener: Some(listener),
            running: Arc::new(Mutex::new(true)),
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// Listen for incoming commands with a callback
    pub fn listen<F>(&self, mut callback: F) -> anyhow::Result<()>
    where
        F: FnMut(IpcCommand) -> anyhow::Result<Option<IpcCommand>>,
    {
        let listener = self.listener.as_ref().unwrap();
        
        while *self.running.lock() {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = Vec::new();
                    let mut temp_buf = [0u8; 1024];
                    
                    // Read the command
                    loop {
                        match stream.read(&mut temp_buf) {
                            Ok(0) => break,
                            Ok(n) => buffer.extend_from_slice(&temp_buf[..n]),
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                std::thread::sleep(Duration::from_millis(10));
                                if buffer.len() > 0 {
                                    break;
                                }
                            }
                            Err(e) => return Err(e.into()),
                        }
                    }
                    
                    if let Ok(command) = serde_json::from_slice::<IpcCommand>(&buffer) {
                        // Process command and get optional response
                        if let Ok(Some(response)) = callback(command) {
                            let response_data = serde_json::to_vec(&response)?;
                            let _ = stream.write_all(&response_data);
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => return Err(e.into()),
            }
        }
        
        Ok(())
    }

    pub fn stop(&self) {
        *self.running.lock() = false;
    }
}

/// IPC Client for sending commands
pub struct IpcClient {
    port: u16,
}

impl IpcClient {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Send a command to the server
    pub fn send(&self, command: IpcCommand) -> anyhow::Result<()> {
        self.send_with_response(command)?;
        Ok(())
    }

    /// Send a command and wait for a response
    pub fn send_with_response(&self, command: IpcCommand) -> anyhow::Result<Option<IpcCommand>> {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", self.port))?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        
        let data = serde_json::to_vec(&command)?;
        stream.write_all(&data)?;
        stream.flush()?;
        
        // Try to read response
        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1024];
        
        match stream.read(&mut temp_buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                buffer.extend_from_slice(&temp_buf[..n]);
                Ok(serde_json::from_slice::<IpcCommand>(&buffer).ok())
            }
            Err(_) => Ok(None),
        }
    }
}

/// Well-known ports for IPC
pub const BACKGROUND_IPC_PORT: u16 = 45789;
pub const UI_IPC_PORT: u16 = 45790;
