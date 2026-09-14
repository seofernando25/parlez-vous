package com.plugin.litert

import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers

@TauriPlugin
class LitertPlugin(private val hostActivity: Activity): Plugin(hostActivity) {
    private val scope = CoroutineScope(Dispatchers.IO)
    private val downloads = ModelDownloads(hostActivity, scope) { event, payload -> trigger(event, payload) }
    private val runtime = LiteRtRuntime(hostActivity, scope)
    private val gallery = GalleryPicker(hostActivity)

    @Command fun checkModelExists(invoke: Invoke) = downloads.checkModelExists(invoke)
    @Command fun downloadModel(invoke: Invoke) = downloads.downloadModel(invoke)
    @Command fun purgeModel(invoke: Invoke) = downloads.purgeModel(invoke)
    @Command fun initModel(invoke: Invoke) = runtime.initModel(invoke)
    @Command fun generateChat(invoke: Invoke) = runtime.generateChat(invoke)
    @Command fun closeModel(invoke: Invoke) = runtime.closeModel(invoke)
    @Command fun pickGalleryImage(invoke: Invoke) = gallery.pick(invoke)

    companion object {
        init {
            try {
                System.loadLibrary("LiteRt")
                Log.i("LitertPlugin", "Loaded libLiteRt.so into global namespace")
            } catch (error: UnsatisfiedLinkError) {
                Log.w("LitertPlugin", "Could not pre-load libLiteRt.so", error)
            }
        }
    }
}
