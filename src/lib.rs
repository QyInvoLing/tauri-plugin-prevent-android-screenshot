use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TauriPreventAndroidScreenshot;
#[cfg(mobile)]
use mobile::TauriPreventAndroidScreenshot;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the tauri-prevent-android-screenshot APIs.
pub trait TauriPreventAndroidScreenshotExt<R: Runtime> {
  fn tauri_prevent_android_screenshot(&self) -> &TauriPreventAndroidScreenshot<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TauriPreventAndroidScreenshotExt<R> for T {
  fn tauri_prevent_android_screenshot(&self) -> &TauriPreventAndroidScreenshot<R> {
    self.state::<TauriPreventAndroidScreenshot<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("tauri-prevent-android-screenshot")
    .invoke_handler(tauri::generate_handler![
      commands::disable_screen_capture,
      commands::enable_screen_capture
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let tauri_prevent_android_screenshot = mobile::init(app, api)?;
      #[cfg(desktop)]
      let tauri_prevent_android_screenshot = desktop::init(app, api)?;
      app.manage(tauri_prevent_android_screenshot);
      Ok(())
    })
    .build()
}
