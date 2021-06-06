//
// Flutter specific overrides that make it easier to work with Rid.
//

// -----------------
// Stateless Widget
// -----------------
class RidStatelessElement extends StatelessElement {
  RidStatelessElement(StatelessWidget widget) : super(widget);

  @override
  Widget build() {
    try {
      ridStoreLock();
      return widget.build(this);
    } finally {
      ridStoreUnlock();
    }
  }
}

/// [RidStatelessWidget] is identical to a [StatelessWidget] except that it ensures to lock the
/// rid store during widget builds.
abstract class RidStatelessWidget extends StatelessWidget {
  const RidStatelessWidget({Key? key}) : super(key: key);
  @override
  StatelessElement createElement() => RidStatelessElement(this);
}

// -----------------
// Stateful Widget
// -----------------
class RidStatefulElement extends StatefulElement {
  /// Creates an element that uses the given widget as its configuration.
  RidStatefulElement(StatefulWidget widget) : super(widget);
  @override
  Widget build() {
    try {
      ridStoreLock();
      return state.build(this);
    } finally {
      ridStoreUnlock();
    }
  }
}

/// The [StateAsync] mixin adds the [setStateAsync] instance method to [State] which makes it
/// easier to update state after a rid message reply was received.
///
/// Example
/// ```dart
/// IconButton(
///   icon: Icon(Icons.all_inclusive),
///   onPressed: () =>
///       setStateAsync(() => _store.msgSetFilter(Filter.All)),
/// )
/// ```
mixin StateAsync<T extends StatefulWidget> on State<T> {
  /// Identical to [setState] except that it takes a function that returns a [Future].
  /// It will call [setState] whenever the reply posted in response to the message is received
  /// Note: that this function returns [void] to conform to the signature that most handlers
  /// like [onTap] want. Therefore you cannot await its outcome, if you need that use
  /// [setState] instead.
  void setStateAsync<T>(Future<T> Function() sendMsg) {
    sendMsg().whenComplete(() => setState(() {}));
  }
}

/// [RidStatefulWidget] is identical to a [StatefulWidget] except that it ensures to lock the
/// rid store during widget builds.
abstract class RidStatefulWidget extends StatefulWidget {
  const RidStatefulWidget({Key? key}) : super(key: key);

  @override
  StatefulElement createElement() => RidStatefulElement(this);
}
