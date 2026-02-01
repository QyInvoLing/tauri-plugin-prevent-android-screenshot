use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TauriPreventAndroidScreenshotExt;

#[command]
pub(crate) async fn disable_screen_capture<R: Runtime>(
    app: AppHandle<R>,
    payload: ScreenCaptureRequest,
) -> Result<()> {
    app.tauri_prevent_android_screenshot().disable_screen_capture(payload);
    Ok(())
}
#[command]
pub(crate) async fn enable_screen_capture<R: Runtime>(
    app: AppHandle<R>,
    payload: ScreenCaptureRequest,
) -> Result<()> {
    app.tauri_prevent_android_screenshot().enable_screen_capture(payload);
    Ok(())
}