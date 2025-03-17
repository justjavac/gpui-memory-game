use crate::colors::*;
use crate::components::card::MemoryCard;
use gpui::{Hsla, SharedString};
use rand::seq::SliceRandom;
use std::sync::LazyLock;

static ICON_CONFIGS: LazyLock<Vec<(&str, Hsla)>> = LazyLock::new(|| {
  vec![
    ("icons/heart.svg", rose_400()),
    ("icons/star.svg", amber_400()),
    ("icons/sun.svg", yellow_400()),
    ("icons/moon.svg", purple_400()),
    ("icons/cloud.svg", sky_400()),
    ("icons/flower.svg", emerald_400()),
  ]
});

pub fn create_cards() -> Vec<MemoryCard> {
  let mut cards = vec![];

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

  cards.shuffle(&mut rand::rng());

  cards
}
