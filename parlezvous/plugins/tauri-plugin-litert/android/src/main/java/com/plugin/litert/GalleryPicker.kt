package com.plugin.litert

import android.app.Activity
import android.content.Intent
import android.net.Uri
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import androidx.activity.ComponentActivity
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import java.io.File
import java.io.FileOutputStream

class GalleryPicker(private val activity: Activity) {
    private var pending: Invoke? = null
    private val launcher: ActivityResultLauncher<Intent> =
        (activity as ComponentActivity).activityResultRegistry.register(
            "litert_gallery_picker",
            ActivityResultContracts.StartActivityForResult()
        ) { result -> handleResult(result.resultCode, result.data?.data) }

    fun pick(invoke: Invoke) {
        pending = invoke
        launcher.launch(Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "image/*"
            addCategory(Intent.CATEGORY_OPENABLE)
        })
    }

    private fun handleResult(resultCode: Int, uri: Uri?) {
        if (resultCode != Activity.RESULT_OK || uri == null) {
            pending?.reject("Image selection cancelled by user")
            pending = null
            return
        }
        try {
            val cacheFile = File(activity.cacheDir, "vision_target_${System.currentTimeMillis()}.jpg")
            activity.contentResolver.openInputStream(uri)?.use { input ->
                FileOutputStream(cacheFile).use { output -> input.copyTo(output) }
            }
            pending?.resolve(JSObject().apply { put("path", cacheFile.absolutePath) })
        } catch (error: Exception) {
            pending?.reject("Failed to process image: ${error.message}")
        } finally {
            pending = null
        }
    }
}
