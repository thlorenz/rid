import 'package:clock/generated/rid_generated.dart';
import 'package:clock/response_channel.dart';

class StopWatch {
  final chan = ResponseChannel.instance;

  void startTimer() async {
    final reqID = rid_ffi.msgStart();
    final res = await chan.response(reqID);
    print('$res');
  }

  void stopTimer() async {
    final reqID = rid_ffi.msgStop();
    final res = await chan.response(reqID);
    print('$res');
  }

  void resetTimer() async {
    final reqID = rid_ffi.msgReset();
    final res = await chan.response(reqID);
    print('$res');
  }
}
