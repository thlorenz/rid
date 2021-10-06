include ../../../Makefile.variable

test-all:
	$(MAKE) test TEST=messaging

test:
	$(PUB_PREFIX) pub get
	cargo build --example $(TEST) && \
	$(SET_PREFIX) TEST=$(TEST)&& cargo run rid_build && \
	dart run test ./test/$(TEST).dart

expand:
	@cargo expand --example $(TEST)

.PHONY: test-all test expand
