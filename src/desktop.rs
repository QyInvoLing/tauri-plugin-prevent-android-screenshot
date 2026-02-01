use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<TauriPreventAndroidScreenshot<R>> {
  Ok(TauriPreventAndroidScreenshot(app.clone()))
}

/// Access to the tauri-prevent-android-screenshot APIs.
pub struct TauriPreventAndroidScreenshot<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TauriPreventAndroidScreenshot<R> {
  pub fn disable_screen_capture(&self, _payload: ScreenCaptureRequest) -> crate::Result<()> {
    Ok(())
  }
  pub fn enable_screen_capture(&self, _payload: ScreenCaptureRequest) -> crate::Result<()> {
    Ok(())
  }
}
