#[cfg(test)]
mod tests {
  #[test]
  fn test_breakpoint_label() {
    assert_eq!(super::breakpoint_label(100.0), "xs");
    assert_eq!(super::breakpoint_label(639.9), "xs");
    assert_eq!(super::breakpoint_label(640.0), "sm");
    assert_eq!(super::breakpoint_label(767.9), "sm");
    assert_eq!(super::breakpoint_label(768.0), "md");
    assert_eq!(super::breakpoint_label(1023.9), "md");
    assert_eq!(super::breakpoint_label(1024.0), "lg");
    assert_eq!(super::breakpoint_label(1279.9), "lg");
    assert_eq!(super::breakpoint_label(1280.0), "xl");
    assert_eq!(super::breakpoint_label(1535.9), "xl");
    assert_eq!(super::breakpoint_label(1536.0), "2xl");
    assert_eq!(super::breakpoint_label(2000.0), "2xl");
  }
}

fn breakpoint_label(width: f64) -> &'static str {
  const XS: &str = "xs";
  const SM: &str = "sm";
  const MD: &str = "md";
  const LG: &str = "lg";
  const XL: &str = "xl";
  const XXL: &str = "2xl";
  match width {
    w if w < 640.0 => XS,
    w if w < 768.0 => SM,
    w if w < 1024.0 => MD,
    w if w < 1280.0 => LG,
    w if w < 1536.0 => XL,
    _ => XXL,
  }
}

use gpui::prelude::*;
use gpui::{App, Empty, Window, black, div, white};

/// Debug-only indicator for Tailwind CSS breakpoints.
#[derive(IntoElement, Default, Debug)]
pub struct TailwindIndicator {}

impl TailwindIndicator {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }
}

impl RenderOnce for TailwindIndicator {
  fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
    if !cfg!(debug_assertions) {
      return Empty {}.into_any_element();
    }

    let width = window.bounds().size.width.to_f64();
    let text = breakpoint_label(width);

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
