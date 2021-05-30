import 'package:clock/generated/rid_generated.dart';
import 'package:clock/response_channel.dart';

extension ModelMessages on Pointer<StoreAccess> {
  Future<Response> msgStart() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgStart(reqID);
    return ResponseChannel.instance.response(reqID);
  }

  Future<Response> msgStop() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgStop(reqID);
    return ResponseChannel.instance.response(reqID);
  }

  Future<Response> msgReset() {
    final reqID = ResponseChannel.instance.reqId;
    rid_ffi.msgReset(reqID);
    return ResponseChannel.instance.response(reqID);
  }
}

class StopWatch {
  final chan = ResponseChannel.instance;

  final Pointer<StoreAccess> store;

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
