# =============================================================================
# SigmaOS Component Makefile
# Component: COMPONENT_NAME
# =============================================================================
# Principles: KISS, DRY, Separation of Concerns, Defensive Programming
# All targets are idempotent and fail-fast.
# =============================================================================

.PHONY: all build test clean lint check install fmt sigma-check help

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------
COMPONENT   := COMPONENT_NAME
CARGO       := cargo
CARGO_FLAGS :=
RELEASE_FLAGS := --release

# Target triple for cross-compilation (override with: make build TARGET=aarch64-unknown-linux-musl)
TARGET ?=
ifdef TARGET
  TARGET_FLAG := --target $(TARGET)
else
  TARGET_FLAG :=
endif

# Install prefix (override with: make install PREFIX=/usr/local)
PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin
LIBDIR := $(PREFIX)/lib

# Coloured output helpers (no-op if terminal doesn't support it)
BOLD  := $(shell tput bold 2>/dev/null || echo '')
RESET := $(shell tput sgr0 2>/dev/null || echo '')
GREEN := $(shell tput setaf 2 2>/dev/null || echo '')
RED   := $(shell tput setaf 1 2>/dev/null || echo '')
CYAN  := $(shell tput setaf 6 2>/dev/null || echo '')

# -----------------------------------------------------------------------------
# Default target
# -----------------------------------------------------------------------------
all: sigma-check build

# -----------------------------------------------------------------------------
# build — compile in debug mode (default) or release
# -----------------------------------------------------------------------------
build:
	@echo "$(CYAN)[sigma] Building $(COMPONENT)...$(RESET)"
	$(CARGO) build $(CARGO_FLAGS) $(TARGET_FLAG)

build-release:
	@echo "$(CYAN)[sigma] Building $(COMPONENT) (release)...$(RESET)"
	$(CARGO) build $(RELEASE_FLAGS) $(CARGO_FLAGS) $(TARGET_FLAG)

# -----------------------------------------------------------------------------
# test — run the full test suite
# -----------------------------------------------------------------------------
test:
	@echo "$(CYAN)[sigma] Testing $(COMPONENT)...$(RESET)"
	$(CARGO) test $(CARGO_FLAGS) $(TARGET_FLAG) -- --test-threads=1

test-release:
	@echo "$(CYAN)[sigma] Testing $(COMPONENT) (release)...$(RESET)"
	$(CARGO) test $(RELEASE_FLAGS) $(CARGO_FLAGS) $(TARGET_FLAG) -- --test-threads=1

# -----------------------------------------------------------------------------
# lint — clippy with all warnings as errors
# -----------------------------------------------------------------------------
lint:
	@echo "$(CYAN)[sigma] Linting $(COMPONENT)...$(RESET)"
	$(CARGO) clippy $(CARGO_FLAGS) $(TARGET_FLAG) -- \
		-D warnings \
		-D clippy::all \
		-D clippy::pedantic \
		-A clippy::module_name_repetitions \
		-A clippy::must_use_candidate

# -----------------------------------------------------------------------------
# fmt — format source code
# -----------------------------------------------------------------------------
fmt:
	@echo "$(CYAN)[sigma] Formatting $(COMPONENT)...$(RESET)"
	$(CARGO) fmt --all

fmt-check:
	@echo "$(CYAN)[sigma] Checking formatting $(COMPONENT)...$(RESET)"
	$(CARGO) fmt --all -- --check

# -----------------------------------------------------------------------------
# check — fast type-check without producing artifacts
# -----------------------------------------------------------------------------
check:
	@echo "$(CYAN)[sigma] Checking $(COMPONENT)...$(RESET)"
	$(CARGO) check $(CARGO_FLAGS) $(TARGET_FLAG)

# -----------------------------------------------------------------------------
# install — install binary/library to PREFIX
# -----------------------------------------------------------------------------
install: build-release
	@echo "$(CYAN)[sigma] Installing $(COMPONENT) to $(PREFIX)...$(RESET)"
	$(CARGO) install --path . --root $(PREFIX) $(CARGO_FLAGS)

