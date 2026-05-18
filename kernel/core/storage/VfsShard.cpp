#include "libc/SovereignLibC.h"
#include "sigma_hal.h"
#include "sigma_kernel_types.h"
#include "VfsShard.hpp"

// Implementation for SovereignVFS could go here if needed, 
// but most is header-only templates/classes for now.

namespace SigmaOS {
namespace Kernel {

// Global VFS instance
SovereignVFS g_VFS;

} // namespace Kernel
} // namespace SigmaOS



 