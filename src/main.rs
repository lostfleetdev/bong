mod modules;

use modules::tray::TrayManager;
use modules::ipc::{IpcClient, IpcCommand, BACKGROUND_IPC_PORT, UI_IPC_PORT};
use std::sync::Arc;
use parking_lot::Mutex;
use tray_icon::menu::MenuEvent;
use std::process::{Command, Child};

/// Process manager for background and UI processes
struct ProcessManager {
    background_process: Arc<Mutex<Option<Child>>>,
    ui_process: Arc<Mutex<Option<Child>>>,
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            background_process: Arc::new(Mutex::new(None)),
            ui_process: Arc::new(Mutex::new(None)),
        }
    }

    fn start_background(&self) -> anyhow::Result<()> {
        let mut process_lock = self.background_process.lock();
        
        // Check if already running
        if let Some(ref mut child) = *process_lock {
            if child.try_wait()?.is_none() {
                println!("Background task already running");
                return Ok(());
            }
        }
        
        // Get the executable directory
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();
        let background_exe = exe_dir.join("bong-background.exe");
        
        println!("Starting background task: {:?}", background_exe);
        let child = Command::new(&background_exe).spawn()?;
        *process_lock = Some(child);
        
        // Give it a moment to start
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("Background task started");
        
        Ok(())
    }

    fn stop_background(&self) -> anyhow::Result<()> {
        println!("Stopping background task...");
        
        // Send stop command via IPC
        let client = IpcClient::new(BACKGROUND_IPC_PORT);
        let _ = client.send(IpcCommand::StopBackground);
        
        // Give it time to gracefully shutdown
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        // Force kill if still running
        let mut process_lock = self.background_process.lock();
        if let Some(ref mut child) = *process_lock {
            let _ = child.kill();
            let _ = child.wait();
        }
        *process_lock = None;
        
        println!("Background task stopped");
        Ok(())
    }

    fn start_ui(&self) -> anyhow::Result<()> {
        let mut process_lock = self.ui_process.lock();
        
        // Check if already running
        if let Some(ref mut child) = *process_lock {
            if child.try_wait()?.is_none() {
                println!("UI already running");
                return Ok(());
            }
        }
        
        // Get the executable directory
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();
        let ui_exe = exe_dir.join("bong-ui.exe");
        
        println!("Starting UI: {:?}", ui_exe);
        let child = Command::new(&ui_exe).spawn()?;
        *process_lock = Some(child);
        
        // Give it a moment to start
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("UI started");
        
        Ok(())
    }

    fn stop_ui(&self) -> anyhow::Result<()> {
        println!("Stopping UI...");
        
        // Send close command via IPC
        let client = IpcClient::new(UI_IPC_PORT);
        let _ = client.send(IpcCommand::CloseUI);
        
        // Give it time to gracefully shutdown
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        // Force kill if still running
        let mut process_lock = self.ui_process.lock();
        if let Some(ref mut child) = *process_lock {
            let _ = child.kill();
            let _ = child.wait();
        }
        *process_lock = None;
        
        println!("UI stopped");
        Ok(())
    }

    fn stop_all(&self) -> anyhow::Result<()> {
        self.stop_ui()?;
        self.stop_background()?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    println!("Bong Tray starting...");
    
    // Create process manager
    let process_manager = Arc::new(ProcessManager::new());
    
    // Auto-start background and UI
    process_manager.start_background()?;
    process_manager.start_ui()?;
    
    // Setup tray icon
    let mut tray_manager = TrayManager::new()?;
    let menu_items = tray_manager.setup()?;
    println!("Tray icon initialized");
    
    // Clone menu IDs for event handling
    let open_id = menu_items.open_id.clone();
    let quit_id = menu_items.quit_id.clone();
    
    // Main event loop
    let process_manager_clone = process_manager.clone();
    loop {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == open_id {
                println!("Open clicked - Starting/Showing UI");
                let _ = process_manager_clone.start_ui();
            } else if event.id == quit_id {
                println!("Exit clicked - Shutting down everything");
                let _ = process_manager_clone.stop_all();
                std::thread::sleep(std::time::Duration::from_millis(500));
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    println!("Bong Tray exiting");
    Ok(())
}