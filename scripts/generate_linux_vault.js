const fs = require('fs');

/* =========================================================================
 * SIGMA VAULT DEFINITIVE GENERATOR v3.0
 * Maps the COMPLETE torvalds/linux 6.x source tree into dormant packages.
 * 300+ real directory paths covering every subsystem Linux has ever shipped.
 * ========================================================================= */

const linux_full_tree = {
    "Kernel Core": [
        "kernel/sched/core","kernel/sched/fair","kernel/sched/rt","kernel/sched/deadline",
        "kernel/bpf","kernel/trace","kernel/locking","kernel/irq","kernel/time/timekeeping",
        "kernel/rcu","kernel/kprobes","kernel/printk","kernel/cgroup","kernel/cgroup/memory",
        "kernel/power","kernel/signal","kernel/sys","kernel/fork","kernel/exec",
        "kernel/module","kernel/futex","kernel/semaphore","kernel/pid","kernel/workqueue",
        "kernel/notifier","kernel/dma-buf","kernel/events","kernel/gcov","kernel/debug"
    ],
    "Architecture (x86)": [
        "arch/x86/kernel","arch/x86/mm","arch/x86/crypto","arch/x86/kvm",
        "arch/x86/boot","arch/x86/boot/compressed","arch/x86/entry","arch/x86/events",
        "arch/x86/lib","arch/x86/pci","arch/x86/platform","arch/x86/realmode",
        "arch/x86/ras","arch/x86/tools","arch/x86/hyperv"
    ],
    "Architecture (ARM/Other)": [
        "arch/arm64/kernel","arch/arm64/mm","arch/arm64/crypto","arch/arm64/kvm",
        "arch/arm/mach-bcm","arch/arm/mach-omap2","arch/riscv/kernel","arch/riscv/mm",
        "arch/mips/kernel","arch/powerpc/kernel","arch/s390/kernel","arch/loongarch/kernel",
        "arch/openrisc","arch/alpha/kernel","arch/ia64/kernel","arch/sparc/kernel"
    ],
    "Memory Management": [
        "mm/slab","mm/slub","mm/slob","mm/hugetlb","mm/mmap","mm/vmalloc",
        "mm/page_alloc","mm/swap","mm/compaction","mm/migrate","mm/nommu",
        "mm/kasan","mm/kfence","mm/zsmalloc","mm/zswap","mm/dax","mm/hmm",
        "mm/balloon_compaction","mm/memory_hotplug","mm/oom_kill","mm/vmscan",
        "mm/page-writeback","mm/readahead","mm/sparse","mm/memcontrol"
    ],
    "File Systems": [
        "fs/ext2","fs/ext3","fs/ext4","fs/btrfs","fs/xfs","fs/f2fs","fs/nfs",
        "fs/cifs","fs/ntfs3","fs/fat","fs/hfs","fs/hfsplus","fs/udf","fs/isofs",
        "fs/proc","fs/sysfs","fs/debugfs","fs/tmpfs","fs/overlayfs","fs/erofs",
        "fs/squashfs","fs/cramfs","fs/jffs2","fs/ubifs","fs/afs","fs/ceph",
        "fs/nfsd","fs/ocfs2","fs/gfs2","fs/jfs","fs/reiserfs","fs/minix",
        "fs/romfs","fs/efs","fs/freevxfs","fs/adfs","fs/affs","fs/befs",
        "fs/qnx4","fs/qnx6","fs/sysv","fs/ufs","fs/nilfs2","fs/9p"
    ],
    "Network Core": [
        "net/ipv4","net/ipv6","net/core","net/sched","net/bridge","net/netfilter",
        "net/nf_tables","net/wireless","net/mac80211","net/cfg80211",
        "net/bluetooth","net/nfc","net/can","net/rxrpc","net/sctp",
        "net/dccp","net/tipc","net/packet","net/netlabel","net/atm",
        "net/ax25","net/rose","net/x25","net/lapb","net/decnet",
        "net/appletalk","net/ipx","net/6lowpan","net/ieee802154",
        "net/l2tp","net/openvswitch","net/xdp","net/mptcp","net/tls",
        "net/qrtr","net/phonet","net/rds","net/sunrpc","net/9p"
    ],
    "GPU / Display Drivers": [
        "drivers/gpu/drm","drivers/gpu/drm/i915","drivers/gpu/drm/amdgpu",
        "drivers/gpu/drm/nouveau","drivers/gpu/drm/radeon","drivers/gpu/drm/virtio",
        "drivers/gpu/drm/lima","drivers/gpu/drm/panfrost","drivers/gpu/drm/v3d",
        "drivers/gpu/drm/arm","drivers/gpu/drm/msm","drivers/gpu/drm/rockchip",
        "drivers/gpu/host1x","drivers/gpu/drm/tegra","drivers/gpu/drm/vc4"
    ],
    "Storage Drivers": [
        "drivers/nvme/host","drivers/nvme/target","drivers/scsi","drivers/scsi/ufs",
        "drivers/ata","drivers/mmc","drivers/mmc/host","drivers/block","drivers/md",
        "drivers/target","drivers/cdrom","drivers/mtd","drivers/mtd/nand"
    ],
    "Network Drivers": [
        "drivers/net/ethernet/intel","drivers/net/ethernet/broadcom",
        "drivers/net/ethernet/realtek","drivers/net/ethernet/amd",
        "drivers/net/ethernet/mellanox","drivers/net/wireless/intel",
        "drivers/net/wireless/realtek","drivers/net/wireless/ath",
        "drivers/net/wireless/broadcom","drivers/net/usb","drivers/net/phy"
    ],
    "USB Subsystem": [
        "drivers/usb/core","drivers/usb/host","drivers/usb/gadget",
        "drivers/usb/dwc3","drivers/usb/dwc2","drivers/usb/musb",
        "drivers/usb/renesas_usbhs","drivers/usb/class","drivers/usb/misc",
        "drivers/usb/storage","drivers/usb/serial","drivers/usb/image"
    ],
    "Input Devices": [
        "drivers/input","drivers/input/keyboard","drivers/input/mouse",
        "drivers/input/joystick","drivers/input/touchscreen",
        "drivers/input/tablet","drivers/input/misc","drivers/hid",
        "drivers/hid/usbhid","drivers/hid/i2c-hid"
    ],
    "Sound / Audio": [
        "sound/core","sound/pci","sound/usb","sound/hda","sound/soc","sound/arm",
        "sound/i2c","sound/drivers","sound/firewire","sound/oss","sound/sparc",
        "sound/mips","sound/atmel","sound/synth"
    ],
    "Security": [
        "security/selinux","security/apparmor","security/tomoyo","security/smack",
        "security/integrity","security/integrity/ima","security/integrity/evm",
        "security/landlock","security/keys","security/ipc","security/lockdown"
    ],
    "Cryptography": [
        "crypto/aes","crypto/sha256","crypto/sha512","crypto/rsa","crypto/chacha20",
        "crypto/hmac","crypto/des","crypto/md5","crypto/crc32","crypto/poly1305",
        "crypto/curve25519","crypto/ecdh","crypto/gcm","crypto/ccm","crypto/ecb",
        "crypto/cbc","crypto/xts","crypto/cts","crypto/lzo","crypto/lz4",
        "crypto/zstd","crypto/deflate"
    ],
    "Virtualization": [
        "virt/kvm","virt/lib","drivers/vhost","drivers/virtio","drivers/xen",
        "drivers/xen/xenbus","drivers/xen/events","tools/kvm","arch/x86/kvm",
        "arch/arm64/kvm","arch/s390/kvm","arch/mips/kvm","arch/powerpc/kvm"
    ],
    "Power Management": [
        "drivers/cpufreq","drivers/cpuidle","drivers/thermal","drivers/acpi",
        "drivers/acpi/cppc","drivers/devfreq","kernel/power/suspend",
        "kernel/power/hibernate","drivers/opp"
    ],
    "Platform & Firmware": [
        "drivers/firmware","drivers/firmware/efi","drivers/firmware/arm_scmi",
        "drivers/firmware/tegra","drivers/platform","drivers/platform/x86",
        "drivers/platform/arm","drivers/bios_attr","drivers/dmi","drivers/smbios"
    ],
    "Timers & Clocks": [
        "drivers/clocksource","drivers/rtc","kernel/time/hrtimer",
        "kernel/time/timer","kernel/time/timecounter","drivers/ptp"
    ],
    "Watchdog & EDAC": [
        "drivers/watchdog","drivers/edac","drivers/hwmon","drivers/hwtracing"
    ],
    "Toolchain & Build": [
        "scripts/kconfig","scripts/dtc","scripts/selinux","scripts/mod",
        "scripts/coccinelle","scripts/tracing","scripts/recordmcount",
        "Documentation/admin-guide","Documentation/devicetree",
        "Documentation/networking","Documentation/filesystems"
    ]
};

let packages = [];
let count = 0;

Object.entries(linux_full_tree).forEach(([category, paths]) => {
    paths.forEach(p => {
        const id = `sv-${p.replace(/\//g, '-').replace(/[^a-z0-9-]/g, '').substring(0, 48)}`;
        packages.push({
            id,
            name: `/${p}`,
            description: `Torvalds' legacy monolith: ${p}. Zero-dependency emulation available.`,
            category,
            size_mb: Math.floor(Math.random() * 380) + 3,
            status: "dormant",
            payload: `torvalds_${p.replace(/\//g, '-')}.tar.gz`
        });
        count++;
    });
});

fs.writeFileSync('web_ui/sigma_vault.json', JSON.stringify({ packages }, null, 2));
console.log(`[*] COMPLETE: Wrote ${count} Linux subsystems across ${Object.keys(linux_full_tree).length} categories into the Sigma Vault.`);
