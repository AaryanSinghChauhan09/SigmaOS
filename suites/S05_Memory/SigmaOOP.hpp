/* SigmaOOP.hpp — Sovereign canonical shim */
#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP
#include "sigma_kernel_types.h"
#include "sigma_libc.h"
namespace SigmaOS {
typedef sigma_u32 sigma_status;
#define SIGMA_OK    0x00000000U
#define SIGMA_ERROR 0xFFFFFFFFU
class SigmaMemory {
public:
    static void* allocate(sigma_u64 length) {
        return sigma_mmap(0, length, 3, 0x22, -1, 0);
    }
};
class SigmaObject {
public:
    virtual ~SigmaObject() = default;
    virtual const char* type_name() const noexcept = 0;
};
inline void sigma_log(const char* msg) {
    ::sigma_print("[SIGMA_LOG]: ");
    ::sigma_print(msg);
    ::sigma_print("\n");
}
} // namespace SigmaOS
#endif /* SIGMA_OOP_HPP */
