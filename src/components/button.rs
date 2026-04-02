use crate::colors::{indigo_400, indigo_500, indigo_800, indigo_900};
use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{App, ClickEvent, FontWeight, Window, div, white};

/// Creates a styled button element for restarting the game.
///
/// The button is designed with specific styles to enhance its appearance and user interaction.
/// It includes hover effects to provide visual feedback when the user interacts with it.
///
/// The button triggers the provided `on_click` callback when clicked, allowing for custom behavior such as restarting the game.
pub fn button(
  label: &'static str,
  on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
  div()
    .id("button-restart")
    .flex()
    .items_center()
    .justify_center()
    .flex_shrink_0()
    .bg(indigo_900().alpha(0.95))
    .border_1()
    .border_color(indigo_500().alpha(0.4))
    .text_indigo_100()
    .font_weight(FontWeight::SEMIBOLD)
    .h_11()
    .px_10()
    .rounded_full()
    .shadow_lg()
    .hover(|this| {
      this
        .bg(indigo_800().alpha(0.95))
        .border_color(indigo_400().alpha(0.55))
        .text_color(white())
    })
    .on_click(on_click)
    .child(label)
}
