#[cfg(test)]
mod integration_tests {
  use crate::memory_game::MemoryGame;
  use crate::utils::TOTAL_CARDS;

  #[test]
  fn memory_game_new_has_cards() {
    let mg = MemoryGame::new();
    assert_eq!(mg.cards_count(), TOTAL_CARDS);
    assert_eq!(mg.matches_count(), 0);
  }
}