# -----------------------------------------------------------------------------
# clean — remove all build artifacts
# -----------------------------------------------------------------------------
clean:
	@echo "$(CYAN)[sigma] Cleaning $(COMPONENT)...$(RESET)"
	$(CARGO) clean
	@rm -rf coverage/ lcov.info tarpaulin-report.html

# -----------------------------------------------------------------------------
# sigma-check — enforce SigmaOS branch discipline:
#   main branch must contain NO .md or .txt files (docs live on docs branch)
# -----------------------------------------------------------------------------
sigma-check:
	@echo "$(CYAN)[sigma] Checking branch discipline...$(RESET)"
	@BRANCH=$$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown"); \
	if [ "$$BRANCH" = "main" ]; then \
		VIOLATIONS=$$(git ls-files '*.md' '*.txt' '*.rst' '*.adoc' 2>/dev/null); \
		if [ -n "$$VIOLATIONS" ]; then \
			echo "$(RED)[sigma-check] FAIL: Documentation files found on main branch:$(RESET)"; \
			echo "$$VIOLATIONS"; \
			echo "$(RED)[sigma-check] Move these to the docs branch. main branch is source code only.$(RESET)"; \
			exit 1; \
		fi; \
	fi
	@echo "$(GREEN)[sigma-check] PASS$(RESET)"

# -----------------------------------------------------------------------------
# audit — check for known security vulnerabilities in dependencies
# -----------------------------------------------------------------------------
audit:
	@echo "$(CYAN)[sigma] Auditing dependencies...$(RESET)"
	$(CARGO) audit

# -----------------------------------------------------------------------------
# coverage — generate code coverage report (requires cargo-tarpaulin)
# -----------------------------------------------------------------------------
coverage:
	@echo "$(CYAN)[sigma] Generating coverage report...$(RESET)"
	$(CARGO) tarpaulin --out Html --out Lcov --output-dir coverage/

# -----------------------------------------------------------------------------
# doc — build internal API documentation
# -----------------------------------------------------------------------------
doc:
	@echo "$(CYAN)[sigma] Building documentation...$(RESET)"
	$(CARGO) doc --no-deps --document-private-items

# -----------------------------------------------------------------------------
# help — list all targets
# -----------------------------------------------------------------------------
help:
	@echo "$(BOLD)SigmaOS Component Makefile — $(COMPONENT)$(RESET)"
	@echo ""
	@echo "  $(CYAN)make$(RESET)              — sigma-check + build (debug)"
	@echo "  $(CYAN)make build$(RESET)        — compile (debug)"
	@echo "  $(CYAN)make build-release$(RESET)— compile (release, optimised)"
	@echo "  $(CYAN)make test$(RESET)         — run test suite"
	@echo "  $(CYAN)make lint$(RESET)         — clippy (warnings as errors)"
	@echo "  $(CYAN)make fmt$(RESET)          — format source with rustfmt"
	@echo "  $(CYAN)make fmt-check$(RESET)    — verify formatting (CI)"
	@echo "  $(CYAN)make check$(RESET)        — fast type-check"
	@echo "  $(CYAN)make install$(RESET)      — install to PREFIX=$(PREFIX)"
	@echo "  $(CYAN)make clean$(RESET)        — remove build artifacts"
	@echo "  $(CYAN)make sigma-check$(RESET)  — enforce branch discipline"
	@echo "  $(CYAN)make audit$(RESET)        — dependency vulnerability audit"
	@echo "  $(CYAN)make coverage$(RESET)     — generate coverage report"
	@echo "  $(CYAN)make doc$(RESET)          — build API docs"
	@echo ""
	@echo "Cross-compile: make build TARGET=aarch64-unknown-linux-musl"
	@echo "Custom prefix: make install PREFIX=/opt/sigma"
