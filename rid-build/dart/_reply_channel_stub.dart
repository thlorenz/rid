class RidReplyChannelStub {
  Future<void> dispose() => Future.value();
}

/// Stubbing reply channel until we declare a #[rid:reply] enum.
final _replyChannel = RidReplyChannelStub();
