#ifndef SIGMA_PROFILING_TESTING_H
#define SIGMA_PROFILING_TESTING_H

#include "suites/S01_Genesis/shards/sigma_types.h"

// SigmaOS Development Profiling & Testing Framework
// Hardware-level hooks for TDD and kernel tracing.

// Run a sovereign-shard unit test macro suite
void dev_test_run_suite(const char* test_suite_name);

// Start an active hardware profiler intercepting cache-misses and cycles
void dev_profiler_start_tracing(uint32_t target_pid);

// Generate automated API documentation dynamically based on C11 AST parsing
void dev_api_generate_docs(const char* module_path, const char* output_repo);

#endif // SIGMA_PROFILING_TESTING_H

