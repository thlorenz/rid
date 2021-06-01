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

  bool handleCommand(String cmd) {
    switch (cmd) {
      case "a":
        stopWatch.startTimer();
        break;
      case "o":
        stopWatch.stopTimer();
        break;
      case "r":
        stopWatch.resetTimer();
        break;
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
      resetScreen();
    });
    stdin.listen((bytes) {
      final cmd = String.fromCharCode(bytes.first);
      final ok = handleCommand(cmd);
      if (!ok || cmd == "q") {
        exit(0);
      }
      resetScreen();
    });
  }
}
