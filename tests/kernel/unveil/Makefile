# SPDX-License-Identifier: GPL-2.0-or-later
CXX      ?= g++
CXXFLAGS  = -std=c++17 -Wall -Wextra -I../../../include -I../../../klib/include \
            -I../../../kernel/security/mac
STUBS     = ../stubs/sigma_stubs.cpp
TESTS     = test_unveil_deny test_unveil_lock

.PHONY: regress clean

regress: $(TESTS)
	@for t in $(TESTS); do \
	  printf "  %-40s " "$$t"; \
	  ./$$t && echo "PASS" || echo "FAIL"; \
	done

test_unveil_deny: test_unveil_deny.cpp $(STUBS) \
                  ../../../kernel/security/mac/sigma_unveil.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

test_unveil_lock: test_unveil_lock.cpp $(STUBS) \
                  ../../../kernel/security/mac/sigma_unveil.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	rm -f $(TESTS)
