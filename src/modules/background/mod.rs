use std::sync::Arc;
use parking_lot::RwLock;
use tokio::task::JoinHandle;

/// Background task manager that runs tasks independently of the UI
pub struct BackgroundTaskManager {
    is_running: Arc<RwLock<bool>>,
    task_handle: Option<JoinHandle<()>>,
}

impl BackgroundTaskManager {
    /// Create a new background task manager
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(RwLock::new(false)),
            task_handle: None,
        }
    }

    /// Start the background tasks
    pub fn start(&mut self) {
        let is_running = self.is_running.clone();
        *is_running.write() = true;

        let is_running_clone = is_running.clone();
        let handle = tokio::spawn(async move {
            println!("Background task started");
            
            while *is_running_clone.read() {
                // Perform background work here
                // Example: periodic checks, data syncing, monitoring, etc.
                
                // Simulate some background work
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                if *is_running_clone.read() {
                    println!("Background task running... (heartbeat)");
                }
            }
            
            println!("Background task stopped");
        });

        self.task_handle = Some(handle);
    }

    /// Stop the background tasks
    pub async fn stop(&mut self) {
        *self.is_running.write() = false;
        
        if let Some(handle) = self.task_handle.take() {
            let _ = handle.await;
        }
    }

    /// Check if background tasks are running
    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        *self.is_running.read()
    }
}

impl Default for BackgroundTaskManager {
    fn default() -> Self {
        Self::new()
    }
}
