import 'dart:async';
import 'dart:isolate';
import 'package:clock/event_channel.dart';
import 'package:clock/generated/rid_generated.dart';

///
/// App
///
class Test {
  final EventChannel<String> eventChannel;
  Test(this.eventChannel) {
    this.eventChannel.subscribe(onLoadedPage);
  }

  void loadPage(String url) {
    // defined in package:ffi/ffi.dart
    final urlPtr = url.toNativeInt8();
    final res = rid_ffi.load_page(eventChannel.nativePort, urlPtr);
    if (res != 1) {
      print("ERROR when initializing page load");
    }
  }

  void onLoadedPage(String res) {
    print('loaded: $res on ${Isolate.current.debugName} thread');
  }
}

Timer wait() {
  return new Timer.periodic(const Duration(seconds: 1), (Timer timer) {
    print('.');
  });
}

Future<void> main() async {
  EventChannel.setup();
  final eventChannel = EventChannel<String>();
  final test = Test(eventChannel);
  final _ = wait();
  test.loadPage("https://github.com");

  print("Waiting for the response, but life goes on\n");
}
