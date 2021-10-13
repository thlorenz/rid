include ../../../Makefile.variable

test-all:
	$(MAKE) test TEST=strings
	$(MAKE) test TEST=primitives
	$(MAKE) test TEST=structs
	$(MAKE) test TEST=enums
	$(MAKE) test TEST=vecs
	$(MAKE) test TEST=hash_maps

test:
	$(PUB_PREFIX) pub get
	cargo build --example $(TEST) && \
	$(SET_PREFIX) TEST=$(TEST)&& cargo run rid_build && \
	dart run test ./test/$(TEST).dart

./.dart_tool/:
	$(PUB_PREFIX) pub get

.PHONY: test-all test expand
