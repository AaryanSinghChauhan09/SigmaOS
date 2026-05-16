#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SigmaOOP.hpp"

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



