package com.plugin.litert

import android.app.Activity
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.util.Base64
import android.util.Log
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import com.google.ai.edge.litertlm.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.launch
import java.io.File
import java.io.FileOutputStream
import kotlin.math.roundToInt
import kotlin.math.sqrt

class LiteRtRuntime(
    private val activity: Activity,
    private val scope: CoroutineScope
) {
    private var engine: Engine? = null
    private var conversation: Conversation? = null
    private var conversationConfig: ConversationConfig? = null

    fun initModel(invoke: Invoke) {
        val args = invoke.parseArgs(InitModelArgs::class.java)
        scope.launch {
            try {
                val modelFile = File(activity.filesDir, args.modelPath)
                if (!modelFile.exists() || modelFile.length() < 100_000_000L) {
                    invoke.reject("Model file not found or corrupted. Please download it first.")
                    return@launch
                }
                closeRuntime()
                engine = initializeWithPolicy(modelFile.absolutePath, args)
                conversationConfig = defaultConversationConfig()
                conversation = engine!!.createConversation(conversationConfig!!)
                invoke.resolve(JSObject().apply { put("success", true) })
            } catch (error: LiteRtLmJniException) {
                Log.e(TAG, "JNI Exception initializing model", error)
                invoke.reject("Native failure: ${error.message}")
            } catch (error: Exception) {
                Log.e(TAG, "Failed to init model", error)
                invoke.reject("Failed to load model: ${error.message}")
            }
        }
    }

    fun generateChat(invoke: Invoke) {
        val args = invoke.parseArgs(GenerateChatArgs::class.java)
        if (engine == null) return invoke.reject("Model is not initialized.")
        scope.launch {
            try {
                if (args.reset || conversation == null) resetConversation(args.systemInstruction)
                val contents = buildContents(args)
                if (contents.isEmpty()) {
                    invoke.reject("No content provided to generate chat.")
                    return@launch
                }
                var response = ""
                conversation!!.sendMessageAsync(Contents.of(contents)).collect { token -> response += token }
                invoke.resolve(JSObject().apply { put("response", response) })
            } catch (error: LiteRtLmJniException) {
                Log.e(TAG, "JNI Exception during chat", error)
                invoke.reject("Native generation failed: ${error.message}")
            } catch (error: Exception) {
                Log.e(TAG, "Failed to generate chat", error)
                invoke.reject("Generation failed: ${error.message}")
            }
        }
    }

    fun closeModel(invoke: Invoke) {
        scope.launch {
            try {
                closeRuntime()
                invoke.resolve(JSObject().apply { put("success", true) })
            } catch (error: Exception) {
                invoke.reject("Failed to close model: ${error.message}")
            }
        }
    }

    private fun initializeWithPolicy(path: String, args: InitModelArgs): Engine {
        val explicit = backend(args.accelerator)
        if (explicit != null) return initialize(path, args.maxTokens, explicit).also {
            Log.i(TAG, "Model initialized with explicit ${args.accelerator} backend")
        }
        val attempts = listOf(
            "NPU" to Backend.NPU(nativeLibraryDir = activity.applicationInfo.nativeLibraryDir),
            "GPU" to Backend.GPU(),
            "CPU" to Backend.CPU()
        )
        var lastError: Exception? = null
        for ((name, candidate) in attempts) {
            try {
                return initialize(path, args.maxTokens, candidate).also { Log.i(TAG, "Model initialized with $name backend") }
            } catch (error: Exception) {
                lastError = error
                Log.w(TAG, "$name backend failed${if (name != "CPU") ", falling back" else ""}", error)
                closeEngineOnly()
            }
        }
        throw lastError ?: IllegalStateException("No LiteRT backend was available")
    }

    private fun initialize(path: String, maxTokens: Int, backend: Backend): Engine {
        val config = EngineConfig(
            modelPath = path,
            maxNumTokens = maxTokens,
            backend = backend,
            visionBackend = Backend.CPU(),
            audioBackend = Backend.CPU(),
            maxNumImages = 1,
            cacheDir = activity.cacheDir.path
        )
        return Engine(config).also { candidate ->
            engine = candidate
            candidate.initialize()
        }
    }

    private fun backend(name: String): Backend? = when (name) {
        "CPU" -> Backend.CPU()
        "GPU" -> Backend.GPU()
        "NPU" -> Backend.NPU(nativeLibraryDir = activity.applicationInfo.nativeLibraryDir)
        else -> null
    }

    private fun defaultConversationConfig(systemInstruction: String? = null) = ConversationConfig(
        systemInstruction = systemInstruction?.takeIf { it.isNotEmpty() }?.let(Contents::of),
        samplerConfig = SamplerConfig(topK = 64, topP = 0.95, temperature = 1.0)
    )

    private fun resetConversation(systemInstruction: String?) {
        conversation?.close()
        val config = ConversationConfig(
            systemInstruction = systemInstruction?.takeIf { it.isNotEmpty() }?.let(Contents::of),
            samplerConfig = conversationConfig?.samplerConfig ?: defaultConversationConfig().samplerConfig
        )
        conversationConfig = config
        conversation = engine!!.createConversation(config)
    }

    private fun buildContents(args: GenerateChatArgs): MutableList<Content> {
        val contents = mutableListOf<Content>()
        args.imageUri?.takeIf { it.isNotEmpty() }?.let { resizeImage(it)?.let(contents::add) }
        args.audioBase64?.takeIf { it.isNotEmpty() }?.let {
            contents.add(Content.AudioBytes(Base64.decode(it, Base64.DEFAULT)))
        }
        args.prompt.trim().takeIf { it.isNotEmpty() }?.let { contents.add(Content.Text(it)) }
        return contents
    }

    private fun resizeImage(path: String): Content.ImageFile? {
        val source = File(path)
        if (!source.exists()) return null
        val bitmap = BitmapFactory.decodeFile(source.absolutePath) ?: run {
            Log.w(TAG, "Failed to decode bitmap from path: $path")
            return null
        }
        val area = bitmap.width.toDouble() * bitmap.height.toDouble()
        val scale = if (area > TARGET_IMAGE_AREA) sqrt(TARGET_IMAGE_AREA / area) else 1.0
        val width = ((bitmap.width * scale).roundToInt() / 48 * 48).coerceAtLeast(48)
        val height = ((bitmap.height * scale).roundToInt() / 48 * 48).coerceAtLeast(48)
        val scaled = Bitmap.createScaledBitmap(bitmap, width, height, true)
        val temp = File(activity.cacheDir, "litert_scaled_temp.jpg")
        FileOutputStream(temp).use { output -> scaled.compress(Bitmap.CompressFormat.JPEG, 90, output) }
        if (scaled !== bitmap) bitmap.recycle()
        return Content.ImageFile(temp.absolutePath)
    }

    private fun closeEngineOnly() {
        try { engine?.close() } catch (error: Exception) { Log.w(TAG, "Error closing engine: ${error.message}") }
        engine = null
        System.gc()
    }

    private fun closeRuntime() {
        conversation?.close()
        conversation = null
        closeEngineOnly()
    }

    companion object {
        private const val TAG = "LitertPlugin"
        private const val TARGET_IMAGE_AREA = 645_120.0
    }
}
