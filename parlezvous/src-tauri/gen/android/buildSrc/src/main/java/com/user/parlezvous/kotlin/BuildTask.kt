import java.io.File
import org.apache.tools.ant.taskdefs.condition.Os
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.logging.LogLevel
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction

open class BuildTask : DefaultTask() {
    @Input
    var rootDirRel: String? = null
    @Input
    var target: String? = null
    @Input
    var release: Boolean? = null

    @TaskAction
    fun assemble() {
        val executable = """bun""";
        try {
            runTauriCli(executable)
        } catch (e: Exception) {
            if (Os.isFamily(Os.FAMILY_WINDOWS)) {
                // Try different Windows-specific extensions
                val fallbacks = listOf(
                    "$executable.exe",
                    "$executable.cmd",
                    "$executable.bat",
                )
                
                var lastException: Exception = e
                for (fallback in fallbacks) {
                    try {
                        runTauriCli(fallback)
                        return
                    } catch (fallbackException: Exception) {
                        lastException = fallbackException
                    }
                }
                throw lastException
            } else {
                throw e;
            }
        }
    }

    fun runTauriCli(executable: String) {
        val rootDirRel = rootDirRel ?: throw GradleException("rootDirRel cannot be null")
        val target = target ?: throw GradleException("target cannot be null")
        val release = release ?: throw GradleException("release cannot be null")
        val args = listOf("tauri", "android", "android-studio-script");

        val rustRoot = File(project.projectDir, rootDirRel)
        project.exec {
            workingDir(rustRoot)
            executable(executable)
            args(args)
            if (project.logger.isEnabled(LogLevel.DEBUG)) {
                args("-vv")
            } else if (project.logger.isEnabled(LogLevel.INFO)) {
                args("-v")
            }
            if (release) {
                args("--release")
            }
            args(listOf("--target", target))
        }.assertNormalExitValue()

        if (!release && System.getenv("PARLEZVOUS_KEEP_ANDROID_SYMBOLS") != "1") {
            stripRustDebugSymbols(rustRoot, target)
        }
    }

    private fun stripRustDebugSymbols(rustRoot: File, target: String) {
        val rustTriple = when (target) {
            "aarch64" -> "aarch64-linux-android"
            "armv7" -> "armv7-linux-androideabi"
            "i686" -> "i686-linux-android"
            "x86_64" -> "x86_64-linux-android"
            else -> target
        }
        val library = File(rustRoot, "target/$rustTriple/debug/libparlezvous_lib.so")
        if (!library.exists()) {
            project.logger.warn("Rust Android library not found for stripping: ${library.absolutePath}")
            return
        }

        val ndkRoot = sequenceOf(
            System.getenv("NDK_HOME"),
            System.getenv("ANDROID_NDK_HOME")
        ).filterNotNull().map(::File).firstOrNull { it.isDirectory } ?: run {
            val androidHome = System.getenv("ANDROID_HOME") ?: System.getenv("ANDROID_SDK_ROOT")
            androidHome?.let { File(it, "ndk") }
                ?.listFiles()
                ?.filter { it.isDirectory }
                ?.maxByOrNull { it.name }
        }

        if (ndkRoot == null) {
            project.logger.warn("NDK not found; Android Rust debug symbols will be packaged.")
            return
        }

        val stripTool = File(ndkRoot, "toolchains/llvm/prebuilt")
            .walkTopDown()
            .firstOrNull {
                it.isFile && it.parentFile?.name == "bin" &&
                    (it.name == "llvm-strip" || it.name == "llvm-strip.exe")
            }

        if (stripTool == null) {
            project.logger.warn("llvm-strip not found under ${ndkRoot.absolutePath}; Android Rust debug symbols will be packaged.")
            return
        }

        project.exec {
            executable(stripTool.absolutePath)
            args("--strip-debug", library.absolutePath)
        }.assertNormalExitValue()
        project.logger.lifecycle("Stripped Rust debug symbols for Android: ${library.name}")
    }
}