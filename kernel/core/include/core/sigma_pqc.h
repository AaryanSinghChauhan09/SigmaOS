#pragma once
#include "core/sigma_types.h"

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignPQC {
public:
    static void init();
    static bool sign(const sigma_u8* msg, sigma_size size, sigma_u8* sig);
    static bool verify(const sigma_u8* msg, sigma_size size, const sigma_u8* sig);
    static void encrypt(sigma_u8* data, sigma_size size);
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
