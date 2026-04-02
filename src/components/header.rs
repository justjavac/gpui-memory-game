use crate::styled::StyledExt;
use crate::utils::CARD_PAIRS;
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
  status: &'static str,
  is_complete: bool,
}

impl Header {
  pub fn new(matches: u8, status: &'static str, is_complete: bool) -> Self {
    Self {
      matches,
      status,
      is_complete,
    }
  }
}

impl RenderOnce for Header {
  fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
    div()
      .id("header")
      .flex()
      .flex_col()
      .items_center()
      .text_center()
      .gap_y_3()
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
          .child(format!("Matches found: {} / {}", self.matches, CARD_PAIRS)),
      )
      .child(div().max_w_96().text_indigo_100().child(self.status))
      .when(self.is_complete, |this| this.text_color(crate::colors::amber_400()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn header_new_sets_matches() {
    let h = Header::new(2, "Keep going!", true);
    // cannot call render without GPUI runtime, but can verify stored value
    assert_eq!(h.matches, 2);
    assert_eq!(h.status, "Keep going!");
    assert!(h.is_complete);
  }
}
