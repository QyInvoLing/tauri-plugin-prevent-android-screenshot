import { invoke } from '@tauri-apps/api/core'

export async function disable_screen_capture(): Promise<null> {
  return await invoke<{}>('plugin:tauri-prevent-android-screenshot|disable_screen_capture', {
    payload: {
    },
  }).then(() => null);
}

export async function enable_screen_capture(): Promise<null> {
  return await invoke<{}>('plugin:tauri-prevent-android-screenshot|enable_screen_capture', {
    payload: {
    },
  }).then(() => null);
}
