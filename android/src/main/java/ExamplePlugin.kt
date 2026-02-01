package com.plugin.tauri-prevent-android-screenshot

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.view.WindowManager


@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun disableScreenCapture() {
        activity.window.setFlags(
            WindowManager.LayoutParams.FLAG_SECURE,
            WindowManager.LayoutParams.FLAG_SECURE
        )
    }

    @Command
    fun enableScreenCapture() {
        activity.window.clearFlags(
            WindowManager.LayoutParams.FLAG_SECURE
        )
    }
}
