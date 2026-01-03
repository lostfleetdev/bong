mod search;
mod downloads;

pub use search::SearchView;
pub use downloads::DownloadsView;

use gpui::*;
use gpui_component::*;
use gpui_component::button::Button;
use gpui_component::IconNamed;
use gpui_component::menu::DropdownMenu;
use gpui_component::menu::PopupMenuItem;


#[allow(dead_code)]
pub enum AppIcon {
    BongApp,
}

impl IconNamed for AppIcon {
    fn path(self) -> gpui::SharedString {
        match self {
            AppIcon::BongApp => "icons/app.svg",
        }
        .into()
    }
}

/// Main application view
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    Downloads,
    Search,
}

/// Main application view
pub struct BongApp {
    aria2c_status: Aria2cStatus,
    view_mode: ViewMode,
    search_view: Entity<SearchView>,
    downloads_view: Entity<DownloadsView>,
} 

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Aria2cStatus {
    Running,
    Stopped,
    Error,
}

impl BongApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            aria2c_status: Aria2cStatus::Stopped,
            view_mode: ViewMode::Downloads,
            search_view: cx.new(|_| SearchView::new()),
            downloads_view: cx.new(|_| DownloadsView::new()),
        }
    }
    
    fn get_aria2c_icon(&self) -> IconName {
        match self.aria2c_status {
            Aria2cStatus::Running => IconName::Heart,
            Aria2cStatus::Stopped => IconName::HeartOff,
            Aria2cStatus::Error => IconName::CircleX,
        }
    }
    
    fn get_aria2c_color(&self) -> Hsla {
        match self.aria2c_status {
            Aria2cStatus::Running => rgb(0x22c55e).into(), // green
            Aria2cStatus::Stopped => rgb(0x6b7280).into(), // gray
            Aria2cStatus::Error => rgb(0xef4444).into(),   // red
        }
    }
    
    fn get_aria2c_status_text(&self) -> &'static str {
        match self.aria2c_status {
            Aria2cStatus::Running => "Running",
            Aria2cStatus::Stopped => "Stopped",
            Aria2cStatus::Error => "Error",
        }
    }
}

impl Render for BongApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let status = self.aria2c_status;
        let status_color = self.get_aria2c_color();
        
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .font_family("Atkinson Hyperlegible")
            .child(
                TitleBar::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .w_full()
                            .h(px(40.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        div()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_sm()
                                            .text_color(cx.theme().foreground)
                                            .child("bong")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .justify_end()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        // Aria2c Status Dropdown
                                        Button::new("aria2c-status")
                                            .icon(self.get_aria2c_icon())
                                            .text_color(status_color)
                                            .dropdown_menu_with_anchor(Corner::BottomRight, move |menu, window, cx| {
                                                menu
                                                    .label("aria2c Status")
                                                    .separator()
                                                    .item(
                                                        PopupMenuItem::new(format!("Status: {}", match status {
                                                            Aria2cStatus::Running => "Running",
                                                            Aria2cStatus::Stopped => "Stopped",
                                                            Aria2cStatus::Error => "Error",
                                                        }))
                                                        .disabled(true)
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Downloads: 0 active")
                                                            .disabled(true)
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Port: 6800")
                                                            .disabled(true)
                                                    )
                                                    .separator()
                                                    .label("Settings")
                                                    .item(
                                                        PopupMenuItem::new("Start aria2c")
                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                println!("Start aria2c clicked");
                                                                this.aria2c_status = Aria2cStatus::Running;
                                                                cx.notify();
                                                            }))
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Stop aria2c")
                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                println!("Stop aria2c clicked");
                                                                this.aria2c_status = Aria2cStatus::Stopped;
                                                                cx.notify();
                                                            }))
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Configuration")
                                                            .icon(IconName::Settings)
                                                            .on_click(|_, window, cx| {
                                                                println!("aria2c configuration clicked");
                                                            })
                                                    )
                                            })
                                    )
                                    .child(
                                        // Settings Dropdown
                                        Button::new("settings")
                                            .icon(IconName::Settings)
                                            .dropdown_menu_with_anchor(Corner::BottomRight, |menu, window, cx| {
                                                menu
                                                    .label("Settings")
                                                    .separator()
                                                    .item(
                                                        PopupMenuItem::new("General")
                                                            .icon(IconName::Settings)
                                                            .on_click(|_, window, cx| {
                                                                println!("General settings clicked");
                                                            })
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Appearance")
                                                            .icon(IconName::Palette)
                                                            .on_click(|_, window, cx| {
                                                                println!("Appearance settings clicked");
                                                            })
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Downloads")
                                                            .icon(IconName::ArrowDown)
                                                            .on_click(|_, window, cx| {
                                                                println!("Download settings clicked");
                                                            })
                                                    )
                                                    .item(
                                                        PopupMenuItem::new("Network")
                                                            .icon(IconName::Globe)
                                                            .on_click(|_, window, cx| {
                                                                println!("Network settings clicked");
                                                            })
                                                    )
                                                    .separator()
                                                    .item(
                                                        PopupMenuItem::new("About")
                                                            .icon(IconName::Info)
                                                            .on_click(|_, window, cx| {
                                                                println!("About clicked");
                                                            })
                                                    )
                                            })
                                    )
                            )
                    )
            )
            .child(
                // Main content area - switches based on view_mode
                match self.view_mode {
                    ViewMode::Search => self.search_view.clone().into_any_element(),
                    ViewMode::Downloads => self.downloads_view.clone().into_any_element(),
                }
            )
    }
}
