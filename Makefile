ROOT:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

test: test_rust test_integration

test_rust:
	cargo test --all

test_integration:
	cd $(ROOT)/tests/dart/field_access && $(MAKE) test-all && \
	cd $(ROOT)/tests/dart/export && $(MAKE) test-all &&       \
	cd $(ROOT)/tests/dart/apps && $(MAKE) test-all

.PHONY: test_rust test_integration test
