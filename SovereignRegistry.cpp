#include "SovereignRegistry.h"
#include <iostream>
#include <fstream>

namespace SigmaOS {
namespace WindowsShard {

void SovereignRegistry::SetValue(RegistryHive hive, const std::string& path, const std::string& key, const std::string& value) {
    _storage[hive][path][key] = value;
    std::cout << "[REGISTRY] Set value [" << key << "] under " << path << " to " << value << std::endl;
}

std::string SovereignRegistry::GetValue(RegistryHive hive, const std::string& path, const std::string& key) {
    if (_storage.count(hive) && _storage[hive].count(path) && _storage[hive][path].count(key)) {
        return _storage[hive][path][key];
    }
    return "";
}

void SovereignRegistry::DeleteValue(RegistryHive hive, const std::string& path, const std::string& key) {
    if (_storage.count(hive) && _storage[hive].count(path)) {
        _storage[hive][path].erase(key);
        std::cout << "[REGISTRY] Deleted value [" << key << "] from " << path << std::endl;
    }
}

void SovereignRegistry::SaveToDisk(const std::string& filePath) {
    std::ofstream out(filePath);
    for (auto const& hive_pair : _storage) {
        auto const& hive = hive_pair.first;
        auto const& paths = hive_pair.second;
        for (auto const& path_pair : paths) {
            auto const& path = path_pair.first;
            auto const& keys = path_pair.second;
            out << "[HIVE:" << static_cast<int>(hive) << "][PATH:" << path << "]" << std::endl;
            for (auto const& key_pair : keys) {
                out << key_pair.first << "=" << key_pair.second << std::endl;
            }
        }
    }
    out.close();
    std::cout << "[REGISTRY] Saved to " << filePath << std::endl;
}

void SovereignRegistry::LoadFromDisk(const std::string& filePath) {
    std::ifstream in(filePath);
    std::string line;
    RegistryHive currentHive = RegistryHive::LOCAL_MACHINE;
    std::string currentPath = "Root";

    while (std::getline(in, line)) {
        if (line.empty()) continue;
        if (line[0] == '[') {
            // Very basic header parsing (could be improved)
            if (line.find("HIVE:0") != std::string::npos) currentHive = RegistryHive::LOCAL_MACHINE;
            else if (line.find("HIVE:1") != std::string::npos) currentHive = RegistryHive::CURRENT_USER;
            else if (line.find("HIVE:2") != std::string::npos) currentHive = RegistryHive::SYSTEM;
            else if (line.find("HIVE:3") != std::string::npos) currentHive = RegistryHive::SOFTWARE;
            
            size_t pathPos = line.find("PATH:");
            if (pathPos != std::string::npos) {
                currentPath = line.substr(pathPos + 5, line.length() - pathPos - 6);
            }
        } else {
            size_t sep = line.find('=');
            if (sep != std::string::npos) {
                std::string k = line.substr(0, sep);
                std::string v = line.substr(sep + 1);
                _storage[currentHive][currentPath][k] = v;
            }
        }
    }
    in.close();
    std::cout << "[REGISTRY] Loaded " << filePath << " into memory." << std::endl;
}

} // namespace WindowsShard
} // namespace SigmaOS
