import 'package:test/test.dart';
import '../lib/generated/rid_api.dart';

void main() {
  test('messaging: isolate', () async {
    rid_ffi.rid_export_send_log_warn_message(0);
    RidMessage msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.LogWarn, message: "Warn 0 from Rust" }');
  });
  test('messaging: log warn/info/debug which have no details', () async {
    rid_ffi.rid_export_send_log_warn_message(0);
    RidMessage msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.LogWarn, message: "Warn 0 from Rust" }');

    rid_ffi.rid_export_send_log_info_message(1);
    msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.LogInfo, message: "Info 1 from Rust" }');

    rid_ffi.rid_export_send_log_debug_message(2);
    msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.LogDebug, message: "Debug 2 from Rust" }');
  });

  test('messaging: error/severe with details', () async {
    rid_ffi.rid_export_send_error_message_with_details(1);
    RidMessage msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.Error, message: "Error 1 from Rust", details: "Some Error Details" }');

    rid_ffi.rid_export_send_severe_error_message_with_details(2);
    msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.Severe, message: "Severe Error 2 from Rust", details: "Some Severe Error Details" }');
  });

  test('messaging: error/severe without details', () async {
    rid_ffi.rid_export_send_error_message_without_details(1);
    RidMessage msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.Error, message: "Error 1 from Rust" }');

    rid_ffi.rid_export_send_severe_error_message_without_details(2);
    msg = await rid.messageChannel.stream.first;
    expect(msg.toString(),
        'RidMessage{ type: RidMessageType.Severe, message: "Severe Error 2 from Rust" }');
  });
}
