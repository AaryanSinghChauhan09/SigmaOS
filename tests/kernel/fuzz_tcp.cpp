// Mock fuzz testing framework for SigmaOS TCP stack
#include <stddef.h>
#include <stdint.h>

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    // Dummy fuzz logic
    return 0;
}
