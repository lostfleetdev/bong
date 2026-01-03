use gpui::*;
use gpui_component::button::Button;

/// Search view component
pub struct SearchView {}

impl SearchView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for SearchView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // Search header with input
                div()
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Search")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9ca3af))
                            .child("Search for torrents, videos, and other content")
                    )
                    .child(
                        // Search input
                        div()
                            .flex()
                            .gap_2()
                            .mt_4()
                            .child(
                                // Search input placeholder
                                div()
                                    .flex_1()
                                    .px_4()
                                    .py_3()
                                    .bg(rgb(0x1f2937))
                                    .border_1()
                                    .border_color(rgb(0x374151))
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6b7280))
                                            .child("Enter search query...")
                                    )
                            )
                            .child(
                                Button::new("search-btn")
                                    .child("Search")
                                    .bg(rgb(0x3b82f6))
                                    .text_color(rgb(0xffffff))
                                    .px(px(24.0))
                                    .py(px(10.0))
                                    .rounded(px(8.0))
                                    .on_click(|_, _window, _cx| {
                                        println!("Search button clicked");
                                    })
                            )
                    )
            )
            .child(
                // Search results placeholder
                div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .text_color(rgb(0x6b7280))
                            .child("No search results yet")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x4b5563))
                            .child("Enter a search term to get started")
                    )
            )
    }
}
