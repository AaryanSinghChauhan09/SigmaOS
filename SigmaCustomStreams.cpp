/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * Σ SIGMA OS: SOVEREIGN C++ STREAMS (v8.0 - ZERO-IOSTREAM CUSTOM)
 * ===============================================================
 * USP Absorbed: Clean Architecture, LLVM libc++ optimization.
 * Capability: Overload "<<" operators mapping directly to SigmaLibC Assembly.
 * Principle: Zero generic `<iostream>` dependency, strict compiler isolation.
 */

// We include only our custom C headers instead of standard libraries
#include "SigmaLibC.h"

// Replaces the entire `std` namespace and `std::ostream` / `std::cout`
namespace sigma {

    // Custom output stream class replacing std::ostream
    class SovereignStream {
    public:
        // Overload operator<< for strings
        SovereignStream& operator<<(const char* text) {
            sigma_print(text); // Maps to our custom x86_64 sys_write assembly wrapper
            return *this;
        }

        // Overload operator<< for 64-bit integers
        SovereignStream& operator<<(sigma_i64 number) {
            sigma_print_int(number); // Maps to our custom ASCII formatting logic
            return *this;
        }

        // Overload operator<< for 32-bit integers
        SovereignStream& operator<<(sigma_i32 number) {
            sigma_print_int((sigma_i64)number);
            return *this;
        }
    };

    // Global singleton instance replacing std::cout
    SovereignStream cout;

    // Define a custom representation for line endings replacing std::endl
    const char* endl = "\n";
}

// Custom Entry Point Bypass (g++ -nostdlib)
extern "C" void _start() {
    // We are executing pure C++ objects but zero generic C++ libraries are linked.
    using namespace sigma;
    
    cout << "[SIGMA_COUT]: Bootstrapping Zero-Library C++ Sovereign Stream." << endl;
    cout << "[SIGMA_COUT]: Using completely overloaded internal << operators." << endl;
    cout << "[SIGMA_COUT]: Number Formatting active: " << (sigma_i64)1024 << " bytes." << endl;
    
    cout << "[SUCCESS]: Competitive C++ Native Output Engine Online. " << endl;

    // Syscall 60 exit
#if defined(__x86_64__)
    __asm__ volatile ("mov $60, %%rax\n xor %%rdi, %%rdi\n syscall\n" ::: "rax", "rdi");
#endif
}

