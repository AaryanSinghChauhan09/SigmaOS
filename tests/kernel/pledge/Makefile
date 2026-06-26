# SPDX-License-Identifier: GPL-2.0-or-later
CXX      ?= g++
CXXFLAGS  = -std=c++17 -Wall -Wextra -I../../../include -I../../../klib/include \
            -I../../../kernel/security/jail
STUBS     = ../stubs/sigma_stubs.cpp
TESTS     = test_pledge_stdio test_pledge_violation

.PHONY: regress clean

regress: $(TESTS)
	@for t in $(TESTS); do \
	  printf "  %-40s " "$$t"; \
	  ./$$t && echo "PASS" || echo "FAIL"; \
	done

test_pledge_stdio: test_pledge_stdio.cpp $(STUBS) \
                   ../../../kernel/security/jail/sigma_pledge.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

test_pledge_violation: test_pledge_violation.cpp $(STUBS) \
                       ../../../kernel/security/jail/sigma_pledge.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	rm -f $(TESTS)
