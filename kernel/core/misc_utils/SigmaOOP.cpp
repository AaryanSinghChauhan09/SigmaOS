#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/core/SigmaOOP.hpp"

void* operator new(sigma_size_t size) {
    return sigma_malloc(size);
}

void* operator new[](sigma_size_t size) {
    return sigma_malloc(size);
}

void operator delete(void* ptr) noexcept {
    sigma_free(ptr);
}

void operator delete(void* ptr, sigma_size_t size) noexcept {
    (void)size;
    sigma_free(ptr);
}

void operator delete[](void* ptr) noexcept {
    sigma_free(ptr);
}



