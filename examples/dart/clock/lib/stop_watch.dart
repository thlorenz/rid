import 'package:clock/generated/rid_generated.dart';

class StopWatch {
  final Store store;

  StopWatch(this.store);

  Future<void> startTimer() async {
    final res = await store.msgStart();
    print('$res');
  }

  Future<void> stopTimer() async {
    final res = await store.msgStop();
    print('$res');
  }

  Future<void> resetTimer() async {
    final res = await store.msgReset();
    print('$res');
  }
}
