#ifndef SOVEREIGN_CORE_UTILS_H
#define SOVEREIGN_CORE_UTILS_H

#include "../../SovereignOSBasicsZenith.h"

namespace SigmaOS {
namespace CoreUtils {

class SovereignListDir {
public:
    const char* type_name() const noexcept;
    void Execute(const char* path);
};

class SovereignConcatenate {
public:
    const char* type_name() const noexcept;
    void Execute(const char* file);
};

class SovereignGrepSearch {
public:
    const char* type_name() const noexcept;
    void Execute(const char* pattern, const char* file);
};

class SovereignProcessMonitor {
public:
    const char* type_name() const noexcept;
    void Execute();
};

class SovereignPermissionMod {
public:
    const char* type_name() const noexcept;
    void Execute(const char* permissions, const char* file);
};

} // namespace CoreUtils
} // namespace SigmaOS

extern "C" void sigma_core_utils_init(void);

#endif
