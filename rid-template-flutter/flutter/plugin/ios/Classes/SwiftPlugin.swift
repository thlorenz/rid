import Flutter
import UIKit

public class SwiftPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result(nil)
  }
  public static func dummyMethodToEnforceBundling() {
    // Prevent Swift Treeshake
    add(40, 2)
  }
}
