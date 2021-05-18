import 'dart:io';

import 'package:clock/stop_watch.dart';

class KeyboardHandler {
  final StopWatch stopWatch;

  KeyboardHandler(this.stopWatch);

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
    printCommands();
    stdout.write("\n> ");
  }

  void start() async {
    resetScreen();
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
