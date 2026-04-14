// =============================================================================
// SigmaOS — S11_Virtualization — SovereignHypervisor.c
// Type-1.5 Bare-Metal Hypervisor Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows WSL2 (Hyper-V) — Linux-in-Windows via lightweight VM
//   • macOS Hypervisor.framework — userland VM management without kexts
//   • Linux KVM/QEMU — near-native perf via hardware acceleration (VT-x/AMD-V)
//   • Firecracker (AWS) — sub-5ms microVM boot times
// Architecture:
//   • Direct VMX/SVM instruction orchestration
//   • EPT (Extended Page Tables) for zero-latency memory isolation
//   • VirtIO transport for sovereign disk/net device passthrough
//   • Sub-10ms microVM cold-boot target (Firecracker model)
// =============================================================================

#include <sigma_types.h>


#define MAX_VMS             16
#define VM_MAX_VCPUS        128
#define VM_MIN_BOOT_NS      5000000 // 5ms boot target

// ── VM Descriptor ─────────────────────────────────────────────────────────────
typedef struct {
    uint32_t vm_id;
    char     guest_os_type[32]; // "Linux", "Windows", "Sigma-Sub"
    uint64_t ram_size_mb;
    uint8_t  vcpu_count;
    bool     vt_x_enabled;
    uint32_t state; // 0=Off, 1=Running, 2=Paused
} SovereignVM;

// ── Virtual Device Table (VirtIO style) ──────────────────────────────────────
typedef struct {
    uint32_t device_id;
    uint8_t  type; // 0=Net, 1=Disk, 2=Entropy
    void*    backend_buffer;
} VirtIODevice;

static SovereignVM vm_registry[MAX_VMS];
static uint32_t    active_vms = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialize hardware virtualization (check VMX/SVM flags)
bool hypervisor_init_hardware(void);

// Create a new microVM instance (Firecracker model)
SovereignVM* hypervisor_create_vm(const char* os_type, uint64_t ram_mb);

// Map a guest physical address to sovereign host physical address (EPT)
void hypervisor_map_memory(SovereignVM* vm, uint64_t guest_addr, uint64_t host_addr);

// Start VM execution (VM-Entry) — triggers S03 scheduler thread
void hypervisor_start_vm(uint32_t vm_id);

// Inject a virtual interrupt into the guest (APICv/vIP)
void hypervisor_inject_irq(uint32_t vm_id, uint8_t irq_vector);

// Graceful VM shutdown and resource reclamation
void hypervisor_terminate_vm(uint32_t vm_id);

// Snapshot a running VM to disk (hibernation)
void hypervisor_snapshot_vm(uint32_t vm_id, const char* out_path);



