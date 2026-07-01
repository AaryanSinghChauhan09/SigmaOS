# SPDX-License-Identifier: GPL-2.0-or-later
# tests/Makefile — top-level regress runner (OpenBSD regress/ inspired)
#
# Usage:
#   make -C tests regress       # run all subsystems
#   make -C tests kernel/pledge # run only pledge tests

SUBDIRS = kernel/pledge kernel/unveil kernel/mm kernel/net kernel/security \
          userland

.PHONY: regress clean $(SUBDIRS)

regress: $(SUBDIRS)
	@echo "=== SigmaOS regression suite complete ==="

$(SUBDIRS):
	@echo "--- regress: $@ ---"
	$(MAKE) -C $@ regress 2>&1 | tee ../tests/regress-$(subst /,-,$@).log || \
	  (echo "FAILED: $@" && exit 1)

clean:
	for d in $(SUBDIRS); do $(MAKE) -C $$d clean; done
	rm -f regress-*.log
