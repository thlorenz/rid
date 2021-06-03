import 'dart:io';

import 'package:clock/generated/rid_generated.dart';
import 'package:clock/stop_watch.dart';

class KeyboardHandler {
  final StopWatch stopWatch;
  final Pointer<Store> store;

  KeyboardHandler(this.store, this.stopWatch);

  printStatus() {
    print('${store.debug(true)}');
  }

  void printCommands() {
    print("\nPlease select one of the below:\n");
    print("  a       -- to start clock");
    print("  o       -- to stop clock");
    print("  r       -- to reset clock");
    print("  q       -- to quit");
  }

  Future<bool> handleCommand(String cmd) async {
    switch (cmd) {
      case "a":
        await stopWatch.startTimer();
        break;
      case "o":
        await stopWatch.stopTimer();
        break;
      case "r":
        await stopWatch.resetTimer();
        break;
      case "q":
        return false;
      default:
        print("\nUnknown command '$cmd'\n");
        return false;
    }
    return true;
  }

  void resetScreen() {
    print("\x1B[2J\x1B[0;0H");
    printStatus();
    printCommands();
    stdout.write("\n> ");
  }

  void start() async {
    resetScreen();
    responseChannel.stream.where((res) => res.post == Post.Tick).listen((_) {
      rid_ffi.rid_store_lock();
      resetScreen();
      rid_ffi.rid_store_unlock();
    });
    stdin.listen((bytes) async {
      final cmd = String.fromCharCode(bytes.first);
      final ok = await handleCommand(cmd);
      if (!ok || cmd == "q") {
        exit(0);
      }
      resetScreen();
    });
  }
}
