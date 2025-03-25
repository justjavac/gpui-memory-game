use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{App, FontWeight, Window, div};

#[derive(IntoElement)]
pub struct Header {
  matches: u8,
}

impl Header {
  pub fn new(matches: u8) -> Self {
    Self { matches }
  }
}

impl RenderOnce for Header {
  fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
    div()
      .id("header")
      .text_center()
      .gap_y_4()
      .child(
        div()
          .text_3xl()
          .font_weight(FontWeight::BOLD)
          .text_indigo_300()
          .child("Memory Match Game"),
      )
      .child(
        div()
          .text_indigo_200()
          .child(format!("Matches found: {} of {}", self.matches, 6)),
      )
  }
}
