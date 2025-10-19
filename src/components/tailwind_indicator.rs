use gpui::prelude::*;
use gpui::{App, Empty, Window, black, div, white};

/// A debug-only indicator that shows the current Tailwind CSS breakpoint.
///
/// This component is only rendered in debug builds and helps developers
/// identify the current responsive breakpoint based on the window width.
/// The indicator displays one of the following labels:
/// "xs", "sm", "md", "lg", "xl", or "2xl".
///
/// The indicator is styled to be unobtrusive, positioned at the bottom-left corner
/// of the window with a semi-transparent background.
#[derive(IntoElement)]
pub struct TailwindIndicator {}

impl TailwindIndicator {
  pub fn new() -> Self {
    Self {}
  }
}

impl RenderOnce for TailwindIndicator {
  fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
    if !cfg!(debug_assertions) {
      return Empty {}.into_any_element();
    }

    let width = window.bounds().size.width.to_f64();
    let text = if width < 640.0 {
      "xs"
    } else if width < 768.0 {
      "sm"
    } else if width < 1024.0 {
      "md"
    } else if width < 1280.0 {
      "lg"
    } else if width < 1536.0 {
      "xl"
    } else {
      "2xl"
    };

    div()
      .id("tailwind-indicator")
      .flex()
      .absolute()
      .bottom_1()
      .left_1()
      .items_center()
      .justify_center()
      .rounded_full()
      .bg(black().grayscale().alpha(0.8))
      .p_2()
      .h_6()
      .w_6()
      .text_xs()
      .text_color(white())
      .child(text)
      .into_any_element()
  }
}
