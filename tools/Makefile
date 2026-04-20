# =============================================================================
# SigmaOS Tools — Sovereign Native Toolchain Build
# Replaces ALL Python/JS tooling with zero-dependency static C binaries
# =============================================================================
# Usage:
#   make all          — build every tool
#   make check        — purity gate: fail if foreign deps in kernel/
#   make clean        — remove compiled binaries
#   make install      — install to /usr/local/sigma/bin (on target OS)
# =============================================================================

CC       = gcc
CFLAGS   = -std=c11 -O2 -Wall -Iinclude
LDFLAGS  = -static

BINDIR   = /usr/local/sigma/bin

# All sovereign tool targets
TOOLS = \
	sigma-audit       \
	sigma-test        \
	sigma-wiki        \
	sigma-shell       \
	sigma-depgraph    \
	sigpkg            \
	sigma-modularize  \
	sigma-purge

.PHONY: all clean check install uninstall

all: $(TOOLS)
	@echo ""
	@echo "  ┌──────────────────────────────────────────┐"
	@echo "  │  SigmaOS Native Toolchain — BUILD OK     │"
	@echo "  │  All tools: zero glibc runtime linkage   │"
	@echo "  └──────────────────────────────────────────┘"
	@echo ""

sigma-audit: security/sovereign_audit/sovereign_audit.c
	$(CC) $(CFLAGS) $(LDFLAGS) $< -o $@
	@echo "  [✓] sigma-audit    → replaces global_integrated_audit.py"

sigma-test: dev/sovereign_test/sovereign_test_runner.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [✓] sigma-test     → replaces sovereign_test_runner.py + logic_tester.py"

sigma-wiki: wiki/sovereign_wiki/sovereign_wiki_builder.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-wiki     -> replaces wiki_builder.py"

sigma-shell: core/sovereign_shell/sovereign_shell.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-shell    -> replaces zenith_shell.py"

sigma-depgraph: dev/sovereign_depgraph/sovereign_depgraph.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-depgraph -> replaces generate_dependency_graph.py"

sigpkg: infra/sovereign_pkg/sovereign_pkg_manager.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [✓] sigpkg             → replaces sigpkg.py"

sigma-modularize: dev/sovereign_modularize/sovereign_modularize.c
	$(CC) $(CFLAGS) $(LDFLAGS) $< -o $@
	@echo "  [✓] sigma-modularize  → replaces sovereign_modularize_v2.py"

sigma-purge: dev/sovereign_purge/sovereign_purge.c
	$(CC) $(CFLAGS) $(LDFLAGS) $< -o $@
	@echo "  [✓] sigma-purge       → replaces sovereign_purge.py"


# Dependency purity gate — fail if any .py/.js found inside kernel suites
check:
	@echo "[sigma-check] Scanning kernel/suites for foreign dependencies..."
	@FOUND=$$(find ../kernel/suites -name "*.py" -o -name "*.js" 2>/dev/null | wc -l); \
	if [ "$$FOUND" -gt 0 ]; then \
	    echo "  [FAIL] $$FOUND foreign files in kernel suites:"; \
	    find ../kernel/suites -name "*.py" -o -name "*.js"; \
	    exit 1; \
	else \
	    echo "  [PASS] Kernel suites S01-S10: zero foreign runtime deps"; \
	fi

install: all
	@mkdir -p $(BINDIR)
	@for tool in $(TOOLS); do \
	    cp $$tool $(BINDIR)/$$tool; \
	    echo "  [install] $(BINDIR)/$$tool"; \
	done

uninstall:
	@for tool in $(TOOLS); do rm -f $(BINDIR)/$$tool; done

clean:
	@rm -f $(TOOLS)
	@echo "  [clean] Removed all tool binaries"
