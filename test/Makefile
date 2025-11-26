ifeq ($(CI),true)
  BUILD_DIR := $(GITHUB_WORKSPACE)
else
  BUILD_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))../
endif

TARGETS := nostdtest nostdtest-serde

default: $(TARGETS)
.PHONY: $(TARGETS)

nostdtest:
	cargo build --manifest-path test/nostdtest/Cargo.toml

nostdtest-serde:
	cargo build --manifest-path test/nostdtest-serde/Cargo.toml

clean:
	bash -c "rm -rf $(BUILD_DIR)/test/*/{Cargo.lock,target/}"
