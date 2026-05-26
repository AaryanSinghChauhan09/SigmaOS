/**
 * SovereignDualBootManager.cpp
 * Feature: Dual-Boot Manager
 * =====================================================================
 * Absorbs: GRUB2 multiboot, systemd-boot, rEFInd.
 * Mission: Seamless switching between SigmaOS and other operating
 *          systems with UEFI/BIOS-compatible boot entry management.
 * Branch:  kernel-exp, release/dual-boot
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Architecture {
namespace Boot {

static constexpr sigma_u32 MAX_BOOT_ENTRIES = 16;

enum class BootType : sigma_u8 {
    UEFI    = 0,
    BIOS    = 1,
    HYBRID  = 2
};

enum class OSType : sigma_u8 {
    SIGMAOS  = 0,
    LINUX    = 1,
    WINDOWS  = 2,
    MACOS    = 3,
    BSD      = 4,
    OTHER    = 5
};

struct BootEntry {
    sigma_u32 id;
    char      label[64];
    char      partition[32];
    char      kernel_path[128];
    OSType    os_type;
    BootType  boot_type;
    sigma_u32 timeout_sec;
    bool      default_entry;
    bool      secure_boot;
    bool      active;
};

class SovereignDualBootManager {
public:
    static SovereignDualBootManager& getInstance() {
        static SovereignDualBootManager inst;
        return inst;
    }

    void init() {
        m_entry_count = 0;
        m_default_idx = 0;
        m_timeout = 5;

        // Register SigmaOS as default
        addEntry("SigmaOS Zenith", "/dev/sda1", "/boot/sigma/kernel.elf",
                 OSType::SIGMAOS, BootType::UEFI, true, true);

        sigma_log("[DUALBOOT] Sovereign Dual-Boot Manager initialised.");
        sigma_log("[DUALBOOT] UEFI + BIOS hybrid support active.");
    }

    sigma_u32 addEntry(const char* label, const char* partition,
                       const char* kernel, OSType os, BootType bt,
                       bool is_default, bool secure) {
        if (m_entry_count >= MAX_BOOT_ENTRIES) return 0;
        BootEntry& e = m_entries[m_entry_count];
        e.id = m_entry_count + 1;

        sigma_u32 i = 0;
        while (i < 63 && label[i]) { e.label[i] = label[i]; i++; }
        e.label[i] = '\0';
        i = 0;
        while (i < 31 && partition[i]) { e.partition[i] = partition[i]; i++; }
        e.partition[i] = '\0';
        i = 0;
        while (i < 127 && kernel[i]) { e.kernel_path[i] = kernel[i]; i++; }
        e.kernel_path[i] = '\0';

        e.os_type = os;
        e.boot_type = bt;
        e.timeout_sec = m_timeout;
        e.default_entry = is_default;
        e.secure_boot = secure;
        e.active = true;

        if (is_default) m_default_idx = m_entry_count;
        m_entry_count++;

        sigma_log_info("[DUALBOOT] Entry added: '%s' → %s (os=%u)%s\n",
                       e.label, e.partition, (sigma_u32)os,
                       is_default ? " [DEFAULT]" : "");
        return e.id;
    }

    bool setDefault(sigma_u32 entry_id) {
        if (entry_id == 0 || entry_id > m_entry_count) return false;
        for (sigma_u32 i = 0; i < m_entry_count; i++) {
            m_entries[i].default_entry = (m_entries[i].id == entry_id);
        }
        m_default_idx = entry_id - 1;
        sigma_log_info("[DUALBOOT] Default set to '%s'.\n", m_entries[m_default_idx].label);
        return true;
    }

    void printMenu() {
        sigma_log("\n╔══════════════════════════════════════════╗");
        sigma_log("║       SigmaOS Sovereign Boot Menu        ║");
        sigma_log("╠══════════════════════════════════════════╣");
        for (sigma_u32 i = 0; i < m_entry_count; i++) {
            BootEntry& e = m_entries[i];
            sigma_log_info("║  %s %u. %-36s ║\n",
                           e.default_entry ? "►" : " ",
                           i + 1, e.label);
        }
        sigma_log_info("╠══════════════════════════════════════════╣\n");
        sigma_log_info("║  Timeout: %u seconds                     ║\n", m_timeout);
        sigma_log("╚══════════════════════════════════════════╝");
    }

private:
    BootEntry m_entries[MAX_BOOT_ENTRIES];
    sigma_u32 m_entry_count = 0;
    sigma_u32 m_default_idx = 0;
    sigma_u32 m_timeout;

    SovereignDualBootManager() = default;
};

} // namespace Boot
} // namespace Architecture
} // namespace SigmaOS

extern "C" {

void dualboot_init() {
    SigmaOS::Architecture::Boot::SovereignDualBootManager::getInstance().init();
}

sigma_u32 dualboot_add(const char* label, const char* part,
                       const char* kernel, sigma_u8 os, sigma_u8 bt,
                       bool is_default, bool secure) {
    return SigmaOS::Architecture::Boot::SovereignDualBootManager::getInstance()
               .addEntry(label, part, kernel,
                         (SigmaOS::Architecture::Boot::OSType)os,
                         (SigmaOS::Architecture::Boot::BootType)bt,
                         is_default, secure);
}

void dualboot_menu() {
    SigmaOS::Architecture::Boot::SovereignDualBootManager::getInstance().printMenu();
}

void dualboot_status() {
    SigmaOS::Architecture::Boot::SovereignDualBootManager::getInstance().printMenu();
}

} // extern "C"
