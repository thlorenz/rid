include ./Makefile.variable

test: test_rust test_integration

test_rust:
	cargo test --all -- --test-threads 1

test_integration:
	cd $(ROOT)tests/dart/field_access && $(MAKE) test-all && \
	cd $(ROOT)tests/dart/export && $(MAKE) test-all &&       \
	cd $(ROOT)tests/dart/apps && $(MAKE) test-all &&         \
	cd $(ROOT)tests/dart/framework && $(MAKE) test-all

.PHONY: test_rust test_integration test
