ifeq ($(TRAVIS),true)
  BUILD_DIR := $(TRAVIS_BUILD_DIR)
else
  BUILD_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))../
endif

TARGETS := nostdtest nostdtest-serde

default: $(TARGETS)
.PHONY: $(TARGETS)

nostdtest:
	cd $(BUILD_DIR)/test/nostdtest && \
	cargo build

nostdtest-serde:
	cd $(BUILD_DIR)/test/nostdtest-serde && \
	cargo build

clean:
	rm -rf $(BUILD_DIR)/test/*/{Cargo.lock,target/}
