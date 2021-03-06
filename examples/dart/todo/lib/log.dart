import 'dart:io';
import 'package:logger/logger.dart';

final LOG_LEVEL = _levelFromString(Platform.environment['DART_LOG']);
final LOG_VERBOSE = LOG_LEVEL.index <= Level.debug.index;

final log = Logger(
  printer: LOG_VERBOSE
      ? PrettyPrinter(
          methodCount: 1,
          lineLength: 80,
          printEmojis: false,
          printTime: false,
        )
      : SimplePrinter(),
);

Level _levelFromString(String? level) {
  switch (level) {
    case "verbose":
      return Level.verbose;
    case "debug":
      return Level.debug;
    case "info":
      return Level.info;
    case "warning":
      return Level.warning;
    case "error":
      return Level.error;
    case "wtf":
      return Level.wtf;
    default:
      return Level.nothing;
  }
}
