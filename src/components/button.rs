use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{App, ClickEvent, Window, div};

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
