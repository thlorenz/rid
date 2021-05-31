import 'package:clock/generated/rid_generated.dart';
import 'package:clock/response.dart';
import 'package:clock/response_channel.dart';

extension ModelMessages on Pointer<StoreAccess> {
  Future<Response> msgStart() {
    final reqID = responseChannel.reqId;
    rid_ffi.msgStart(reqID);
    return responseChannel.response(reqID);
  }

  Future<Response> msgStop() {
    final reqID = responseChannel.reqId;
    rid_ffi.msgStop(reqID);
    return responseChannel.response(reqID);
  }

  Future<Response> msgReset() {
    final reqID = responseChannel.reqId;
    rid_ffi.msgReset(reqID);
    return responseChannel.response(reqID);
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
