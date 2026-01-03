use gpui::*;

/// Downloads view component
pub struct DownloadsView {}

impl DownloadsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for DownloadsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // Downloads header with stats
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
                            .child("Downloads")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9ca3af))
                            .child("Manage your active and completed downloads")
                    )
                    .child(
                        // Stats bar
                        div()
                            .flex()
                            .gap_6()
                            .mt_4()
                            .p_4()
                            .bg(rgb(0x1f2937))
                            .rounded(px(10.0))
                            .border_1()
                            .border_color(rgb(0x374151))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9ca3af))
                                            .child("Active")
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("0")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9ca3af))
                                            .child("Completed")
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("0")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9ca3af))
                                            .child("Speed")
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("0 KB/s")
                                    )
                            )
                    )
            )
            .child(
                // Downloads list
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
                            .child("No active downloads")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x4b5563))
                            .child("Add a download to get started")
                    )
            )
    }
}
