import SwiftRs
import Tauri
import UIKit
import WebKit

/// Minimal iOS bridge. Supertonic remains Android-only for on-device speech;
/// iOS uses the configured OpenAI-compatible TTS server through the shared Rust
/// media pipeline.
final class SupertonicPlugin: Plugin {
  @objc func isSupertonicReady(_ invoke: Invoke) throws {
    invoke.resolve(["exists": false, "isDownloading": false])
  }
  @objc func downloadSupertonicModels(_ invoke: Invoke) throws { invoke.resolve(["success": false]) }
  @objc func purgeSupertonicModels(_ invoke: Invoke) throws { invoke.resolve() }
}

@_cdecl("init_plugin_supertonic")
func initPlugin() -> Plugin { SupertonicPlugin() }
