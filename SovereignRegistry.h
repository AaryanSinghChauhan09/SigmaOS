#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

#include "SigmaOOP.hpp"
#include <map>
#include <string>
#include <vector>

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

    void SetValue(RegistryHive hive, const std::string& path, const std::string& key, const std::string& value);
    std::string GetValue(RegistryHive hive, const std::string& path, const std::string& key);
    void DeleteValue(RegistryHive hive, const std::string& path, const std::string& key);

    void LoadFromDisk(const std::string& filePath);
    void SaveToDisk(const std::string& filePath);

private:
    std::map<RegistryHive, std::map<std::string, std::map<std::string, std::string>>> _storage;
};

} // namespace WindowsShard
} // namespace SigmaOS

#endif
