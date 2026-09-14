package com.plugin.litert

import app.tauri.annotation.InvokeArg

@InvokeArg class InitModelArgs {
    var modelPath: String = ""
    var accelerator: String = "Auto"
    var maxTokens: Int = 5000
}
@InvokeArg class CheckModelArgs { var modelPath: String = "" }
@InvokeArg class DownloadModelArgs { var modelPath: String = ""; var token: String? = null }
@InvokeArg class PurgeModelArgs { var modelPath: String = "" }
@InvokeArg class GenerateChatArgs {
    var prompt: String = ""
    var reset: Boolean = false
    var audioBase64: String? = null
    var imageUri: String? = null
    var systemInstruction: String? = null
}
