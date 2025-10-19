use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{App, ClickEvent, Window, div};

/// Creates a styled button element for restarting the game.
///
/// The button is designed with specific styles to enhance its appearance and user interaction.
/// It includes hover effects to provide visual feedback when the user interacts with it.
///
/// The button triggers the provided `on_click` callback when clicked, allowing for custom behavior such as restarting the game.
pub fn button(on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> impl IntoElement {
  div()
    .id("button-restart")
    .flex()
    .items_center()
    .justify_center()
    .flex_shrink_0()
    .bg_indigo_950()
    .border_1()
    .border_indigo_700()
    .text_indigo_200()
    .h_10()
    .px_8()
    .rounded_md()
    .shadow_sm()
    .hover(|this| this.bg_indigo_900().border_indigo_500().text_indigo_100())
    .on_click(on_click)
    .child("Start New Game")
}
