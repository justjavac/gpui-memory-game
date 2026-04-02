use crate::colors::{amber_400, indigo_500, indigo_900};
use crate::styled::StyledExt;
use crate::utils::CARD_PAIRS;
use gpui::prelude::*;
use gpui::{App, FontWeight, Window, div, svg};

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
    let remaining = CARD_PAIRS.saturating_sub(self.matches);

    div()
      .id("header")
      .flex()
      .flex_col()
      .items_center()
      .text_center()
      .gap_y_4()
      .child(
        div()
          .px_4()
          .py_2()
          .rounded_full()
          .border_1()
          .border_indigo_700()
          .bg_indigo_950()
          .text_indigo_100()
          .child("Rust + GPUI Memory Match"),
      )
      .child(
        div()
          .text_3xl()
          .font_weight(FontWeight::BOLD)
          .text_indigo_300()
          .child("Memory Match Game"),
      )
      .child(
        div()
          .flex()
          .flex_wrap()
          .justify_center()
          .gap_3()
          .child(
            div()
              .px_4()
              .py_2()
              .rounded_full()
              .bg_indigo_950()
              .border_1()
              .border_indigo_800()
              .text_indigo_100()
              .child(format!("Matches {} / {}", self.matches, CARD_PAIRS)),
          )
          .child(
            div()
              .px_4()
              .py_2()
              .rounded_full()
              .bg(indigo_900().alpha(0.65))
              .border_1()
              .border_color(indigo_500().alpha(0.35))
              .text_indigo_200()
              .child(format!("Pairs left {}", remaining)),
          ),
      )
      .child(
        div()
          .px_5()
          .py_3()
          .rounded_xl()
          .bg(indigo_900().alpha(0.35))
          .border_1()
          .border_color(indigo_500().alpha(0.25))
          .text_indigo_100()
          .child(self.status),
      )
      .when(self.is_complete, |this| {
        this.child(
          div()
            .flex()
            .items_center()
            .justify_center()
            .gap_4()
            .px_5()
            .py_4()
            .rounded_xl()
            .bg(amber_400().alpha(0.12))
            .border_1()
            .border_color(amber_400().alpha(0.35))
            .child(svg().path("icons/trophy.svg").size_8().text_color(amber_400()))
            .child(
              div()
                .flex()
                .flex_col()
                .items_start()
                .text_left()
                .gap_y_1()
                .child(
                  div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(amber_400())
                    .child("Perfect memory run"),
                )
                .child(
                  div()
                    .text_indigo_100()
                    .child("All pairs cleared. Hit Play Again for a fresh shuffle."),
                ),
            ),
        )
      })
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
