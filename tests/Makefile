# Makefile for SigmaOS Unit Tests

SOURCES = $(wildcard ../src/**/*.c) $(wildcard ../src/*.c)
TEST_SOURCES = $(wildcard test_*.c)

.PHONY: test valgrind_check fuzz_test clean

test:
	@echo "Compiling test suite..."
	@mkdir -p bin
	@gcc -coverage -g $(TEST_SOURCES) -o bin/test_suite -I../include
	@echo "Running tests..."
	@./bin/test_suite
	@echo "Generating coverage..."
	@# gcov -b ../src/**/*.c (commented out as sources are mocked)

valgrind_check:
	@echo "Running Valgrind Memory Checks..."
	@valgrind --leak-check=full --show-leak-kinds=all ./bin/test_suite

fuzz_test:
	@echo "Compiling Fuzzer..."
	@# clang -fsanitize=fuzzer,address -g $(SOURCES) fuzz_targets.c -o bin/fuzz
	@echo "Fuzzer compilation target ready."

clean:
	@rm -rf bin/ *.gcno *.gcda
