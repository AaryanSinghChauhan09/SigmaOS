/**
 * @file orchestrator.cpp
 * @brief SigmaOS Sovereign Build Orchestrator (Industrial Native Implementation)
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
};

// Simplified JSON parsing
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

    pos = content.find("\"", pos + key.length() + 2);
    if (pos == std::string::npos) return "";

    size_t end = content.find("\"", pos + 1);
    return content.substr(pos + 1, end - pos - 1);
}

class BuildOrchestrator {
public:
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
                
                for (const auto& p : fs::directory_iterator(mod.path)) {
                    if (p.path().extension() == ".c" || p.path().extension() == ".cpp" || p.path().extension() == ".asm" || p.path().extension() == ".s") {
                        mod.sources.push_back(p.path().string());
                    }
                }
                modules[mod.name] = mod;
            }
        }
    }

    void build(const std::string& arch) {
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
        std::cout << "[*] Cleaning build artifacts...\n";
        if (fs::exists("build")) {
            fs::remove_all("build");
        }
        std::cout << "[✓] Clean complete.\n";
    }

    void scaffold_shard(const std::string& name) {
        std::cout << "[*] Scaffolding new shard: " << name << "\n";
        std::string path = "suites/" + name;
        if (fs::exists(path)) {
            std::cerr << "[!] Shard already exists at " << path << "\n";
            return;
        }

        fs::create_directories(path);
        
        std::ofstream f(path + "/module.json");
        f << "{\n  \"module\": \"" << name << "\",\n  \"dependencies\": []\n}\n";
        f.close();

        std::ofstream src(path + "/shard_init.c");
        src << "#include \"sigma_libc.h\"\n\nvoid shard_init() {\n    sigma_printf(\"[SHARD] " << name << " initialized.\\n\");\n}\n";
        src.close();

        std::cout << "[✓] Shard scaffolded successfully.\n";
    }

private:
    std::map<std::string, Module> modules;

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
        std::cout << "[*] Module: " << mod.name << "\n";
        std::vector<std::string> objs;
        
        std::string base_cflags = "-nostdlib -ffreestanding -O2 -Wall -I. -Iinclude -Isuites/S01_Genesis/include";
        if (arch == "x86_64") base_cflags += " -m64";
        
        for (const auto& src : mod.sources) {
            std::string obj = "build/" + fs::path(src).filename().string() + ".o";
            objs.push_back(obj);

            if (!needs_rebuild(src, obj)) {
                std::cout << "    [SKIP] " << fs::path(src).filename() << "\n";
                continue;
            }

            std::string cmd;
            if (src.ends_with(".asm") || src.ends_with(".s")) {
                cmd = "nasm -f elf64 " + src + " -o " + obj;
            } else {
                cmd = "gcc " + base_cflags + " -c " + src + " -o " + obj;
            }
            
            std::cout << "    [CC]   " << fs::path(src).filename() << "\n";
            if (std::system(cmd.c_str()) != 0) {
                std::cerr << "    [ERR] Failed to build " << src << "\n";
                std::exit(1);
            }
        }
        return objs;
    }

    void link_image(const std::vector<std::string>& objects, const std::string& arch) {
        std::cout << "\n[*] Linking Sovereign Kernel Image...\n";
        std::string out = "build/sigmaos_" + arch + ".bin";
        std::string cmd = "ld -T linker.ld -o " + out;
        for (const auto& obj : objects) cmd += " " + obj;
        
        if (std::system(cmd.c_str()) == 0) {
            std::cout << "[✓] Kernel image ready: " << out << "\n";
        } else {
            std::cerr << "[✗] Linking failed.\n";
            std::exit(1);
        }
    }
};

int main(int argc, char** argv) {
    std::string cmd = (argc > 1) ? argv[1] : "build";
    std::string arch = (argc > 2) ? argv[2] : "x86_64";
    
    fs::create_directories("build");

    BuildOrchestrator orch;
    orch.discover_modules("modules");
    orch.discover_modules("suites");

    if (cmd == "build") {
        auto start = std::chrono::high_resolution_clock::now();
        orch.build(arch);
        auto end = std::chrono::high_resolution_clock::now();
        std::chrono::duration<double> diff = end - start;
        std::cout << "\n[Σ] Build successful in " << diff.count() << "s.\n";
    } else if (cmd == "clean") {
        orch.clean();
    } else if (cmd == "list") {
        orch.list();
    } else if (cmd == "scaffold") {
        if (argc < 3) {
            std::cerr << "Usage: orchestrator scaffold <shard_name>\n";
            return 1;
        }
        orch.scaffold_shard(argv[2]);
    } else {
        std::cerr << "[!] Unknown command: " << cmd << "\n";
        return 1;
    }

    return 0;
}
