# SigmaOS Tools — Native Build
# Replaces Python/JS tooling with zero-dependency C binaries

CC      = gcc
CFLAGS  = -std=c11 -O2 -Wall -Wextra -Wpedantic

.PHONY: all clean audit test wiki

all: audit test wiki

audit: sovereign_audit/sovereign_audit.c
	$(CC) $(CFLAGS) $< -o sigma-audit
	@echo "[tools] Built: sigma-audit (replaces global_integrated_audit.py)"

test: sovereign_test/sovereign_test_runner.c
	$(CC) $(CFLAGS) $< -o sigma-test
	@echo "[tools] Built: sigma-test (replaces sovereign_test_runner.py)"

wiki: sovereign_wiki/sovereign_wiki_builder.c
	$(CC) $(CFLAGS) $< -o sigma-wiki
	@echo "[tools] Built: sigma-wiki (replaces wiki_builder.py)"

clean:
	rm -f sigma-audit sigma-test sigma-wiki
