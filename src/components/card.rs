use crate::colors::*;
use crate::styled::StyledExt;
use gpui::prelude::*;
use gpui::{
  Animation, AnimationExt, App, ClickEvent, Empty, Hsla, SharedString, Transformation, Window, div, size, svg,
};
use std::time::Duration;

/// Represents a memory card with an icon, color, and matched state.
///
/// This struct is used to define the properties of each card in the memory game.
/// It includes the card's icon, color, and whether it has been matched with another card.
#[derive(Clone, Debug, PartialEq)]
pub struct MemoryCard {
  pub icon: SharedString,
  pub color: Hsla,
  pub is_matched: bool,
}

type CardClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A UI component representing a memory card in the game.
///
/// The `Card` struct implements the `IntoElement` trait, allowing it to be rendered
/// as a UI element within the GPUI framework. It includes properties for the card's
/// identifier, the associated `MemoryCard` data, its flipped state, and a click handler.
///
/// The `render` method defines the visual appearance and behavior of the card,
/// including styles for different states (flipped, matched) and animations for flipping.
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
    let is_face_up = self.card.is_matched || self.is_flipped;

    div()
      .id(self.id)
      .relative()
      .flex()
      .items_center()
      .justify_center()
      .size_20()
      .border_1()
      .border_color(indigo_400().alpha(0.5))
      .bg(indigo_900().alpha(0.5))
      .rounded_xl()
      .shadow_lg()
      .cursor_pointer()
      .when(width >= 640.0, |this| this.size_24())
      .when(width >= 1100.0, |this| this.size_32())
      .when(is_face_up, |this| {
        this.bg(indigo_800().alpha(0.55)).border_color(indigo_500().alpha(0.5))
      })
      .when(self.card.is_matched, |this| {
        this
          .bg(self.card.color.alpha(0.16))
          .border_color(self.card.color.alpha(0.45))
          .shadow_xl()
      })
      .when(!self.card.is_matched && !self.is_flipped, |this| {
        this
          .bg_indigo_950()
          .border_indigo_800()
          .hover(|this| this.border_indigo_600().bg(indigo_900().alpha(0.82)))
          .on_click(self.on_click)
      })
      .child(if is_face_up {
        svg()
          .path(self.card.icon.clone())
          .text_color(self.card.color)
          .size_10()
          .when(width >= 640.0, |this| this.size_12())
          .when(self.card.is_matched, |this| this.shadow_lg())
          .with_animation(
            ("card-icon-up", self.id),
            Animation::new(Duration::from_millis(260)),
            |this, delta| {
              let scale = 0.85 + (delta * 0.15);
              this.with_transformation(Transformation::scale(size(scale, scale)))
            },
          )
          .into_any_element()
      } else {
        Empty {}.into_any_element()
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use gpui::SharedString;

  #[test]
  fn memory_card_clone_and_eq() {
    let c = MemoryCard {
      icon: SharedString::from("icons/heart.svg"),
      color: indigo_400(),
      is_matched: false,
    };
    let d = c.clone();
    assert_eq!(c, d);
  }

  #[test]
  fn card_new_sets_fields() {
    let mem = MemoryCard {
      icon: SharedString::from("icons/star.svg"),
      color: indigo_400(),
      is_matched: false,
    };
    let card = Card::new(3, mem.clone(), true, |_e, _w, _a| {});
    assert_eq!(card.id, 3);
    assert_eq!(card.card, mem);
    assert!(card.is_flipped);
  }
}
