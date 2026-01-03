use tray_icon::{
    menu::{Menu, MenuItem, MenuId},
    TrayIcon, TrayIconBuilder, Icon,
};
use std::path::PathBuf;

/// Menu item identifiers
pub struct MenuItems {
    pub open_id: MenuId,
    pub quit_id: MenuId,
}

/// Tray icon manager for the application
pub struct TrayManager {
    _tray_icon: Option<TrayIcon>,
}

impl TrayManager {
    /// Create a new tray manager
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            _tray_icon: None,
        })
    }

    /// Initialize and show the tray icon
    pub fn setup(&mut self) -> anyhow::Result<MenuItems> {
        // Create menu items
        let open_item = MenuItem::new("Open", true, None);
        let quit_item = MenuItem::new("Exit", true, None);

        let open_id = open_item.id().clone();
        let quit_id = quit_item.id().clone();

        // Build the menu
        let menu = Menu::new();
        menu.append(&open_item)?;
        menu.append(&quit_item)?;

        // Load icon from file
        let icon = Self::load_icon_from_file()?;

        // Build the tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Bong App")
            .with_icon(icon)
            .build()?;

        self._tray_icon = Some(tray_icon);

        Ok(MenuItems { open_id, quit_id })
    }

    /// Load icon from icons/app.ico file
    fn load_icon_from_file() -> anyhow::Result<Icon> {
        // Try to load from src/icons/app.ico (when running in dev)
        let mut icon_path = PathBuf::from("src/icons/app.ico");
        
        // If not found, try relative to executable (when running as built binary)
        if !icon_path.exists() {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    icon_path = exe_dir.join("icons/app.ico");
                }
            }
        }
        
        // Load and decode the icon
        if icon_path.exists() {
            let icon_data = std::fs::read(&icon_path)
                .map_err(|e| anyhow::anyhow!("Failed to read icon file: {}", e))?;
            
            let img = image::load_from_memory(&icon_data)
                .map_err(|e| anyhow::anyhow!("Failed to decode icon: {}", e))?;
            
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            
            Icon::from_rgba(rgba.into_raw(), width, height)
                .map_err(|e| anyhow::anyhow!("Failed to create tray icon: {:?}", e))
        } else {
            // Fallback: Create a simple default icon if file not found
            let width = 32u32;
            let height = 32u32;
            let mut rgba = vec![0u8; (width * height * 4) as usize];
            
            let center_x = width / 2;
            let center_y = height / 2;
            let radius = 12.0;
            
            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - center_x as f32;
                    let dy = y as f32 - center_y as f32;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    let idx = ((y * width + x) * 4) as usize;
                    if distance <= radius {
                        rgba[idx] = 100;
                        rgba[idx + 1] = 150;
                        rgba[idx + 2] = 255;
                        rgba[idx + 3] = 255;
                    } else {
                        rgba[idx + 3] = 0;
                    }
                }
            }
            
            Icon::from_rgba(rgba, width, height)
                .map_err(|e| anyhow::anyhow!("Failed to create icon: {:?}", e))
        }
    }
}

impl Default for TrayManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
