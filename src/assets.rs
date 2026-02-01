use anyhow::anyhow;
use gpui::AssetSource;
use rust_embed::RustEmbed;

/// Asset manager for embedded assets such as icons.
///
/// The `Assets` struct uses the `RustEmbed` derive macro to include all files
/// located in the `./assets` directory, specifically targeting the `icons` subdirectory.
/// Files matching the pattern `*.DS_Store` are excluded to avoid unnecessary clutter.
///
/// This setup allows for efficient access to embedded assets at runtime,
/// facilitating their use within the application without relying on external file dependencies.
///
/// The `AssetSource` trait implementation provides methods to load and list assets,
/// enabling seamless integration with the GPUI framework.
#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
  fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
    if path.is_empty() {
      return Ok(None);
    }

    Self::get(path)
      .map(|f| Some(f.data))
      .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
  }

  fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
    Ok(
      Self::iter()
        .filter_map(|p| if p.starts_with(path) { Some(p.into()) } else { None })
        .collect(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn load_empty_path_returns_none() {
    let a = Assets;
    let res = a.load("");
    assert!(res.unwrap().is_none());
  }

  #[test]
  fn list_returns_ok() {
    let a = Assets;
    let list = a.list("icons/");
    assert!(list.is_ok());
  }
}
