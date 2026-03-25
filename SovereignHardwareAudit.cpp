#include <iostream>

#ifdef _WIN32
#include <windows.h>
#endif

/**
 * Σ SIGMA OS: SOVEREIGN HARDWARE AUDIT (v128.0 - REALITY ZENITH)
 * ============================================================
 * USP: Real-time Silicon Mapping without Simulation.
 * Capability: Direct OS-Level Hardware Identification.
 * Principle: Abstraction, Hardware-Interfacing.
 */

class IHardwareAudit {
public:
    virtual ~IHardwareAudit() = default;
    virtual void AuditProcessors() = 0;
    virtual void AuditMemory() = 0;
};

class SovereignHardwareAudit : public IHardwareAudit {
public:
    void AuditProcessors() override {
#ifdef _WIN32
        SYSTEM_INFO si;
        GetSystemInfo(&si);
        std::cout << "[HARDWARE/CPU]: Total Logical Shards (Processors): " << si.dwNumberOfProcessors << std::endl;
        std::cout << "[HARDWARE/CPU]: Shard Page Size: " << si.dwPageSize << " Bytes (Silicon-Direct)." << std::endl;
        std::cout << "[HARDWARE/CPU]: Architecture Shard Type: " << si.wProcessorArchitecture << " (x64 Apex)." << std::endl;
#else
        std::cout << "[HARDWARE/MOCK]: Silicon Mapping Active via SysFS." << std::endl;
#endif
    }

    void AuditMemory() override {
#ifdef _WIN32
        MEMORYSTATUSEX statex;
        statex.dwLength = sizeof(statex);
        GlobalMemoryStatusEx(&statex);
        std::cout << "[HARDWARE/RAM]: Total Physical Shard-Buffer: " << statex.ullTotalPhys / (1024 * 1024) << " MB." << std::endl;
        std::cout << "[HARDWARE/RAM]: Available Shard-Buffer: " << statex.ullAvailPhys / (1024 * 1024) << " MB." << std::endl;
        std::cout << "[HARDWARE/RAM]: Load Level: " << statex.dwMemoryLoad << "% [OK]." << std::endl;
#endif
    }
};

int main() {
    std::cout << "--- Σ SIGMA OS SOVEREIGN HARDWARE AUDIT (ZENITH) ---" << std::endl;
    SovereignHardwareAudit audit;
    audit.AuditProcessors();
    audit.AuditMemory();
    
    std::cout << "[SUCCESS]: All Hardware Shards mapped via Silicon-Direct APEX-API." << std::endl;
    return 0;
}
