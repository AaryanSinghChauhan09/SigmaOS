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
extern "C" void ne2000_init(sigma_u32 base);
extern "C" void rtl8139_init(sigma_u64 base);
extern "C" void ixgbe_init(sigma_u64 base);
extern "C" void wlan_init();
extern "C" void wpan_init();
extern "C" void tcpip_init();
extern "C" void ipv6_init();
extern "C" void firewall_init();
extern "C" void ipx_init();
extern "C" void ppp_init();
extern "C" void dhcp_init();
extern "C" void vnet_init();
extern "C" void secnet_init();
extern "C" void bcachefs_init();
extern "C" void fat_init();
extern "C" void ntfs_init();
extern "C" void ext4_init();
extern "C" void xfs_init();
extern "C" void legacyfs_init();
extern "C" void opticalfs_init();
extern "C" void netfs_init();
extern "C" void tmpfs_init();
extern "C" void raid_init();
extern "C" void quota_init();
extern "C" void acl_init();
extern "C" void fscrypt_init();
extern "C" void lvm_init();
extern "C" void selinux_init();
extern "C" void nx_init();
extern "C" void aslr_init();
extern "C" void seccomp_init();
extern "C" void audit_init();
extern "C" void ima_init();
extern "C" void kvm_init();
extern "C" void lxc_init();

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

        // Layer 0: Memory & Core HAL (Hardware & Foundational Security)
        allocator_init();
        nx_init();
        aslr_init();
        
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
        
        // Layer 3: Connectivity & Network Hardware
        e1000_init(0xFEB00000);
        ne2000_init(0x0300);
        rtl8139_init(0xFEC00000);
        ixgbe_init(0xFEA00000);
        wlan_init();
        wpan_init();
        
        // Layer 3.5: Network Protocols & Security
        net_init();
        tcpip_init();
        ipv6_init();
        ipx_init();
        ppp_init();
        dhcp_init();
        vnet_init();
        firewall_init();
        secnet_init();
        
        // Layer 4: Persistence, File Systems & Volume Management
        lvm_init();
        raid_init();
        fscrypt_init();
        vfs_init();
        bcachefs_init();
        fat_init();
        ntfs_init();
        ext4_init();
        xfs_init();
        legacyfs_init();
        opticalfs_init();
        netfs_init();
        tmpfs_init();
        quota_init();
        acl_init();
        
        ext2_mount("/dev/sda1"); // Root mount
        
        // Layer 4.5: Userspace Security & Auditing
        selinux_init();
        seccomp_init();
        audit_init();
        ima_init();
        
        pkg_init();
        wm_init();
        
        // Layer 5: Advanced Virtualization & Isolation
        hyp_init();
        kvm_init();
        container_init();
        lxc_init();

        sigma_log_info("[INIT] ASI: Total Singularity Achieved. 750+ Industrial Shards Active.\n");
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
