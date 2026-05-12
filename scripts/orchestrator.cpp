/**
 * @file orchestrator.cpp
 * @brief SigmaOS Sovereign Build Orchestrator v4.1 (Industrial Native)
 * - Declarative Feature-Flag Integration
 * - Multi-Arch Matrix Support (x86_64, AArch64, RISC-V)
 * - Topological Shard Resolution
 */

#include <iostream>
#include <vector>
#include <string>
#include <filesystem>
#include <fstream>
#include <map>
#include <set>
#include <algorithm>
#include <cstdlib>
#include <chrono>

namespace fs = std::filesystem;

struct Module {
    std::string name;
    std::string path;
    std::vector<std::string> dependencies;
    std::vector<std::string> sources;
    bool enabled = true;
};

// Simplified JSON/Config parsing
std::vector<std::string> parse_array(const std::string& content, const std::string& key) {
    std::vector<std::string> results;
    size_t pos = content.find("\"" + key + "\"");
    if (pos == std::string::npos) return results;

    pos = content.find("[", pos);
    if (pos == std::string::npos) return results;

    size_t end = content.find("]", pos);
    std::string array_content = content.substr(pos + 1, end - pos - 1);

    size_t start = 0;
    while ((start = array_content.find("\"", start)) != std::string::npos) {
        size_t next = array_content.find("\"", start + 1);
        if (next == std::string::npos) break;
        results.push_back(array_content.substr(start + 1, next - start - 1));
        start = next + 1;
    }
    return results;
}

std::string parse_string(const std::string& content, const std::string& key) {
    size_t pos = content.find("\"" + key + "\"");
    if (pos == std::string::npos) return "";

    pos = content.find(":", pos);
    pos = content.find("\"", pos);
    if (pos == std::string::npos) return "";

    size_t end = content.find("\"", pos + 1);
    return content.substr(pos + 1, end - pos - 1);
}

class BuildOrchestrator {
public:
    bool verbose = false;

