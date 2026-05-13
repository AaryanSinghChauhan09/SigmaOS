# Contribution Guide

1

> How to add new modules, tools, shaders, and features to SigmaOS.

---

1

1

Have an idea?
    ¦
    +- New kernel primitive?  ? suites/S01_Genesis/sigma_<name>.h
    +- New hardware driver?   ? suites/S04_HAL/sigma_<name>.hpp
    +- New security module?   ? suites/S08_Security/sigma_<name>.h
    +- New network feature?   ? suites/S37_SovereignWire/sigma_<name>.h
    +- New perf optimization? ? suites/S28_PerformanceLattice/sigma_<name>.h
    +- New OOP driver class?  ? sigmaos/core/src/atomic_sigma_<name>_oop.cpp
    +- New CLI command?       ? orchestrator/main.cpp ? add ICommand subclass

1

---

1

1

Find the right `suites/S<NN>_*` directory: | Suite Range | Domain | |-------------|--------| | S01–S10 | Kernel core (allocator, HAL, IPC, security) | | S11–S20 | System services (VFS, networking, process mgmt) | | S21–S30 | Performance (NUMA, cache, BPF, containers) | | S31–S40 | Storage, immutability, ZKP, sovereign wire | | S41–S50 | Boot, self-healing, caps, mesh | | S51–S65 | Applications, AI, developer tools | ---

1

1

1

1

touch suites/S37_SovereignWire/sigma_vpn.h

1

1

Follow the **Atomic Module Contract** (see Developer Guide):

1

1

1

Edit `orchestrator/main.cpp`, find `TestCommand::run_subsystem_test()`:

1

static void run_subsystem_test(const char* subsystem) {
    // Add your new subsystem here:
    std::cout << "[?] " << subsystem << " ? All shards passed.\n";
}

1

1

1

// In orchestrator/main.cpp, add a new ICommand subclass:
class VPNCommand : public ICommand {
public:
    bool matches(const std::string& cmd) const override { return cmd == "vpn"; }
    int execute(int argc, char** argv) const override {
        std::cout << "[SigmaOS] VPN tunnel established via Sigma-Net.\n";
        return 0;
    }
};
// Then register in CommandDispatcher constructor

1

1

1

g++ -std=c++20 orchestrator/main.cpp -o s-cli
./s-cli test --subsystem networking

1

1

1

git add suites/S37_SovereignWire/sigma_vpn.h orchestrator/main.cpp
git commit -m "feat: Add sigma_vpn atomic networking module"
git push origin main

1

CI will automatically run all 4 workflows and report results.

---

1

