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

/// [RidStatefulWidget] is identical to a [StatefulWidget] except that it ensures to lock the
/// rid store during widget builds.
abstract class RidStatefulWidget extends StatefulWidget {
  const RidStatefulWidget({Key? key}) : super(key: key);

  @override
  StatefulElement createElement() => RidStatefulElement(this);
}
