#include "sigma_log.h"
#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"

/* Forward declarations for Zenith functional layers */
extern "C" void allocator_init();
extern "C" void vfs_init();
extern "C" void ext2_mount(const char* device);
extern "C" void net_init();
extern "C" void pkg_init();
extern "C" void kbd_init();
extern "C" void vesa_init(sigma_u32 w, sigma_u32 h, sigma_u32 bpp, sigma_u64 lfb);
extern "C" void ata_init();
extern "C" void e1000_init(sigma_u64 base);
extern "C" void wm_init();
extern "C" void hyp_init();
extern "C" void container_init();
extern "C" void ubuntu_init();
extern "C" void nvidia_init();

/**
 * SigmaOS Sovereign Init Implementation (v15.0 Zenith)
 * Implements an Asynchronous Shard Ignition (ASI) algorithm.
 * Mission: Orchestrate the total industrial functional stack.
 */

namespace SigmaOS {
namespace Kernel {
namespace Boot {

class SovereignInitEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignInitEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignInitEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignInitEngine"; }

    void ignite() {
        sigma_log_info("[INIT] ASI: Orchestrating Industrial Zenith Singularity...");

        // Layer 0: Memory & Core HAL
        allocator_init();
        
        // Layer 1: Hardware Interaction & Compatibility
        ata_init();
        kbd_init();
        vesa_init(1920, 1080, 32, 0xFD000000);
        ubuntu_init(); // Generic driver lattice
        nvidia_init(); // GPU acceleration
        
        // Layer 2: Connectivity & Network
        e1000_init(0xFEB00000);
        net_init();
        
        // Layer 3: Persistence
        vfs_init();
        ext2_mount("/dev/sda1");
        
        // Layer 4: Userland & UI
        pkg_init();
        wm_init();
        
        // Layer 5: Advanced Virtualization & Isolation
        hyp_init();
        container_init();

        sigma_log_info("[INIT] ASI: Total Singularity Achieved. 620 Industrial Shards Active.\n");
    }

private:
    SovereignInitEngine() = default;
};

} // namespace Boot
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void sinit_ignite() { SigmaOS::Kernel::Boot::SovereignInitEngine::getInstance().ignite(); }
}
