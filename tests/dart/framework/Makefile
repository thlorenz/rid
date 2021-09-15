test-all: 
	$(MAKE) test TEST=messaging

test: ./.dart_tool/
	cargo build --example $(TEST) && \
	TEST=$(TEST) cargo run rid_build && \
	dart run test ./test/$(TEST).dart

expand:
	@cargo expand --example $(TEST)

./.dart_tool/:
	pub get

.PHONY: test-all test expand
