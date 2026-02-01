const COMMANDS: &[&str] = &["disable_screen_capture", "enable_screen_capture"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
