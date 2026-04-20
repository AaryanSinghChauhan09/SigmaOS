# =============================================================================
# SigmaOS Tools — Sovereign Native Toolchain Build
# Replaces ALL Python/JS tooling with zero-dependency C binaries
# =============================================================================

CC       = gcc
CFLAGS   = -std=c11 -O2 -Wall -DSIGMA_EXCLUDE_STD_ALIASES -Iinclude -I../include -I../include/suites/S01_Genesis/shards

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
	@echo "  SigmaOS Native Toolchain: BUILD OK"
	@echo ""

sigma-audit: security/sovereign_audit/sovereign_audit.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-audit    -> sovereign_audit"

sigma-test: dev/sovereign_test/sovereign_test_runner.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-test     -> sovereign_test_runner"

sigma-wiki: wiki/sovereign_wiki/sovereign_wiki_builder.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-wiki     -> sovereign_wiki_builder"

sigma-shell: core/sovereign_shell/sovereign_shell.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-shell    -> sovereign_shell"

sigma-depgraph: dev/sovereign_depgraph/sovereign_depgraph.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-depgraph -> sovereign_depgraph"

sigpkg: infra/sovereign_pkg/sovereign_pkg_manager.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigpkg         -> sovereign_pkg_manager"

sigma-modularize: dev/sovereign_modularize/sovereign_modularize.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-modularize -> sovereign_modularize"

sigma-purge: dev/sovereign_purge/sovereign_purge.c
	$(CC) $(CFLAGS) $< -o $@
	@echo "  [OK] sigma-purge    -> sovereign_purge"


# Dependency purity gate
check:
	@echo "[sigma-check] Scanning kernel/suites for foreign dependencies..."
	@FOUND=$$(find ../kernel/suites -name "*.py" -o -name "*.js" 2>/dev/null | wc -l); \
	if [ "$$FOUND" -gt 0 ]; then \
	    echo "  [FAIL] $$FOUND foreign files in kernel suites:"; \
	    find ../kernel/suites -name "*.py" -o -name "*.js"; \
	    exit 1; \
	else \
	    echo "  [PASS] Zero foreign runtime deps in kernel suites"; \
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
