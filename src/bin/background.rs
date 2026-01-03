use bong::modules::background::BackgroundTaskManager;
use bong::modules::ipc::{IpcServer, IpcCommand, BACKGROUND_IPC_PORT};
use std::sync::Arc;
use parking_lot::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Background Task starting...");
    
    // Create background task manager
    let manager = Arc::new(Mutex::new(BackgroundTaskManager::new()));
    manager.lock().start();
    
    println!("Background Task started on port {}", BACKGROUND_IPC_PORT);
    
    // Setup IPC server
    let server = IpcServer::new(BACKGROUND_IPC_PORT)?;
    let running = Arc::new(Mutex::new(true));
    let running_clone = running.clone();
    let manager_clone = manager.clone();
    
    // Listen for commands
    let result = server.listen(move |command| {
        println!("Background Task received command: {:?}", command);
        
        match command {
            IpcCommand::StopBackground | IpcCommand::QuitAll => {
                println!("Stopping background task...");
                *running_clone.lock() = false;
                Ok(None)
            }
            IpcCommand::Ping => {
                let is_running = manager_clone.lock().is_running();
                Ok(Some(IpcCommand::BackgroundStatus(is_running)))
            }
            _ => Ok(None),
        }
    });
    
    // Cleanup
    if !*running.lock() {
        manager.lock().stop().await;
        println!("Background Task stopped");
    }
    
    result
}
