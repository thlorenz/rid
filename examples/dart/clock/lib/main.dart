import 'dart:async';
import 'dart:isolate';
import 'package:clock/generated/rid_generated.dart';
import 'package:clock/stream_channel.dart';

///
/// App
///
class Test {
  Test() {
    StreamChannel.instance<Topic>().stream.listen(onResponse);
  }

  void onResponse(Response res) {
    print('loaded: $res on ${Isolate.current.debugName} thread');
  }

  void loadPage(String url) {
    // defined in package:ffi/ffi.dart
    final urlPtr = url.toNativeInt8();
    final res = rid_ffi.load_page(urlPtr);
    if (res != 1) {
      print("ERROR when initializing page load");
    }
  }
}

Timer wait() {
  return new Timer.periodic(const Duration(seconds: 1), (Timer timer) {
    print('.');
  });
}

Future<void> main() async {
  final test = Test();
  final _ = wait();
  test.loadPage("https://github.com");

  print("Waiting for the response, but life goes on\n");
}
