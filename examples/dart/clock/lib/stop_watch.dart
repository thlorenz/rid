import 'package:clock/generated/rid_generated.dart';

class StopWatch {
  final Pointer<Store> store;

  StopWatch(this.store);

  void startTimer() async {
    final res = await store.msgStart();
    print('$res');
  }

  void stopTimer() async {
    final res = await store.msgStop();
    print('$res');
  }

  void resetTimer() async {
    final res = await store.msgReset();
    print('$res');
  }
}
