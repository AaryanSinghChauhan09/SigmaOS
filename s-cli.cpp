#include <iostream>
#include <string>
#include <vector>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <thread>
#include <chrono>

namespace fs = std::filesystem;

void print_header(const std::string& text) {
    std::cout << "\n\033[95m\033[1m=== " << text << " ===\033[0m\n";
}

int run_command(const std::string& cmd) {
    return std::system(cmd.c_str());
}

void ensure_native_orch() {
    if (!fs::exists("scripts/orchestrator")) {
        std::cout << "[*] Compiling native orchestrator...\n";
        run_command("g++ -std=c++20 scripts/orchestrator.cpp -o scripts/orchestrator");
    }
}

void ensure_native_test() {
    if (!fs::exists("scripts/test_runner")) {
        std::cout << "[*] Compiling native test runner...\n";
        run_command("g++ -std=c++20 scripts/test_runner.cpp -o scripts/test_runner");
    }
}

void build(int argc, char** argv) {
    print_header("Initializing Sovereign Build");
    ensure_native_orch();
    
    std::string cmd = "./scripts/orchestrator build";
    if (argc > 2) {
        cmd += " ";
        cmd += argv[2]; // Architecture
    }
    
    if (run_command(cmd) == 0) {
        std::cout << "\033[92m[✓] Native build successful!\033[0m\n";
    } else {
        std::cout << "\033[91m[✗] Native build failed.\033[0m\n";
    }
}

void clean() {
    print_header("Cleaning Lattice Shards");
    ensure_native_orch();
    run_command("./scripts/orchestrator clean");
}

void list_modules() {
    print_header("SigmaOS Sovereign Modules");
    ensure_native_orch();
    run_command("./scripts/orchestrator list");
}

void test() {
    print_header("Running Sovereign Lattice Tests");
    ensure_native_test();
    run_command("./scripts/test_runner");
}

void info() {
    print_header("SigmaOS System Info");
    std::ifstream f("sigma_features.json");
    if (f.is_open()) {
        std::string line;
        while (std::getline(f, line)) {
            std::cout << "  " << line << "\n";
        }
    } else {
        std::cout << "  [ERR] sigma_features.json not found.\n";
    }
}

void handle_profile(int argc, char** argv) {
    std::string profile_dir = "meta/profiles";
    if (argc < 3) {
        print_header("Available Profiles");
        for (const auto& entry : fs::directory_iterator(profile_dir)) {
            if (entry.path().extension() == ".json") {
                std::cout << "  - " << entry.path().stem().string() << "\n";
            }
        }
        return;
    }

    std::string profile_name = argv[2];
    std::string src = profile_dir + "/" + profile_name + ".json";
    if (fs::exists(src)) {
        fs::copy_file(src, "sigma_features.json", fs::copy_options::overwrite_existing);
        std::cout << "\033[92m[✓] Switched to profile: " << profile_name << "\033[0m\n";
    } else {
        std::cout << "\033[91m[✗] Profile '" << profile_name << "' not found.\033[0m\n";
    }
}

void scaffold(int argc, char** argv) {
    if (argc < 3) {
        std::cout << "Usage: s-cli scaffold <shard_name>\n";
        return;
    }
    std::string shard_name = argv[2];
    std::string path = "suites/" + shard_name;
    if (fs::exists(path)) {
        std::cout << "\033[91m[✗] Shard '" << shard_name << "' already exists.\033[0m\n";
        return;
    }

    fs::create_directories(path);
    std::ofstream f(path + "/module.json");
    f << "{\n  \"module\": \"" << shard_name << "\",\n  \"dependencies\": []\n}\n";
    f.close();

    std::ofstream src(path + "/shard_init.c");
    src << "#include \"sigma_libc.h\"\n\nvoid shard_init() {\n    sigma_printf(\"[SHARD] " << shard_name << " initialized.\\n\");\n}\n";
    src.close();

    std::cout << "\033[92m[✓] Scaffolded shard: " << path << "\033[0m\n";
}

void run_qemu(int argc, char** argv) {
    print_header("Booting SigmaOS in QEMU");
    std::string arch = (argc > 2) ? argv[2] : "x86_64";
    std::string iso_path = "build/sigmaos_" + arch + ".iso";
    
    if (!fs::exists(iso_path)) {
        std::cout << "\033[93m[!] ISO not found at " << iso_path << ". Building first...\033[0m\n";
        build(argc, argv);
    }
    
    std::string qemu_cmd = "qemu-system-" + arch + " -cdrom " + iso_path + " -m 512M -serial stdio -no-reboot";
    std::cout << "[*] Command: " << qemu_cmd << "\n";
    run_command(qemu_cmd);
}

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cout << "SigmaOS Sovereign Native CLI\n";
        std::cout << "Commands:\n";
        std::cout << "  build [arch]  - Build the kernel\n";
        std::cout << "  clean         - Clean artifacts\n";
        std::cout << "  list          - List modules\n";
        std::cout << "  test          - Run tests\n";
        std::cout << "  run [arch]    - Run in QEMU\n";
        std::cout << "  info          - Show system info\n";
        std::cout << "  profile [name]- List or switch profiles\n";
        std::cout << "  scaffold [n]  - Create new shard\n";
        return 0;
    }

    std::string cmd = argv[1];

    if (cmd == "build") {
        build(argc, argv);
    } else if (cmd == "clean") {
        clean();
    } else if (cmd == "list") {
        list_modules();
    } else if (cmd == "test") {
        test();
    } else if (cmd == "run") {
        run_qemu(argc, argv);
    } else if (cmd == "info") {
        info();
    } else if (cmd == "profile") {
        handle_profile(argc, argv);
    } else if (cmd == "scaffold") {
        scaffold(argc, argv);
    } else {
        std::cout << "[!] Unknown command: " << cmd << "\n";
    }

    return 0;
}
