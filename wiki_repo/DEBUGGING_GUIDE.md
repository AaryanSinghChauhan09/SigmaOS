# DEBUGGING GUIDE

1

Welcome to the SigmaOS debugging guide! Since SigmaOS targets bare-metal execution with a unique 600-shard modular architecture, standard debugging requires a slightly tailored approach.

1

Memory safety is a core priority. When compiling the kernel or individual shards for testing on a host OS (e.g., Linux/macOS), we enforce AddressSanitizer and UndefinedBehaviorSanitizer.

1

1

Every shard should use the lightweight logging framework.

1

1

1

We strictly enforce `clang-tidy` to catch C++ object lifecycle bugs and potential concurrency issues.

1

1

If you are modifying low-level parsing or cryptography shards, prefer the Rust implementations mapped via `SovereignRustInterop.cpp`. Rust's borrow checker eliminates a large class of memory bugs by default.

