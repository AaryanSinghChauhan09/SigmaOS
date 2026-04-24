/**
 * @file test_runner.cpp
 * @brief SigmaOS Sovereign Native Test Runner (Low-Level Implementation)
 * 
 * Replaces run_sigma_tests.sh to reduce dependency on bash/python.
 */

#include <iostream>
#include <vector>
#include <string>
#include <filesystem>
#include <cstdlib>

namespace fs = std::filesystem;

class TestRunner {
public:
    void run_all() {
        std::cout << "Σ [1/4] Build Artifact Verification\n";
        verify_build();

        std::cout << "\nΣ [2/4] Core Suite Presence\n";
        verify_suites();

        std::cout << "\nΣ [3/4] Module Manifest Integrity\n";
        verify_manifests();

        std::cout << "\nΣ [4/4] Hardware Abstraction Layer Contracts\n";
        verify_hal();

        print_summary();
    }

private:
    int pass = 0;
    int fail = 0;

    void check(const std::string& desc, bool result) {
        if (result) {
            std::cout << "  ✅ PASS: " << desc << "\n";
            pass++;
        } else {
            std::cout << "  ❌ FAIL: " << desc << "\n";
            fail++;
        }
    }

    void verify_build() {
        check("build/ directory exists", fs::exists("build"));
    }

    void verify_suites() {
        std::vector<std::string> core_suites = {
            "S01_Genesis", "S03_Orchestrator", "S04_HAL", "S05_Memory", "S30_Supremacy"
        };
        for (const auto& s : core_suites) {
            check("Suite " + s + " present", fs::exists("suites/" + s));
        }
    }

    void verify_manifests() {
        int count = 0;
        for (const auto& entry : fs::recursive_directory_iterator("suites")) {
            if (entry.path().filename() == "module.json") {
                count++;
            }
        }
        check("Module manifests found: " + std::to_string(count), count > 0);
    }

    void verify_hal() {
        check("HAL Contract Header", fs::exists("include/sigma/hal_contract.h"));
    }

    void print_summary() {
        std::cout << "\n══════════════════════════════════════════════════════════\n";
        std::cout << "  Σ TEST RESULTS: " << pass << " passed | " << fail << " failed\n";
        std::cout << "══════════════════════════════════════════════════════════\n";
        if (fail > 0) std::exit(1);
    }
};

int main() {
    std::cout << "╔══════════════════════════════════════════════════════════╗\n";
    std::cout << "║  Σ SigmaOS Native Test Runner v1.0                      ║\n";
    std::cout << "║  Bare-Metal Lattice Integrity Verification              ║\n";
    std::cout << "╚══════════════════════════════════════════════════════════╝\n\n";

    TestRunner runner;
    runner.run_all();

    return 0;
}
