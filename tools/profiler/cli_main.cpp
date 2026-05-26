#include <iostream>
#include <string>

extern "C" {
    void autotune_profile(const char* profile);
    void autotune_reset();
    void analyze_telemetry();
}

void print_help() {
    std::cout << "SigmaOS Performance Profiler (sigma-prof)\n";
    std::cout << "Usage:\n";
    std::cout << "  sigma-prof analyze       - Gather advanced PMU telemetry and bottlenecks\n";
    std::cout << "  sigma-prof tune <prof>   - Apply optimization profile (hpc, ai, embedded)\n";
    std::cout << "  sigma-prof reset         - Revert to declarative baseline performance\n";
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    std::string cmd = argv[1];

    if (cmd == "analyze") {
        analyze_telemetry();
        return 0;
    } 
    else if (cmd == "tune") {
        if (argc < 3) {
            std::cout << "[sigma-prof] Error: Please specify a profile (e.g. hpc, ai, embedded).\n";
            return 1;
        }
        autotune_profile(argv[2]);
        return 0;
    } 
    else if (cmd == "reset") {
        autotune_reset();
        return 0;
    }
    else {
        std::cout << "[sigma-prof] Unknown command: " << cmd << "\n";
        print_help();
        return 1;
    }
}
