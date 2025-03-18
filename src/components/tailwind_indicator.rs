use gpui::prelude::*;
use gpui::{black, div, white, App, Window, Empty};

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
      return Empty{}.into_any_element();
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
