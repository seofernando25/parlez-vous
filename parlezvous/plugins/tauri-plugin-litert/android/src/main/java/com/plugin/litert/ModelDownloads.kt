package com.plugin.litert

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkInfo
import androidx.work.WorkManager
import androidx.work.workDataOf
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.io.File

class ModelDownloads(
    private val activity: Activity,
    private val scope: CoroutineScope,
    private val emit: (String, JSObject) -> Unit
) {
    fun checkModelExists(invoke: Invoke) {
        val args = invoke.parseArgs(CheckModelArgs::class.java)
        scope.launch {
            try {
                val modelFile = File(activity.filesDir, args.modelPath)
                val exists = modelFile.exists() && modelFile.length() > 100_000_000L
                val infos = WorkManager.getInstance(activity.applicationContext)
                    .getWorkInfosForUniqueWork(WORK_NAME).get()
                val downloading = infos.firstOrNull()?.state in setOf(WorkInfo.State.RUNNING, WorkInfo.State.ENQUEUED)
                if (downloading) pollProgress()
                invoke.resolve(JSObject().apply {
                    put("exists", exists)
                    put("isDownloading", downloading)
                })
            } catch (error: Exception) {
                invoke.reject(error.message)
            }
        }
    }

    fun downloadModel(invoke: Invoke) {
        val args = invoke.parseArgs(DownloadModelArgs::class.java)
        requestNotificationPermission()
        scope.launch {
            try {
                val data = workDataOf(
                    "modelPath" to args.modelPath,
                    "downloadUrl" to MODEL_URL,
                    "token" to args.token
                )
                val request = OneTimeWorkRequestBuilder<ModelDownloadWorker>().setInputData(data).build()
                WorkManager.getInstance(activity.applicationContext)
                    .enqueueUniqueWork(WORK_NAME, ExistingWorkPolicy.KEEP, request)
                pollProgress()
                invoke.resolve(JSObject().apply { put("success", true) })
            } catch (error: Exception) {
                invoke.reject(error.message)
            }
        }
    }

    fun purgeModel(invoke: Invoke) {
        val args = invoke.parseArgs(PurgeModelArgs::class.java)
        try {
            val model = File(activity.filesDir, args.modelPath)
            val temp = File("${model.absolutePath}.tmp")
            if (model.exists()) model.delete()
            if (temp.exists()) temp.delete()
            invoke.resolve()
        } catch (error: Exception) {
            invoke.reject(error.message)
        }
    }

    private fun pollProgress() {
        scope.launch {
            val workManager = WorkManager.getInstance(activity.applicationContext)
            var completed = false
            while (!completed) {
                val info = workManager.getWorkInfosForUniqueWork(WORK_NAME).get().firstOrNull()
                if (info != null) {
                    val downloaded = info.progress.getLong("downloaded", -1L)
                    val total = info.progress.getLong("total", -1L)
                    if (downloaded >= 0) emit("download_progress", JSObject().apply {
                        put("downloaded", downloaded); put("total", total)
                    })
                    completed = info.state in setOf(WorkInfo.State.SUCCEEDED, WorkInfo.State.FAILED, WorkInfo.State.CANCELLED)
                    if (completed) emit("download_progress", JSObject().apply {
                        put("downloaded", total); put("total", total); put("state", info.state.name)
                    })
                }
                if (!completed) delay(500)
            }
        }
    }

    private fun requestNotificationPermission() {
        if (android.os.Build.VERSION.SDK_INT < android.os.Build.VERSION_CODES.TIRAMISU) return
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(activity, arrayOf(Manifest.permission.POST_NOTIFICATIONS), 1002)
        }
    }

    companion object {
        private const val WORK_NAME = "litert_download"
        private const val MODEL_URL = "https://huggingface.co/litert-community/gemma-4-E2B-it-litert-lm/resolve/main/gemma-4-E2B-it.litertlm"
    }
}
