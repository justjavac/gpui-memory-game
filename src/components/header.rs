use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{App, FontWeight, Window, div};

/// The header component displaying the game title and match count.
///
/// It uses styled extensions for consistent styling across the application.
/// The header shows the number of matches found out of the total possible matches.
/// This component is designed to be reusable and easily integrated into the main game layout.
/// It leverages the `IntoElement` trait for seamless rendering within the GPUI framework.
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
