include ../../../Makefile.variable

test-all: 
	$(MAKE) test TEST=primitives
	$(MAKE) test TEST=strings
	$(MAKE) test TEST=enums
	$(MAKE) test TEST=vecs
	$(MAKE) test TEST=args_strings
# 	$(MAKE) test TEST=structs

test:
	$(MAKE) ./.dart_tool/
	cargo build --example $(TEST) && \
	$(SET_PREFIX) TEST=$(TEST)&& cargo run rid_build && \
	dart run test ./test/$(TEST).dart

expand:
	@cargo expand --example $(TEST)

./.dart_tool/:
	$(PUB_PREFIX) pub get

.PHONY: test-all test expand
