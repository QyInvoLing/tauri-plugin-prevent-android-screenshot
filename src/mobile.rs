use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_tauri_prevent_android_screenshot);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<TauriPreventAndroidScreenshot<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.tauri-prevent-android-screenshot", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_tauri_prevent_android_screenshot)?;
  Ok(TauriPreventAndroidScreenshot(handle))
}

/// Access to the tauri-prevent-android-screenshot APIs.
pub struct TauriPreventAndroidScreenshot<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TauriPreventAndroidScreenshot<R> {
  pub fn disable_screen_capture(&self, payload: ScreenCaptureRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("disableScreenCapture", payload)
      .map_err(Into::into)
  }
  pub fn enable_screen_capture(&self, payload: ScreenCaptureRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("enableScreenCapture", payload)
      .map_err(Into::into)
  }
}
