# SPDX-License-Identifier: GPL-2.0-or-later
CXX      ?= g++
CXXFLAGS  = -std=c++17 -Wall -Wextra \
            -I../../include -I../../klib/include \
            -I../../userland/pkg
STUBS     = ../kernel/stubs/sigma_stubs.cpp
TESTS     = test_pkg_resolve test_pkg_verify test_staged_update

.PHONY: regress clean

regress: $(TESTS)
	@for t in $(TESTS); do \
	  printf "  %-40s " "$$t"; \
	  ./$$t && echo "PASS" || echo "FAIL"; \
	done

test_pkg_resolve: test_pkg_resolve.cpp $(STUBS)
	$(CXX) $(CXXFLAGS) $^ -o $@

test_pkg_verify: test_pkg_verify.cpp $(STUBS)
	$(CXX) $(CXXFLAGS) $^ -o $@

test_staged_update: test_staged_update.cpp $(STUBS) \
                    ../../userland/pkg/sigma_staged_update.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	rm -f $(TESTS)
