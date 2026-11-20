.PHONY: all verify registry-audit consolidation-audit doctor sync-wiki sync-github lint clean help hub hub-list hub-status hub-audit hub-create hub-upgrade hub-sync hub-search hub-info hub-depends hub-families hub-consolidate hub-test hub-health hub-dupes hub-connect hub-graph hub-wire

PYTHON ?= python3
BASH ?= bash

# This repo is the connector/control plane for the SigmaOS-* component
# fleet. Component implementations live in their own SigmaOS-<name>
# repositories — see scripts/sigma_hub.sh and AGENTS.md.

all: verify

# Milestone verify: registry audits + shell syntax checks.
verify: registry-audit consolidation-audit lint doctor
	@echo "==> make verify succeeded: registry audits, lint, and doctor passed cleanly."

# Read-only inventory of all repositories declared in COMPONENTS.toml.
registry-audit:
	@$(PYTHON) scripts/audit_component_fleet.py

# Read-only validation of canonical suite mappings and unresolved constituents.
consolidation-audit:
	@$(PYTHON) scripts/audit_consolidation.py

# Component-repository hub: this repo is the connector for the SigmaOS-*
# fleet. See scripts/sigma_hub.sh --help for the full command set.
hub:
	@$(BASH) scripts/sigma_hub.sh help

hub-list:
	@$(BASH) scripts/sigma_hub.sh list $(FILTER)

hub-status:
	@$(BASH) scripts/sigma_hub.sh status $(NAME)

hub-audit:
	@$(BASH) scripts/sigma_hub.sh audit

hub-connect:
	@$(BASH) scripts/sigma_hub.sh connect $(ARGS)

hub-create:
	@$(BASH) scripts/sigma_hub.sh create $(NAME) $(ARGS)

hub-upgrade:
	@$(BASH) scripts/sigma_hub.sh upgrade $(NAME) $(ARGS)

hub-sync:
	@$(BASH) scripts/sigma_hub.sh sync $(ARGS)

hub-search:
	@$(BASH) scripts/sigma_hub.sh search $(ARGS)

hub-info:
	@$(BASH) scripts/sigma_hub.sh info $(NAME)

hub-depends:
	@$(BASH) scripts/sigma_hub.sh depends $(NAME)

hub-families:
	@$(BASH) scripts/sigma_hub.sh families $(ARGS)

hub-consolidate:
	@$(BASH) scripts/sigma_hub.sh consolidate $(ARGS)

hub-test:
	@$(BASH) scripts/sigma_hub.sh test $(ARGS)

hub-health:
	@$(BASH) scripts/sigma_hub.sh health

hub-dupes:
	@$(BASH) scripts/sigma_hub.sh dupes

hub-graph:
	@$(BASH) scripts/sigma_hub.sh graph $(ARGS)
hub-wire:
	@$(BASH) scripts/sigma_hub.sh wire $(ARGS)

# Shell syntax check across all connector scripts.
lint:
	@echo "==> bash -n on all shell scripts..."
	@fail=0; \
	for f in $$(git ls-files '*.sh' | grep -v archive/); do \
		bash -n "$$f" || { echo "SYNTAX FAIL: $$f"; fail=1; }; \
	done; \
	[ $$fail -eq 0 ] && echo "==> all shell scripts parse cleanly" || exit 1

# Repository/profile validation.
doctor:
	@$(BASH) scripts/sigmaos_doctor.sh

# Wiki synchronization (component wikis -> GitHub wiki).
sync-wiki:
	@$(BASH) scripts/sync_wiki.sh

sync-wiki-gh:
	@$(BASH) scripts/sync_wiki_to_github.sh

# Push connector repo changes.
sync-github:
	@git push origin main

clean:
	@echo "==> Cleaning connector build artifacts..."
	rm -rf build/ target/ __pycache__/ .pytest_cache/

help:
	@echo "SigmaOS Connector — fleet control plane"
	@echo "  make verify       - registry audits + lint + doctor (read-only)"
	@echo "  make registry-audit - Audit managed component checkouts and metadata"
	@echo "  make consolidation-audit - Validate canonical suite mappings"
	@echo "  make doctor       - Repository/profile validation"
	@echo "  make lint         - bash -n on all shell scripts"
	@echo "  make hub          - Component hub help (connect/create/upgrade/sync)"
	@echo "  make hub-list     - List components + local git status (FILTER=--category X)"
	@echo "  make hub-status   - Fleet or per-repo status (NAME=SigmaOS-foo)"
	@echo "  make hub-audit    - Count COMPONENT.toml coverage / upgrade candidates"
	@echo "  make hub-connect  - Clone/connect component repos (ARGS=SigmaOS-foo)"
	@echo "  make hub-create   - Scaffold new component repo (NAME=..., ARGS=--desc ... --dry-run)"
	@echo "  make hub-upgrade  - Bring repos up to component standard (NAME=--all)"
	@echo "  make hub-sync     - Pull/push component checkouts (ARGS=--push)"
	@echo "  make hub-search   - Search components (ARGS=<regex>)"
	@echo "  make hub-info     - Show component metadata (NAME=SigmaOS-foo)"
	@echo "  make hub-depends  - Show component deps (NAME=SigmaOS-foo)"
	@echo "  make hub-families - Scaffold-fleet consolidation analyzer"
	@echo "  make hub-consolidate - Dry-run merge plan (ARGS=<family>)"
	@echo "  make hub-test     - cargo test across fleet (ARGS=<name|--local>)"
	@echo "  make hub-health   - Fleet-health summary (coverage/scaffold/git)"
	@echo "  make hub-dupes    - Duplicate-component analyzer"
	@echo "  make hub-graph    - Graphviz dependency edges (ARGS=<out.dot>)"
	@echo "  make sync-wiki    - Sync component wikis to GitHub wiki"
	@echo "  make sync-github  - Push connector repo to origin/main"
	@echo "  make clean        - Remove build artifacts"
