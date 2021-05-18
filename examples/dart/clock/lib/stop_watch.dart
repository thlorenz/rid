import 'package:clock/generated/rid_generated.dart';
import 'package:clock/response_channel.dart';

extension ModelMessages on Pointer<Model> {
  Future<Response> msgStart() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgStart(reqID, this);
    return ResponseChannel.instance.response(reqID);
  }

  Future<Response> msgStop() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgStop(reqID, this);
    return ResponseChannel.instance.response(reqID);
  }

  Future<Response> msgReset() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgReset(reqID, this);
    return ResponseChannel.instance.response(reqID);
  }
}

class StopWatch {
  final chan = ResponseChannel.instance;

  final Pointer<Model> model;

  StopWatch(this.model);

  void startTimer() async {
    final res = await model.msgStart();
    print('$res');
  }

  void stopTimer() async {
    final res = await model.msgStop();
    print('$res');
  }

  void resetTimer() async {
    final res = await model.msgReset();
    print('$res');
  }
}
