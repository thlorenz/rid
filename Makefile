ROOT:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
test:
	cargo test --all &&                                       \
	cd $(ROOT)/tests/dart/field_access && $(MAKE) test-all && \
	cd $(ROOT)/tests/dart/export && $(MAKE) test-all &&       \
	cd $(ROOT)/tests/dart/apps && $(MAKE) test-all
