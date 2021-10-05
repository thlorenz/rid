include ../../../Makefile.variable

test-all:
	$(MAKE) test TEST=strings
	$(MAKE) test TEST=primitives
	$(MAKE) test TEST=structs
	$(MAKE) test TEST=enums
	$(MAKE) test TEST=vecs
	$(MAKE) test TEST=hash_maps

test: ./.dart_tool/
	cargo build --example $(TEST) && \
	TEST=$(TEST) cargo run rid_build && \
	dart run test ./test/$(TEST).dart

expand:
	@cargo expand --example $(TEST) 

./.dart_tool/:
	$(PUB_PREFIX) pub get

.PHONY: test-all test expand
