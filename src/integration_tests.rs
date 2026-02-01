#[cfg(test)]
mod gpui_integration {
  use crate::memory_game::MemoryGame;
  use gpui::prelude::*;

  #[gpui::test]
  async fn memory_game_startup(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    // Create a window view with the MemoryGame component to ensure startup doesn't panic
    let (_view, mut vcx) = cx.add_window_view(|_window, _cx| MemoryGame::new());

    // Run until the scheduler is parked to allow background work to complete
    vcx.run_until_parked();

    // If we reach here, the view started successfully
    assert!(true);
  }
}
