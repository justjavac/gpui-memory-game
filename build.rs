use std::env;

fn main() {
  // Skip building from docs.rs.
  if env::var_os("DOCS_RS").is_some() {
    return;
  }

  if cfg!(target_os = "macos") {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

    // Weakly link ReplayKit to ensure Zed can be used on macOS 10.15+.
    println!("cargo:rustc-link-arg=-Wl,-weak_framework,ReplayKit");

    // Seems to be required to enable Swift concurrency
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    // Register exported Objective-C selectors, protocols, etc
    println!("cargo:rustc-link-arg=-Wl,-ObjC");
  }

  #[cfg(target_os = "windows")]
  {
    #[cfg(target_env = "msvc")]
    {
      // todo(windows): This is to avoid stack overflow. Remove it when solved.
      // https://github.com/zed-industries/zed/blob/c85a3cc1/crates/zed/build.rs#L51
      println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
    }

    let icon = std::path::Path::new("assets/icon.ico");
    println!("cargo:rerun-if-changed={}", icon.display());

    let mut res = winres::WindowsResource::new();
    res.set_icon(icon.to_str().unwrap());
    res.set_language(winapi::um::winnt::MAKELANGID(
      winapi::um::winnt::LANG_ENGLISH,
      winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    res.compile().unwrap();
  }
}
