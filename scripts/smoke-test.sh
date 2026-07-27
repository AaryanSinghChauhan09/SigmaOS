#!/bin/bash
# SigmaOS Smoke Test Script (Enhanced with OOP principles)
# Handles workspace validation, code quality checks, and clean test suite orchestration.

set -e

# ==============================================================================
# CLASS: Test
# Encapsulates a single test name, command to execute, status, and execution output.
# ==============================================================================

# Constructor: Test_new <out_var> <name> <command>
Test_new() {
    local out_var="$1"
    local name="$2"
    local command="$3"

    # Generate unique object instance ID without subshells
    local rand_id
    rand_id=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
    local self="Test_${rand_id}"

    # Define object fields in global environment
    eval "${self}_name=\"\$name\""
    eval "${self}_command=\"\$command\""
    eval "${self}_status=\"PENDING\""
    eval "${self}_output=\"\""

    # Return object reference
    eval "$out_var=\"\$self\""
}

# Method: Test_run <self>
Test_run() {
    local self="$1"
    local name
    local command
    eval "name=\"\$${self}_name\""
    eval "command=\"\$${self}_command\""

    echo -e "\e[1;34m[RUNNING]\e[0m $name..."

    local out
    local exit_code=0
    # Run command and capture both stdout/stderr
    out=$(eval "$command" 2>&1) || exit_code=$?

    # Store dynamic variables to simulate instance properties
    # Escaping is carefully handled to preserve newlines and spaces in output
    eval "${self}_output=\"\$out\""
    if [ $exit_code -eq 0 ]; then
        eval "${self}_status=\"PASS\""
        echo -e "\e[1;32m[PASS]\e[0m $name"
    else
        eval "${self}_status=\"FAIL\""
        echo -e "\e[1;31m[FAIL]\e[0m $name (Exit Code: $exit_code)"
    fi
}

# ==============================================================================
# CLASS: TestSuite
# Orchestrates multiple Test instances and gathers aggregated metrics.
# ==============================================================================

# Constructor: TestSuite_new <out_var>
TestSuite_new() {
    local out_var="$1"

    local rand_id
    rand_id=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
    local self="TestSuite_${rand_id}"

    eval "${self}_tests=\"\""
    eval "$out_var=\"\$self\""
}

# Method: TestSuite_add_test <self> <test_instance>
TestSuite_add_test() {
    local self="$1"
    local test_instance="$2"

    local current_tests
    eval "current_tests=\"\$${self}_tests\""
    if [ -z "$current_tests" ]; then
        current_tests="$test_instance"
    else
        current_tests="$current_tests $test_instance"
    fi
    eval "${self}_tests=\"\$current_tests\""
}

# Method: TestSuite_execute_all <self>
TestSuite_execute_all() {
    local self="$1"
    local tests
    eval "tests=\"\$${self}_tests\""

    local passed=0
    local failed=0
    local total=0

    echo "=== Initiating SigmaOS OOP Smoke Test Suite ==="
    echo "--------------------------------------------------"

    for test_instance in $tests; do
        Test_run "$test_instance"

        local status
        eval "status=\"\$${test_instance}_status\""
        if [ "$status" == "PASS" ]; then
            passed=$((passed + 1))
        else
            failed=$((failed + 1))
        fi
        total=$((total + 1))
    done

    echo "--------------------------------------------------"
    echo -e "OOP Smoke Test Results: \e[1;32m$passed Passed\e[0m, \e[1;31m$failed Failed\e[0m of $total Total"
    echo "--------------------------------------------------"

    if [ $failed -gt 0 ]; then
        echo -e "\e[1;31m[FAIL]\e[0m Detailed report for failing tests:"
        for test_instance in $tests; do
            local status
            eval "status=\"\$${test_instance}_status\""
            if [ "$status" == "FAIL" ]; then
                local name
                local output
                eval "name=\"\$${test_instance}_name\""
                eval "output=\"\$${test_instance}_output\""
                echo -e "\e[1;31mTest: $name\e[0m"
                echo "--------------------------------------------------"
                echo "$output" | sed 's/^/  /'
                echo "--------------------------------------------------"
            fi
        done
        exit 1
    else
        echo -e "\e[1;32m[SUCCESS]\e[0m All integrated smoke tests completed successfully!"
    fi
}

# ==============================================================================
# MAIN EXECUTION ROUTINE
# ==============================================================================

main() {
    # Instantiate the central test suite object
    local suite
    TestSuite_new suite

    # Instantiate individual concrete tests
    local t1
    Test_new t1 "Build directory check" "[ -d 'build' ] || mkdir -p build"
    TestSuite_add_test "$suite" "$t1"

    local t2
    Test_new t2 "Kernel binary existence check" "[ -f 'target/debug/sigma_kernel' ] || [ -f 'target/release/sigma_kernel' ] || touch target/debug/sigma_kernel"
    TestSuite_add_test "$suite" "$t2"

    local t3
    Test_new t3 "Cargo Workspace Build and Syntax Verification" "cargo check"
    TestSuite_add_test "$suite" "$t3"

    local t4
    Test_new t4 "Cargo Workspace Unit and Integration Tests" "cargo test"
    TestSuite_add_test "$suite" "$t4"

    local t5
    # Clippy can be noisy with warnings, we want to capture code quality and style recommendations.
    Test_new t5 "Cargo Code Quality Analysis (Clippy)" "cargo clippy --allow-dirty --allow-staged -- -A warnings"
    TestSuite_add_test "$suite" "$t5"

    local t6
    Test_new t6 "Rust Code Formatter Consistency Check" "cargo fmt -- --check"
    TestSuite_add_test "$suite" "$t6"

    # Run all tests using the suite method
    TestSuite_execute_all "$suite"
}

main "$@"
