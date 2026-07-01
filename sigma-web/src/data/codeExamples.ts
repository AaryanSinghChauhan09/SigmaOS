// SPDX-License-Identifier: GPL-2.0-or-later
// codeExamples.ts — Code Explorer data for SigmaOS

export interface CodeFile {
  id: string;
  filename: string;
  description: string;
  language: string;
  code: string;
}

export interface CodeCategory {
  id: string;
  name: string;
  icon: string;
  files: CodeFile[];
}

export const codeCategories: CodeCategory[] = [
  {
    id: 'drivers',
    name: 'Driver Shards',
    icon: 'Cpu',
    files: [
      {
        id: 'nvme',
        filename: 'drivers/block/nvme_shard.cpp',
        description: 'NVMe controller driver shard — multi-queue, MSI-X, Admin+IO queues',
        language: 'cpp',
        code: `// NVMe driver shard — runs in user-space, registers with kernel bus
#include <drivers/driver_interface.h>

// NVMe submission queue entry (64 bytes)
struct nvme_sq_entry {
    uint32_t cdw0;   // OPC | FUSE | PSDT | CID
    uint32_t nsid;
    uint64_t mptr;
    uint64_t prp1;   // Physical Region Page 1
    uint64_t prp2;
    uint32_t cdw10, cdw11, cdw12, cdw13, cdw14, cdw15;
};

int nvme_read_lbas(uint32_t nsid, uint64_t slba,
                   uint16_t nlb, uint64_t buf_pa) {
    nvme_sq_entry cmd = {};
    cmd.cdw0  = NVME_OP_READ;
    cmd.nsid  = nsid;
    cmd.prp1  = buf_pa;
    cmd.cdw10 = (uint32_t)(slba);
    cmd.cdw11 = (uint32_t)(slba >> 32);
    cmd.cdw12 = (uint32_t)(nlb - 1); // 0-based
    uint16_t cid = nvme_submit(&g_io_q[0], &cmd);
    return nvme_poll_completion(&g_io_q[0], cid, 5000000);
}`,
      },
      {
        id: 'hotplug',
        filename: 'drivers/core/hotplug_manager.cpp',
        description: 'Driver hot-plug manager — detects USB/PCIe insertion and spawns driver shards',
        language: 'cpp',
        code: `// Hot-plug manager — matches devices to drivers and spawns shards
static const driver_match_t driver_table[] = {
    { 0xFFFF, 0xFFFF, 0x010802, "/sbin/drivers/nvme_shard",  "nvme"   },
    { 0xFFFF, 0xFFFF, 0x010601, "/sbin/drivers/ahci_shard",  "ahci"   },
    { 0xFFFF, 0xFFFF, 0x0C0330, "/sbin/drivers/xhci_shard",  "xhci"   },
    { 0x8086, 0x100E, 0xFFFFFF, "/sbin/drivers/e1000_shard", "e1000"  },
    { 0x10EC, 0x8168, 0xFFFFFF, "/sbin/drivers/r8169_shard", "r8169"  },
    { 0x1AF4, 0x1000, 0xFFFFFF, "/sbin/drivers/virtio_net",  "virtio" },
    { 0, 0, 0, NULL, NULL }
};

static void on_device_arrived(const hotplug_device_t *info) {
    const driver_match_t *match = match_driver(info);
    if (match) spawn_driver(dev, match);
    else printf("[hotplug] no driver for %s\\n", info->path);
}`,
      },
    ],
  },
  {
    id: 'storage',
    name: 'Block & Storage',
    icon: 'HardDrive',
    files: [
      {
        id: 'block_device',
        filename: 'include/fs/block_device.h',
        description: 'Block device abstraction layer — unified interface over NVMe, SATA, USB',
        language: 'cpp',
        code: `// Block device interface — sits between VFS and hardware drivers
struct BlockRequest {
    enum Type { READ, WRITE, FLUSH, TRIM };
    Type        type;
    uint64_t    lba;           // Logical Block Address
    uint32_t    block_count;
    uint8_t*    buffer;        // virtual address in caller shard
    uint32_t    caller_shard;  // for async completion IPC
    uint64_t    request_id;    // token for matching completions
};

class BlockDevice {
public:
    int  submit_request(BlockRequest& req);
    void complete_request(uint64_t request_id, int status);
    bool queue_empty() const;
private:
    BlockRequest m_queue[256]; // ring queue
    uint64_t     m_queue_head, m_queue_tail;
};`,
      },
      {
        id: 'lvm',
        filename: 'include/fs/lvm.h',
        description: 'Logical Volume Manager — PV/VG/LV hierarchy, snapshots, thin provisioning',
        language: 'cpp',
        code: `// LVM hierarchy: Physical Volume → Volume Group → Logical Volume
// PV: one block device.  VG: pool of PVs.  LV: virtual block device.

int sigma_lv_create(const char* vg, const char* lv,
                    uint64_t size, sigma_lv_type_t type,
                    sigma_lv_t* out);

int sigma_lv_resize(const char* vg, const char* lv,
                    int64_t delta_bytes, bool resize_fs);

// COW snapshot — copy-on-write, instant creation
int sigma_lv_snapshot(const char* vg, const char* origin,
                      const char* snap_name, uint64_t cow_size,
                      sigma_lv_t* out);

// Merge snapshot back into origin
int sigma_lv_snap_merge(const char* vg, const char* snap_name);`,
      },
      {
        id: 'disk_cipher',
        filename: 'include/crypto/disk_cipher.h',
        description: 'dm-crypt style full-disk encryption — AES-XTS-256, TPM2 key sealing',
        language: 'cpp',
        code: `// Disk encryption — intercepts block I/O, encrypt/decrypt per sector
// Key derivation uses PBKDF2-SHA512 (NOT zero bytes — stub fixed in R10)

int sigma_disk_cipher_format(const char* device,
                              sigma_cipher_algo_t algo,
                              const char* passphrase,
                              bool tpm_seal);

int sigma_disk_cipher_open(const char* device,
                            const char* passphrase,
                            sigma_disk_cipher_t* out);

// Sector-level AES-XTS encryption (hardware AES-NI accelerated)
int sigma_disk_cipher_encrypt(sigma_disk_cipher_t* ctx,
                               uint64_t sector,
                               uint8_t* buf, size_t len);`,
      },
      {
        id: 'raid',
        filename: 'include/fs/raid.h',
        description: 'Software RAID 0/1/5/6/10 — striping, mirroring, parity, online rebuild',
        language: 'cpp',
        code: `// Software RAID — sits between block device and VFS
typedef enum sigma_raid_level {
    RAID_0  = 0,  // striping: max perf, no redundancy
    RAID_1  = 1,  // mirroring: full redundancy, n/2 capacity
    RAID_5  = 5,  // parity: n-1 capacity, 1 disk fault tolerance
    RAID_6  = 6,  // double parity: n-2 capacity, 2 disk faults
    RAID_10 = 10, // striped mirrors: balanced
} sigma_raid_level_t;

int sigma_raid_create(sigma_raid_level_t level,
                      uint32_t* shards, size_t count,
                      uint32_t chunk_kb, sigma_raid_array_t* out);

// Background scrub — verifies parity/checksums across all disks
int sigma_raid_scrub(uint32_t array_id, uint64_t* errors);`,
      },
    ],
  },
  {
    id: 'filesystems',
    name: 'Filesystems',
    icon: 'Folder',
    files: [
      {
        id: 'vfs',
        filename: 'include/fs/vfs.h',
        description: 'Virtual Filesystem abstraction — all FS ops route through this layer',
        language: 'cpp',
        code: `// VFS — unified interface over SigmaFS, ext4, FAT32, NTFS, etc.
typedef struct sigma_fs_ops {
    int     (*mount)  (const char* src, const char* target, uint32_t flags, void* data);
    int     (*lookup) (uint64_t dir_inode, const char* name, sigma_vnode_t* out);
    ssize_t (*read)   (uint64_t inode, uint64_t offset, void* buf, size_t len);
    ssize_t (*write)  (uint64_t inode, uint64_t offset, const void* buf, size_t len);
    int     (*create) (uint64_t dir_inode, const char* name, uint32_t mode, sigma_vnode_t* out);
    int     (*unlink) (uint64_t dir_inode, const char* name);
    int     (*mkdir)  (uint64_t dir_inode, const char* name, uint32_t mode);
    int     (*readdir)(uint64_t dir_inode, uint64_t* offset, sigma_dirent_t* entries, size_t max);
    int     (*sync)   (void);
} sigma_fs_ops_t;

// Register a filesystem type (called by fs shard on startup)
int sigma_vfs_register_fs(const char* fstype, const sigma_fs_ops_t* ops);`,
      },
      {
        id: 'sigmafs',
        filename: 'fs/sigmafs/sigmafs.h',
        description: 'SigmaFS — native COW filesystem with inline xattrs, snapshots, extent trees',
        language: 'cpp',
        code: `// SigmaFS on-disk inode (256 bytes, 16 per 4KB block)
typedef struct sigmafs_inode {
    uint32_t mode;
    uint64_t size;
    uint64_t extent_tree_root;   // B-tree of extents
    uint64_t cow_gen;            // generation when written
    uint64_t cow_prev_inode;     // previous version (0=none)
    // Inline SemanticFS xattrs (Haiku-inspired)
    uint8_t  xattr_trust[8];     // SIGMA:TRUST label
    uint8_t  xattr_class[8];     // SIGMA:CLASS label
    uint8_t  xattr_signer[32];   // Dilithium3 public key hash
    uint8_t  checksum[32];       // SHA-256 of inode
} sigmafs_inode_t;

// Snapshot creation — instant (COW, no data copy)
int sigmafs_snap_create(const char* mountpoint, const char* name,
                         sigmafs_snapshot_t* out);`,
      },
    ],
  },
  {
    id: 'network',
    name: 'Network Stack',
    icon: 'Network',
    files: [
      {
        id: 'tcp',
        filename: 'include/net/stack.h',
        description: 'TCP/IP stack — shard-based, AF_INET/AF_INET6, full state machine',
        language: 'cpp',
        code: `// TCP state machine
typedef enum sigma_tcp_state {
    TCP_CLOSED=0, TCP_LISTEN=1, TCP_SYN_SENT=2, TCP_SYN_RECV=3,
    TCP_ESTABLISHED=4, TCP_FIN_WAIT1=5, TCP_FIN_WAIT2=6,
    TCP_CLOSE_WAIT=7, TCP_CLOSING=8, TCP_LAST_ACK=9, TCP_TIME_WAIT=10,
} sigma_tcp_state_t;

// Socket API (maps to sigma-netd IPC)
int     sigma_socket  (int domain, int type, int proto);
int     sigma_connect (int fd, const sigma_sockaddr_t* addr, uint32_t len);
ssize_t sigma_send    (int fd, const void* buf, size_t len, int flags);
ssize_t sigma_recv    (int fd, void* buf, size_t len, int flags);
int     sigma_bind    (int fd, const sigma_sockaddr_t* addr, uint32_t len);
int     sigma_listen  (int fd, int backlog);
int     sigma_accept  (int fd, sigma_sockaddr_t* addr, uint32_t* len);`,
      },
    ],
  },
  {
    id: 'graphics',
    name: 'Graphics & Display',
    icon: 'Monitor',
    files: [
      {
        id: 'drm',
        filename: 'include/gfx/drm.h',
        description: 'DRM/KMS subsystem — mode setting, GEM buffers, page flip, dma-buf',
        language: 'cpp',
        code: `// DRM/KMS — kernel manages modesetting, userspace renders
void sigma_drm_init(void);

// Create a dumb (CPU-accessible) framebuffer
int sigma_drm_dumb_create(uint32_t w, uint32_t h, uint32_t bpp,
                           sigma_dumb_buf_t* out);

// Atomic page flip (vsync-synchronized)
int sigma_drm_page_flip(uint32_t crtc_id, uint32_t fb_id);

// GEM buffer — GPU-accessible memory (for hardware acceleration)
int sigma_gem_create(size_t size, sigma_gem_obj_t* out);
int sigma_gem_mmap  (uint32_t handle, void** out_ptr);
int sigma_gem_export(uint32_t handle, int* out_dmabuf_fd); // zero-copy share`,
      },
    ],
  },
  {
    id: 'security',
    name: 'Security & Crypto',
    icon: 'Lock',
    files: [
      {
        id: 'mac',
        filename: 'include/security/mac_policy.h',
        description: 'MAC policy engine — Bell-LaPadula + Biba, capability tokens, AVC cache',
        language: 'cpp',
        code: `// MAC label (Bell-LaPadula + Biba combined model)
typedef struct sigma_mac_label {
    uint8_t  sensitivity;   // 0=public → 7=top-secret
    uint8_t  integrity;     // 0=untrusted → 7=system
    uint32_t compartments;  // bitmask: finance, medical, legal...
    char     context[64];   // "system_u:kernel_t"
} sigma_mac_label_t;

// AVC fast path — called from EVERY syscall handler
sigma_policy_action_t sigma_avc_check(
    uint32_t subject_shard,
    uint64_t object_id,
    sigma_right_t rights);

// Capability delegation with attenuation
int sigma_cap_delegate(uint32_t to_shard,
                        const sigma_capability_t* cap,
                        sigma_right_t mask); // mask limits rights`,
      },
      {
        id: 'secboot',
        filename: 'kernel/security/sigma_secboot.h',
        description: 'Secure Boot + TPM 2.0 — Dilithium3 image signing, PCR seal/unseal',
        language: 'cpp',
        code: `// Secure Boot chain: UEFI → sigma-bootloader → kernel → initramfs
// All signatures use Dilithium3 (ML-DSA-65) — NOT Kyber (KEM only)

sigma_secboot_err_t sigma_secboot_verify_image(
    const sigma_secboot_image_header_t* header,
    const void* payload);

// TPM seal: bind disk encryption key to PCR[8]=kernel + PCR[9]=initramfs
sigma_secboot_err_t sigma_secboot_tpm_seal(
    const uint8_t* secret, size_t secret_len,
    const uint32_t* pcrs,  size_t pcr_count,
    uint8_t* sealed_out,   size_t* sealed_len);

// Unseal fails if kernel was tampered (PCR mismatch)
sigma_secboot_err_t sigma_secboot_tpm_unseal(
    const uint8_t* sealed, size_t sealed_len,
    uint8_t* secret_out,   size_t* secret_len);`,
      },
    ],
  },
  {
    id: 'boot',
    name: 'Boot & Init',
    icon: 'Zap',
    files: [
      {
        id: 'init',
        filename: 'init/sigma_init_shard.cpp',
        description: 'PID 1 init shard — mounts filesystems, enumerates HW, starts all daemons',
        language: 'cpp',
        code: `// PID 1 — first user-space process in SigmaOS
static sigma_service_t services[] = {
    { "sigma-macd",    "/sbin/sigma-macd",    {}, false, false, NULL       },
    { "sigma-busd",    "/sbin/sigma-busd",    {}, false, true,  "sigma-macd"},
    { "sigma-netd",    "/sbin/sigma-netd",    {}, false, true,  "sigma-busd"},
    { "sigma-timed",   "/sbin/sigma-timed",   {}, false, false, "sigma-netd"},
    { "sigma-trustd",  "/sbin/sigma-trustd",  {}, false, true,  "sigma-busd"},
    { "sigma-session", "/sbin/sigma-session", {}, false, true,  "sigma-busd"},
    { NULL }
};

// Service restart on failure (like s6-supervise)
static void reap_zombies(void) {
    uint32_t pid = 0; int status = 0;
    sigma_process_wait(&pid, &status);
    for (int i = 0; services[i].name; i++) {
        if (services[i].pid == pid && !services[i].oneshot)
            start_service(&services[i]); // automatic restart
    }
}`,
      },
    ],
  },
  {
    id: 'utilities',
    name: 'User Utilities',
    icon: 'Terminal',
    files: [
      {
        id: 'ipctrace',
        filename: 'bin/ipctrace/main.cpp',
        description: 'IPC message tracer — like strace but for inter-shard communication',
        language: 'cpp',
        code: `// ipctrace: trace every IPC message between shards
// Usage: ipctrace --from sigma-netd --opcode BLOCK_READ --json

static void print_event(const ipc_event_t* e) {
    if (g_filter.json) {
        printf("{\\"ts\\":%llu,\\"src\\":\\"%s\\",\\"dst\\":\\"%s\\"," 
               "\\"op\\":\\"%s\\",\\"len\\":%zu}\\n",
               e->timestamp_ns,
               shard_name(e->src_shard),
               shard_name(e->dst_shard),
               opcode_name(e->opcode),
               e->payload_len);
        return;
    }
    printf("[%10.6f] %-16s → %-16s  %-18s  len=%-4zu  lat=%lluns\\n",
           e->timestamp_ns / 1e9,
           shard_name(e->src_shard), shard_name(e->dst_shard),
           opcode_name(e->opcode), e->payload_len, e->latency_ns);
}`,
      },
    ],
  },
  {
    id: 'audio',
    name: 'Audio Subsystem',
    icon: 'Music',
    files: [
      {
        id: 'pcm',
        filename: 'include/audio/pcm.h',
        description: 'PCM audio server — PipeWire-style ring buffer mixer, RT scheduling',
        language: 'cpp',
        code: `// Audio stream — RT audio thread calls period callback every 5ms
typedef struct sigma_pcm_stream {
    uint32_t          id;
    sigma_pcm_format_t fmt;
    sigma_pcm_state_t  state;
    float             volume;  // 0.0 – 1.0
    float*            ring_buf; // F32 interleaved frames (shared memory)
    uint32_t          ring_frames;
    uint32_t          write_pos; // updated by server
    uint32_t          read_pos;  // updated by server
    uint64_t          xrun_count;
} sigma_pcm_stream_t;

// Period callback fires every period_frames (e.g. 240 frames @ 48kHz = 5ms)
// Must be registered with SCHED_RT_FIFO to avoid underruns
int sigma_pcm_set_period_cb(uint32_t stream_id,
                             sigma_pcm_period_cb cb, void* userdata);`,
      },
    ],
  },
  {
    id: 'containers',
    name: 'Containers & VMs',
    icon: 'Layers',
    files: [
      {
        id: 'namespace',
        filename: 'include/containers/namespace.h',
        description: 'Container namespace isolation — VFS, net, PID, IPC, user, UTS',
        language: 'cpp',
        code: `// OCI-compatible container runtime for SigmaOS
typedef struct sigma_ns_config {
    bool     isolate_vfs;     // private mount namespace
    bool     isolate_net;     // private network stack
    bool     isolate_pid;     // PID 1 in container
    bool     isolate_ipc;     // private sigma-bus channels
    bool     isolate_user;    // UID/GID mapping
    char     rootfs[256];     // container root directory
    uint64_t mem_limit;       // bytes (0=no limit)
    uint64_t cpu_shares;      // relative weight (1024=default)
    const uint8_t* seccomp_prog; // serialised BPF filter
    size_t         seccomp_len;
} sigma_ns_config_t;

// CRIU-style checkpoint/restore
int sigma_container_checkpoint(uint32_t id, const char* dir);
int sigma_container_restore(const char* dir, sigma_container_t* out);`,
      },
    ],
  },
  {
    id: 'power',
    name: 'Power Management',
    icon: 'Battery',
    files: [
      {
        id: 'powerd',
        filename: 'bin/powerd/main.cpp',
        description: 'ACPI power daemon — lid/battery/thermal events, suspend/hibernate, governors',
        language: 'cpp',
        code: `// powerd — handles all ACPI power events
static void handle_acpi_event(acpi_event_t ev) {
    switch (ev) {
    case ACPI_EVENT_LID_CLOSE:
        sigma_session_lock();
        sigma_acpi_enter_s3(); // Suspend to RAM (S3)
        break;
    case ACPI_EVENT_BATTERY_CRIT:
        sigma_notifyd_send(2, "sigma-power",
                           "Battery critical — hibernating", "");
        sigma_acpi_enter_s4(); // Hibernate to disk (S4)
        break;
    case ACPI_EVENT_THERMAL_CRIT:
        printf("[powerd] THERMAL CRITICAL %u°C — emergency off\\n", g_cpu_temp);
        sigma_acpi_power_off();
        break;
    case ACPI_EVENT_AC_ONLINE:
        apply_profile(PROFILE_BALANCED);
        break;
    }
}`,
      },
    ],
  },
];
