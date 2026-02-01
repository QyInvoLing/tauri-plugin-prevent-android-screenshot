import { invoke } from '@tauri-apps/api/core'

export async function disableScreenCapture(payload: {}): Promise<null> {
  return await invoke<{}>('plugin:tauri-prevent-android-screenshot|disable_screen_capture', {
    payload
  }).then(() => null);
}

export async function enableScreenCapture(payload: {}): Promise<null> {
  return await invoke<{}>('plugin:tauri-prevent-android-screenshot|enable_screen_capture', {
    payload
  }).then(() => null);
}
