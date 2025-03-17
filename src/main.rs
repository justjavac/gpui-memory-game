// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod colors;
mod components;
mod memory_game;
mod styled;
mod utils;

use anyhow::Error;
use assets::Assets;
use gpui::prelude::*;
use gpui::{actions, px, size};
use gpui::{App, Application, Bounds, KeyBinding, TitlebarOptions, WindowBounds, WindowKind, WindowOptions};
// use llrt_core::bytecode::BYTECODE_EXT;
// use llrt_core::compiler::compile_file;
// use llrt_core::modules::console::{self, LogLevel};
// use llrt_core::modules::path::name_extname;
// use llrt_core::utils::io::{is_supported_ext, DirectoryWalker, SUPPORTED_EXTENSIONS};
// use llrt_core::vm::Vm;
// use llrt_core::{async_with, runtime_client, CatchResultExt, VERSION};
use memory_game::MemoryGame;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

actions!(gpui_shadcn, [Quit, Open, CloseWindow]);

#[tokio::main]
async fn main() -> Result<(), Error> {
  #[cfg(all(not(debug_assertions), target_os = "windows"))]
  unsafe {
    use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
    let _ = AttachConsole(ATTACH_PARENT_PROCESS);
  }

  let app = Application::new().with_assets(Assets);

  app.run(|cx| {
    cx.bind_keys([KeyBinding::new("cmd-w", Quit, None)]);
    cx.on_window_closed(|cx| {
      if cx.windows().is_empty() {
        cx.quit();
      }
    })
    .detach();

    cx.on_action(quit);
    cx.activate(true);
    gpui_tokio::init(cx);

    let titlebar = TitlebarOptions {
      title: Some("Memory Match Game".into()),
      ..Default::default()
    };
    let min_size = size(px(800.0), px(850.0));
    let bounds = Bounds::centered(None, min_size, cx);
    let options = WindowOptions {
      window_bounds: Some(WindowBounds::Windowed(bounds)),
      window_min_size: Some(min_size),
      titlebar: Some(titlebar),
      kind: WindowKind::Normal,
      #[cfg(target_os = "linux")]
      window_background: gpui::WindowBackgroundAppearance::Transparent,
      #[cfg(target_os = "linux")]
      window_decorations: Some(gpui::WindowDecorations::Client),
      ..Default::default()
    };
    cx.open_window(options, |_window, cx| cx.new(|_cx| MemoryGame::new()))
      .expect("failed to open window");
  });

  Ok(())
}

fn quit(_: &Quit, cx: &mut App) {
  cx.quit();
}
