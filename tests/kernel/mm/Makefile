# SPDX-License-Identifier: GPL-2.0-or-later
CXX      ?= g++
CXXFLAGS  = -std=c++17 -Wall -Wextra -I../../../include -I../../../klib/include
STUBS     = ../stubs/sigma_stubs.cpp
TESTS     = test_cow test_demand_page

.PHONY: regress clean

regress: $(TESTS)
	@for t in $(TESTS); do \
	  printf "  %-40s " "$$t"; \
	  ./$$t && echo "PASS" || echo "FAIL"; \
	done

test_cow: test_cow.cpp $(STUBS)
	$(CXX) $(CXXFLAGS) $^ -o $@

test_demand_page: test_demand_page.cpp $(STUBS)
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	rm -f $(TESTS)