    void load_features(const std::string& path) {
        if (!fs::exists(path)) return;
        std::ifstream f(path);
        std::string content((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
        
        // Simple feature detection
        if (content.find("\"networking\": true") != std::string::npos) active_features.insert("networking");
        if (content.find("\"persistence\": true") != std::string::npos) active_features.insert("persistence");
        if (content.find("\"intelligence\": true") != std::string::npos) active_features.insert("intelligence");
    }

    void discover_modules(const std::string& dir) {
        if (!fs::exists(dir)) return;
        for (const auto& entry : fs::recursive_directory_iterator(dir)) {
            if (entry.path().filename() == "module.json") {
                std::ifstream f(entry.path());
                std::string content((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
                
                Module mod;
                mod.name = parse_string(content, "module");
                mod.path = entry.path().parent_path().string();
                mod.dependencies = parse_array(content, "dependencies");
                
                // Feature-based exclusion
                if (mod.name == "S10_Networking" && !active_features.count("networking")) mod.enabled = false;
                if (mod.name == "S06_Persistence" && !active_features.count("persistence")) mod.enabled = false;

                if (mod.enabled) {
                    for (const auto& p : fs::directory_iterator(mod.path)) {
                        if (p.path().extension() == ".c" || p.path().extension() == ".cpp" || p.path().extension() == ".asm" || p.path().extension() == ".s") {
                            mod.sources.push_back(p.path().string());
                        }
                    }
                    modules[mod.name] = mod;
                }
            }
        }
    }

    void build(const std::string& arch) {
        std::cout << "[Î£] Target Architecture: " << arch << "\n";
        
        std::vector<std::string> build_order;
        std::set<std::string> visited;
        std::set<std::string> in_stack;

        for (const auto& [name, mod] : modules) {
            topological_sort(name, visited, in_stack, build_order);
        }

        std::vector<std::string> all_objects;
        for (const auto& name : build_order) {
            auto objs = build_module(modules[name], arch);
            all_objects.insert(all_objects.end(), objs.begin(), objs.end());
        }

        link_image(all_objects, arch);
    }

    void clean() {
        std::cout << "[*] Cleaning lattice artifacts...\n";
        if (fs::exists("build")) fs::remove_all("build");
        std::cout << "[âœ"] Lattice is clean.\n";
    }

private:
    std::map<std::string, Module> modules;
    std::set<std::string> active_features;

    void topological_sort(const std::string& name, std::set<std::string>& visited, std::set<std::string>& in_stack, std::vector<std::string>& order) {
        if (visited.count(name)) return;
        visited.insert(name);
        
        for (const auto& dep : modules[name].dependencies) {
            if (modules.count(dep)) {
                topological_sort(dep, visited, in_stack, order);
            }
        }
        order.push_back(name);
    }

    bool needs_rebuild(const std::string& src, const std::string& obj) {
        if (!fs::exists(obj)) return true;
        return fs::last_write_time(src) > fs::last_write_time(obj);
    }

    std::vector<std::string> build_module(const Module& mod, const std::string& arch) {
        std::cout << "  -> Shard: " << mod.name << "\n";
        std::vector<std::string> objs;
        
        std::string cc = "gcc";
        std::string base_cflags = "-nostdlib -ffreestanding -O2 -I. -Iinclude -Isuites/S01_Genesis";
        
        if (arch == "aarch64") {
            cc = "aarch64-linux-gnu-gcc";
            base_cflags += " -march=armv8-a";
        } else if (arch == "riscv64") {
            cc = "riscv64-linux-gnu-gcc";
            base_cflags += " -march=rv64imac -mabi=lp64";
        } else {
            base_cflags += " -m64";
        }
        
        for (const auto& src : mod.sources) {
            std::string obj = "build/" + fs::path(src).stem().string() + ".o";
            objs.push_back(obj);

            if (!needs_rebuild(src, obj)) continue;

            std::string cmd;
            if (src.ends_with(".asm") || src.ends_with(".s")) {
                cmd = "nasm -f elf64 " + src + " -o " + obj;
            } else {
                cmd = cc + " " + base_cflags + " -c " + src + " -o " + obj;
            }
            
            if (verbose) std::cout << "     [CC] " << cmd << "\n";
            if (std::system(cmd.c_str()) != 0) {
                std::cerr << "\n[!] Build error in " << src << "\n";
                std::exit(1);
            }
        }
        return objs;
    }

    void link_image(const std::vector<std::string>& objects, const std::string& arch) {
        std::cout << "[*] Linking Sovereign Lattice Image...\n";
        std::string out = "build/sigmaos_" + arch + ".bin";
        std::string ld = (arch == "aarch64") ? "aarch64-linux-gnu-ld" : 
                         (arch == "riscv64") ? "riscv64-linux-gnu-ld" : "ld";
        
        std::string cmd = ld + " -T linker.ld -o " + out;
        for (const auto& obj : objects) cmd += " " + obj;
        
        if (std::system(cmd.c_str()) == 0) {
            std::cout << "[âœ"] Sovereign Image: " << out << "\n";
        } else {
            std::cerr << "[âœ—] Linking failed.\n";
            std::exit(1);
        }
    }
};

int main(int argc, char** argv) {
    std::string cmd = (argc > 1) ? argv[1] : "build";
    std::string arch = (argc > 2) ? argv[2] : "x86_64";
    
    if (!fs::exists("build")) fs::create_directories("build");

    BuildOrchestrator orch;
    orch.load_features("sigma_features.json");
    orch.discover_modules("modules");
    orch.discover_modules("suites");

    if (cmd == "build") {
        auto start = std::chrono::high_resolution_clock::now();
        orch.build(arch);
        auto end = std::chrono::high_resolution_clock::now();
        std::chrono::duration<double> diff = end - start;
        std::cout << "[Î£] Lattice synchronized in " << diff.count() << "s.\n";
    } else if (cmd == "clean") {
        orch.clean();
    } else {
        std::cerr << "[!] Unknown orchestrator command: " << cmd << "\n";
        return 1;
    }

    return 0;
}
