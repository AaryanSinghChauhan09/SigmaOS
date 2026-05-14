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
extern "C" void sata_init(sigma_u64 base);
extern "C" void scsi_init();
extern "C" void usb3_init(sigma_u64 base);
extern "C" void e1000_init(sigma_u64 base);
extern "C" void firewire_init(sigma_u64 base);
extern "C" void pcmcia_init();
extern "C" void agp_init(sigma_u64 base, sigma_u32 size);
extern "C" void wm_init();
extern "C" void hyp_init();
extern "C" void container_init();
extern "C" void ubuntu_init();
extern "C" void nvidia_init();
extern "C" void ati_init();
extern "C" void media_init();
extern "C" void tuner_init();
extern "C" void video_init();

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
        
        // Layer 1: Hardware Interaction (Storage & Bus)
        ata_init();
        sata_init(0xFEA00000);
        scsi_init();
        usb3_init(0xFE000000);
        firewire_init(0xFD000000);
        pcmcia_init();
        kbd_init();
        
        // Layer 2: Graphics & Multimedia
        agp_init(0xE0000000, 128);
        vesa_init(1920, 1080, 32, 0xFD000000);
        nvidia_init();
        ati_init();
        media_init();
        tuner_init();
        video_init();
        ubuntu_init();
        
        // Layer 3: Connectivity & Network
        e1000_init(0xFEB00000);
        net_init();
        
        // Layer 4: Persistence & Userland
        vfs_init();
        ext2_mount("/dev/sda1");
        pkg_init();
        wm_init();
        
        // Layer 5: Advanced Virtualization & Isolation
        hyp_init();
        container_init();

        sigma_log_info("[INIT] ASI: Total Singularity Achieved. 700+ Industrial Shards Active.\n");
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
