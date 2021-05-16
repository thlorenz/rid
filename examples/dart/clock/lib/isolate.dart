import 'dart:async';
import 'dart:isolate';

// Adapted from https://github.com/dart-lang/isolate

void _castComplete<R>(Completer<R> completer, Object? value) {
  try {
    completer.complete(value as R);
  } catch (error, stack) {
    completer.completeError(error, stack);
  }
}

SendPort singleCallbackPort<P>(void Function(P? response) callback,
    {Duration? timeout, P? timeoutValue}) {
  var responsePort = RawReceivePort();
  var zone = Zone.current;
  callback = zone.registerUnaryCallback(callback);
  Timer? timer;
  responsePort.handler = (response) {
    responsePort.close();
    timer?.cancel();
    zone.runUnary(callback, response as P);
  };
  if (timeout != null) {
    timer = Timer(timeout, () {
      responsePort.close();
      callback(timeoutValue);
    });
  }
  return responsePort.sendPort;
}

SendPort singleCompletePort<R, P>(Completer<R> completer,
    {FutureOr<R> Function(P message)? callback,
    Duration? timeout,
    FutureOr<R> Function()? onTimeout}) {
  if (callback == null && timeout == null) {
    return singleCallbackPort<Object>((response) {
      _castComplete<R>(completer, response);
    });
  }
  var responsePort = RawReceivePort();
  Timer? timer;
  if (callback == null) {
    responsePort.handler = (response) {
      responsePort.close();
      timer?.cancel();
      _castComplete<R>(completer, response);
    };
  } else {
    var zone = Zone.current;
    var action = zone.registerUnaryCallback((response) {
      try {
        // Also catch it if callback throws.
        completer.complete(callback(response as P));
      } catch (error, stack) {
        completer.completeError(error, stack);
      }
    });
    responsePort.handler = (response) {
      responsePort.close();
      timer?.cancel();
      zone.runUnary(action, response as P);
    };
  }
  if (timeout != null) {
    timer = Timer(timeout, () {
      responsePort.close();
      if (onTimeout != null) {
        completer.complete(Future.sync(onTimeout));
      } else {
        completer
            .completeError(TimeoutException('Future not completed', timeout));
      }
    });
  }
  return responsePort.sendPort;
}
