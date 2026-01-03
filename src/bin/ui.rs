use gpui::*;
use gpui_component::*;
use bong::modules::ui::BongApp;
use bong::modules::ipc::{IpcServer, IpcCommand, UI_IPC_PORT};
use std::sync::Arc;
use parking_lot::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("UI starting...");
    
    // Setup IPC server in a separate thread
    let should_close = Arc::new(Mutex::new(false));
    let should_close_clone = should_close.clone();
    
    std::thread::spawn(move || {
        if let Ok(server) = IpcServer::new(UI_IPC_PORT) {
            println!("UI IPC server listening on port {}", UI_IPC_PORT);
            
            let _ = server.listen(move |command| {
                println!("UI received command: {:?}", command);
                
                match command {
                    IpcCommand::CloseUI | IpcCommand::QuitAll => {
                        println!("UI closing by request...");
                        *should_close_clone.lock() = true;
                        std::process::exit(0);
                    }
                    IpcCommand::Ping => {
                        Ok(Some(IpcCommand::UIStatus(true)))
                    }
                    _ => Ok(None),
                }
            });
        }
    });
    
    // Create GPUI application
    let app = Application::new().with_assets(gpui_component_assets::Assets);
    
    // Run GPUI application
    app.run(move |cx| {
        gpui_component::init(cx);
        
        // Allow window to close (and close the app)
        cx.on_window_closed(|cx| {
            println!("UI window closed - exiting UI process");
            cx.quit();
        }).detach();
        
        cx.spawn(async move |cx| {
            let _window_handle = cx.open_window(
                WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(700.0)),
                    })),
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_background: WindowBackgroundAppearance::Opaque,
                    app_id: Some("com.bong.ui".to_string()),
                    window_min_size: Some(size(px(600.0), px(400.0))),
                    ..Default::default()
                },
                |window, cx| {
                    // Set up window close handler
                    window.on_window_should_close(cx, |_window, cx| {
                        println!("UI window closing - app will exit");
                        cx.quit();
                        true
                    });
                    
                    let view = cx.new(|cx| BongApp::new(cx));
                    cx.new(|cx| Root::new(view, window, cx))
                }, 
            )?;
            
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
    
    Ok(())
}
