// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * =============================================================================
 * Σ SIGMAOS: libFuzzer harness for the TCP/IP receive path
 * =============================================================================
 * Replaces the old rand()-based deterministic "fuzzer" (test_tcp.cpp) with a
 * real coverage-guided libFuzzer entry point. libFuzzer will:
 *   - Mutate the input corpus automatically
 *   - Track edge coverage via SanitizerCoverage instrumentation
 *   - Report crashes with AddressSanitizer / UndefinedBehaviorSanitizer
 *
 * Build:
 *   clang++ -std=c++17 -fsanitize=fuzzer,address,undefined \
 *           -Iklib/include -Iinclude \
 *           tests/kernel/fuzz_tcp.cpp \
 *           kernel/net/sigma_tcpip.c \
 *           -o fuzz_tcp
 *
 * Run (30-second budget, detect memory leaks):
 *   ./fuzz_tcp -max_total_time=30 -detect_leaks=1
 *
 * Reproduce a specific crash:
 *   ./fuzz_tcp crash-<hash>
 * =============================================================================
 */

#include <stdint.h>
#include <stddef.h>

/*
 * sigma_tcp_rx — entry point under test.
 * Declared here so the fuzzer can link against the kernel net object without
 * pulling in the rest of the kernel (the klib stubs satisfy missing symbols).
 */
extern "C" void sigma_tcp_rx(void* packet, size_t len);

/*
 * Minimum viable packet: 20-byte IPv4 header + 20-byte TCP header = 40 bytes.
 * Inputs shorter than this cannot form a valid TCP segment — skip them to
 * keep the corpus focused on interesting cases.
 */
static constexpr size_t MIN_PACKET_SIZE = 40;

/*
 * LLVMFuzzerTestOneInput — called by libFuzzer with each mutated input.
 * Must be extern "C", must return 0 (non-zero values are reserved).
 * Must not call exit() or abort() — crashes/hangs are caught by the harness.
 */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size) {
    if (size < MIN_PACKET_SIZE) {
        return 0;  /* Too short to be interesting — discard without penalty */
    }

    /*
     * Cast away const: sigma_tcp_rx takes void* because the kernel may
     * modify headers in-place (e.g. byte-swap). The fuzzer's copy is on the
     * heap so this is safe.
     */
    sigma_tcp_rx(const_cast<uint8_t*>(data), size);

    return 0;
}

/*
 * Optional: provide an initial seed corpus so libFuzzer starts with a valid
 * TCP SYN packet rather than random bytes. This dramatically accelerates
 * coverage growth.
 *
 * Run with: ./fuzz_tcp tests/kernel/fuzz_corpus/
 * where fuzz_corpus/ contains binary files of valid TCP packets.
 */
