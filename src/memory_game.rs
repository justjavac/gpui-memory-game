use crate::colors::*;
use crate::components::*;
use crate::styled::StyledExt;
use crate::utils::{CARD_PAIRS, TOTAL_CARDS, create_cards};
use gpui::prelude::*;
use gpui::{Context, Window, div, linear_color_stop, linear_gradient};
use smallvec::{SmallVec, smallvec};
use std::time::Duration;

const MATCH_DELAY: Duration = Duration::from_millis(500);
const MISMATCH_DELAY: Duration = Duration::from_millis(1000);

/// The main memory game component managing game state and rendering.
///
/// It handles user interactions, game logic, and the overall layout of the memory game.
/// The component maintains the state of the cards, flipped cards, and match count.
/// It provides methods to flip cards, check for matches, and reset the game.
/// The rendering logic uses GPUI's element system to create a responsive and visually appealing game interface.
pub struct MemoryGame {
  /// The collection of memory cards in the game.
  cards: SmallVec<[MemoryCard; TOTAL_CARDS]>,
  /// Indexes of currently flipped cards.
  flipped_indexes: SmallVec<[usize; 2]>,
  /// The number of matches found so far.
  matches: u8,
  /// Flag indicating if the game is currently checking for a match.
  is_checking: bool,
  /// Monotonic token used to ignore stale async resolutions after a reset.
  generation: u64,
}

impl MemoryGame {
  pub fn new() -> Self {
    let cards = create_cards();
    Self {
      cards,
      flipped_indexes: smallvec![],
      matches: 0,
      is_checking: false,
      generation: 0,
    }
  }

  #[cfg(test)]
  pub(crate) fn cards_count(&self) -> usize {
    self.cards.len()
  }

  #[cfg(test)]
  pub(crate) fn matches_count(&self) -> u8 {
    self.matches
  }

  fn resolve_match(&mut self, first_idx: usize, second_idx: usize, generation: u64) -> bool {
    if self.generation != generation {
      return false;
    }

    self.cards[first_idx].is_matched = true;
    self.cards[second_idx].is_matched = true;
    self.flipped_indexes.clear();
    self.matches += 1;
    self.is_checking = false;
    true
  }

  fn resolve_mismatch(&mut self, generation: u64) -> bool {
    if self.generation != generation {
      return false;
    }

    self.flipped_indexes.clear();
    self.is_checking = false;
    true
  }

  fn is_complete(&self) -> bool {
    self.matches == CARD_PAIRS
  }

  fn is_card_flipped(&self, index: usize) -> bool {
    self.flipped_indexes.contains(&index)
  }

  fn status_message(&self) -> &'static str {
    if self.is_complete() {
      "You found every pair. Nice run!"
    } else if self.is_checking {
      "Checking the cards..."
    } else {
      "Flip two cards to find a match."
    }
  }

  fn reset_button_label(&self) -> &'static str {
    if self.is_complete() {
      "Play Again"
    } else {
      "Start New Game"
    }
  }

  /// Flips a card at the given index and checks for matches if two cards are flipped.
  pub fn flip_card(&mut self, index: usize, cx: &mut Context<Self>) {
    if index >= self.cards.len() {
      return;
    }

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
    let generation = self.generation;

    let is_match = self.cards[first_idx].icon == self.cards[second_idx].icon;

    if is_match {
      cx.spawn(async move |this, cx| {
        cx.background_executor().timer(MATCH_DELAY).await;
        this
          .update(cx, |this, cx| {
            if !this.resolve_match(first_idx, second_idx, generation) {
              return;
            }

            cx.notify()
          })
          .ok()
      })
      .detach();
    } else {
      // If the cards don't match, flip them back over after a delay
      cx.spawn(async move |this, cx| {
        cx.background_executor().timer(MISMATCH_DELAY).await;
        this
          .update(cx, |this, cx| {
            if !this.resolve_mismatch(generation) {
              return;
            }
            cx.notify()
          })
          .ok()
      })
      .detach();
    }
  }

  /// Resets the game by re-creating the cards and clearing the game state.
  pub fn reset_game(&mut self, cx: &mut Context<Self>) {
    self.generation = self.generation.wrapping_add(1);
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
      .id("memory-game")
      .flex()
      .flex_col()
      .items_center()
      .size_full()
      .p_4()
      .gap_y_8()
      .bg(linear_gradient(
        0.5,
        linear_color_stop(purple_950(), 0.0),
        linear_color_stop(slate_950(), 1.0),
      ))
      .overflow_scroll()
      .child(Header::new(self.matches, self.status_message()))
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
              self.is_card_flipped(index),
              cx.listener(move |this, _, _, cx| {
                this.flip_card(index, cx);
              }),
            )
          })),
      )
      .child(button(
        self.reset_button_label(),
        cx.listener(|this, _: &gpui::ClickEvent, _, cx| {
          this.reset_game(cx);
        }),
      ))
      .child(TailwindIndicator::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_initializes() {
    let mg = MemoryGame::new();
    assert_eq!(mg.cards.len(), TOTAL_CARDS);
    assert_eq!(mg.matches, 0);
    assert!(mg.flipped_indexes.is_empty());
    assert!(!mg.is_checking);
  }

  #[test]
  fn stale_match_resolution_is_ignored() {
    let mut mg = MemoryGame::new();
    mg.flipped_indexes.extend([0, 1]);
    mg.is_checking = true;
    let stale_generation = mg.generation;
    mg.generation += 1;

    assert!(!mg.resolve_match(0, 1, stale_generation));
    assert!(!mg.cards[0].is_matched);
    assert!(!mg.cards[1].is_matched);
    assert_eq!(mg.matches, 0);
    assert_eq!(mg.flipped_indexes.as_slice(), &[0, 1]);
    assert!(mg.is_checking);
  }

  #[test]
  fn current_match_resolution_updates_state() {
    let mut mg = MemoryGame::new();
    mg.flipped_indexes.extend([0, 1]);
    mg.is_checking = true;
    let generation = mg.generation;

    assert!(mg.resolve_match(0, 1, generation));
    assert!(mg.cards[0].is_matched);
    assert!(mg.cards[1].is_matched);
    assert_eq!(mg.matches, 1);
    assert!(mg.flipped_indexes.is_empty());
    assert!(!mg.is_checking);
  }

  #[test]
  fn status_message_reflects_game_state() {
    let mut mg = MemoryGame::new();
    assert_eq!(mg.status_message(), "Flip two cards to find a match.");

    mg.is_checking = true;
    assert_eq!(mg.status_message(), "Checking the cards...");

    mg.is_checking = false;
    mg.matches = CARD_PAIRS;
    assert_eq!(mg.status_message(), "You found every pair. Nice run!");
    assert_eq!(mg.reset_button_label(), "Play Again");
  }
}
