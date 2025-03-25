use crate::colors::*;
use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{Animation, AnimationExt, App, ClickEvent, Hsla, SharedString, Transformation, Window, div, size, svg};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryCard {
  pub icon: SharedString,
  pub color: Hsla,
  pub is_matched: bool,
}

type CardClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Card {
  pub(crate) id: usize,
  pub(crate) card: MemoryCard,
  pub(crate) is_flipped: bool,
  pub(crate) on_click: CardClickHandler,
}

impl Card {
  pub fn new(
    id: usize,
    card: MemoryCard,
    is_flipped: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
  ) -> Self {
    Self {
      id,
      card,
      is_flipped,
      on_click: Box::new(on_click),
    }
  }
}

impl RenderOnce for Card {
  fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
    let width = window.bounds().size.width.to_f64();

    div()
      .id(self.id)
      .relative()
      .flex()
      .items_center()
      .justify_center()
      .size_24()
      .border_1()
      .border_color(indigo_400().alpha(0.5))
      .bg(indigo_900().alpha(0.5))
      .rounded_lg()
      .shadow_lg()
      .cursor_pointer()
      .when(width > 768.0, |this| this.size_32())
      .when(self.card.is_matched || self.is_flipped, |this| {
        this.bg(indigo_800().alpha(0.5)).border_color(indigo_500().alpha(0.5))
      })
      .when(!self.card.is_matched && !self.is_flipped, |this| {
        this
          .bg_indigo_950()
          .border_indigo_800()
          .hover(|this| this.border_indigo_600().bg(indigo_900().alpha(0.8)))
          .on_click(self.on_click)
      })
      .child(
        svg()
          .path(self.card.icon.clone())
          .text_color(self.card.color)
          .when(self.card.is_matched, |this| this.shadow_lg().size_12())
          .when(self.is_flipped, |this| this.size_12())
          .with_animation(
            "card-flip",
            Animation::new(Duration::from_millis(300)),
            |this, delta| this.with_transformation(Transformation::scale(size(delta, delta))),
          ),
      )
  }
}
