use crate::colors::*;
use crate::components::card::MemoryCard;
use gpui::{Hsla, SharedString};
use rand::seq::SliceRandom;
use smallvec::{SmallVec, smallvec};
use std::sync::LazyLock;

/// Predefined icon and color configurations for the memory cards.
/// Each tuple contains the path to the icon and its associated color.
/// This static variable is lazily initialized to ensure efficient memory usage.
/// The icons are paired with vibrant colors to enhance the visual appeal of the memory game.
/// The use of `LazyLock` ensures that the configurations are only created when first accessed.
/// This approach optimizes performance, especially in scenarios where the memory game might not be used immediately.
static ICON_CONFIGS: LazyLock<SmallVec<[(&str, Hsla); 6]>> = LazyLock::new(|| {
  smallvec![
    ("icons/heart.svg", rose_400()),
    ("icons/star.svg", amber_400()),
    ("icons/sun.svg", yellow_400()),
    ("icons/moon.svg", purple_400()),
    ("icons/cloud.svg", sky_400()),
    ("icons/flower.svg", emerald_400()),
  ]
});

/// Creates and returns a shuffled collection of memory cards.
/// Each card is represented by a `MemoryCard` struct containing an icon and a color.
/// The function duplicates each icon-color pair to create matching pairs and then shuffles the entire collection.
/// This ensures that the memory game has a randomized layout each time it is initialized.
pub fn create_cards() -> SmallVec<[MemoryCard; 12]> {
  let mut cards = smallvec![];

  for (icon, color) in ICON_CONFIGS.iter() {
    cards.push(MemoryCard {
      icon: SharedString::from(*icon),
      color: *color,
      is_matched: false,
    });
    cards.push(MemoryCard {
      icon: SharedString::from(*icon),
      color: *color,
      is_matched: false,
    });
  }

  // Shuffle the cards to randomize their order
  cards.shuffle(&mut rand::rng());

  cards
}
