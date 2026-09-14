import SwiftRs
import Tauri
import UIKit
import WebKit

/// Minimal iOS bridge. LiteRT-LM is Android-only for now; these commands keep
/// the shared Tauri plugin surface available so the iOS app can build and use
/// server-backed AI without pretending on-device inference exists.
final class LitertPlugin: Plugin {
  @objc func initModel(_ invoke: Invoke) throws { invoke.resolve(["success": false]) }
  @objc func checkModelExists(_ invoke: Invoke) throws { invoke.resolve(["exists": false]) }
  @objc func downloadModel(_ invoke: Invoke) throws { invoke.resolve(["success": false]) }
  @objc func purgeModel(_ invoke: Invoke) throws { invoke.resolve() }
  @objc func generateChat(_ invoke: Invoke) throws {
    invoke.resolve(["response": "On-device LiteRT is not available on iOS."])
  }
  @objc func closeModel(_ invoke: Invoke) throws { invoke.resolve(["success": true]) }
  @objc func pickGalleryImage(_ invoke: Invoke) throws { invoke.resolve(["path": ""]) }
}

@_cdecl("init_plugin_litert")
func initPlugin() -> Plugin { LitertPlugin() }
