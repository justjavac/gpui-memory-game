use crate::colors::*;
use crate::components::*;
use crate::styled::StyledExt;
use crate::utils::create_cards;
use gpui::prelude::*;
use gpui::{Context, Window, div, linear_color_stop, linear_gradient};
use smallvec::{SmallVec, smallvec};
use std::time::Duration;

pub struct MemoryGame {
  cards: SmallVec<[MemoryCard; 12]>,
  flipped_indexes: SmallVec<[usize; 12]>,
  matches: u8,
  is_checking: bool,
}

impl MemoryGame {
  pub fn new() -> Self {
    let cards = create_cards();
    Self {
      cards,
      flipped_indexes: smallvec![],
      matches: 0,
      is_checking: false,
    }
  }

  pub fn flip_card(&mut self, index: usize, cx: &mut Context<Self>) {
    // Prevent clicking if already checking
    if self.is_checking {
      return;
    }

    // Prevent clicking if card is already matched
    if self.cards[index].is_matched {
      return;
    }

    // Prevent clicking if card is already flipped
    // Prevent clicking if two cards are already flipped
    if self.flipped_indexes.contains(&index) || self.flipped_indexes.len() == 2 {
      return;
    }

    self.flipped_indexes.push(index);

    if self.flipped_indexes.len() < 2 {
      cx.notify();
      return;
    }

    // If we now have two cards flipped, check for a match
    self.is_checking = true;

    let first_idx = self.flipped_indexes[0];
    let second_idx = self.flipped_indexes[1];

    let first_card = self.cards[self.flipped_indexes[0]].clone();
    let second_card = self.cards[self.flipped_indexes[1]].clone();

    if first_card.icon == second_card.icon {
      cx.spawn(async move |this, cx| {
        cx.background_executor().timer(Duration::from_millis(500)).await;
        this
          .update(cx, |this, cx| {
            this.cards[first_idx].is_matched = true;
            this.cards[second_idx].is_matched = true;
            this.flipped_indexes.clear();
            this.matches += 1;
            this.is_checking = false;

            // Check for game completion
            if this.matches == this.cards.len() as u8 / 2 {
              println!("Game Over!");
            }
            cx.notify()
          })
          .ok()
      })
      .detach();
    } else {
      // If the cards don't match, flip them back over after a delay
      cx.spawn(async move |this, cx| {
        cx.background_executor().timer(Duration::from_millis(1000)).await;
        this
          .update(cx, |this, cx| {
            this.flipped_indexes.clear();
            this.is_checking = false;
            cx.notify()
          })
          .ok()
      })
      .detach();
    }
  }

  pub fn reset_game(&mut self, cx: &mut Context<Self>) {
    self.cards = create_cards();
    self.flipped_indexes.clear();
    self.matches = 0;
    self.is_checking = false;
    cx.notify();
  }
}

impl Render for MemoryGame {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let width = window.bounds().size.width.to_f64();

    div()
      .flex()
      .flex_col()
      .items_center()
      .min_h_full()
      .p_4()
      .gap_y_8()
      .bg(linear_gradient(
        0.5,
        linear_color_stop(purple_950(), 0.0),
        linear_color_stop(slate_950(), 1.0),
      ))
      .child(Header::new(self.matches))
      .child(
        div()
          .flex()
          .flex_wrap()
          .justify_center()
          .gap_4()
          .p_6()
          .w_128()
          .rounded_xl()
          .bg_indigo_950()
          .when(width > 768.0, |this| this.gap_6())
          .children(self.cards.iter().enumerate().map(|(index, card)| {
            Card::new(
              index,
              card.clone(),
              self.flipped_indexes.contains(&index),
              cx.listener(move |this, _, _, cx| {
                this.flip_card(index, cx);
              }),
            )
          })),
      )
      .child(button(cx.listener(|this, _: &gpui::ClickEvent, _, cx| {
        this.reset_game(cx);
      })))
      .child(TailwindIndicator::new())
  }
}
