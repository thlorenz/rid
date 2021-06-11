// This file defines identical util functions and vars as ./_rid_utils_flutter.dart

// We don't have the foundation constants in dart so we determine this in a
// somewhat hacky manner
bool _isDebugModeDart() {
  bool isDebug = false;
  assert(isDebug = true);
  return isDebug;
}

final _isDebugMode = _isDebugModeDart();
