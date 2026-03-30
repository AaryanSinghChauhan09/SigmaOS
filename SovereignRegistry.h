#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace WindowsShard {

enum class RegistryHive {
    LOCAL_MACHINE,
    CURRENT_USER,
    SYSTEM,
    SOFTWARE
};

class SovereignRegistry : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignRegistry"; }

    void SetValue(RegistryHive hive, const char* path, const char* key, const char* value);
    const char* GetValue(RegistryHive hive, const char* path, const char* key);
    void DeleteValue(RegistryHive hive, const char* path, const char* key);

    void LoadFromDisk(const char* filePath);
    void SaveToDisk(const char* filePath);

private:
   sigma_u32 _entries_sharded = 0;
};

} // namespace WindowsShard
} // namespace SigmaOS

#endif
