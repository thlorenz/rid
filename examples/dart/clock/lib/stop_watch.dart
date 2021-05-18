import 'package:clock/generated/rid_generated.dart';
import 'package:clock/response_channel.dart';

class StopWatch {
  final chan = ResponseChannel.instance;

  final Pointer<Model> model;

  StopWatch(this.model);

  void startTimer() async {
    final reqID = rid_ffi.msgStart(model);
    final res = await chan.response(reqID);
    print('$res');
  }

  void stopTimer() async {
    final reqID = rid_ffi.msgStop(model);
    final res = await chan.response(reqID);
    print('$res');
  }

  void resetTimer() async {
    final reqID = rid_ffi.msgReset(model);
    final res = await chan.response(reqID);
    print('$res');
  }
}
