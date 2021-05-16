class ReplyChannelStub {
  Future<void> dispose() => Future.value();
}

/// Stubbing reply channel until we declare a #[rid:reply] enum.
final replyChannel = ReplyChannelStub();
