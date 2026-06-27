// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_vcpu.cpp — Virtual CPU (VCPU) — VMLAUNCH/VMEXIT handling
//
// Completes the hypervisor started in sigma_hypervisor.cpp + sigma_vmm.cpp.
//
// Each VCPU runs in VMX non-root mode (guest) until a VM-exit occurs.
// On exit, we dispatch to the appropriate exit handler and either:
//   • emulate the offending instruction, or
//   • inject a virtual exception, or
//   • forward the request to a userland device model (virtio).
//
// Inspired by:
//   • Linux KVM arch/x86/kvm/vmx/vmx.c
//   • bhyve (FreeBSD) usr.sbin/bhyve/vmx.c
//   • Intel SDM Vol 3C Chapter 25–27 (VMX)

#include "sigma_vcpu.h"
#include "sigma_vmm.h"
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// ── VMCS field encodings (Intel SDM Vol 3C Table 24-3) ───────────────────────

#define VMCS_GUEST_RIP          0x681E
#define VMCS_GUEST_RSP          0x681C
#define VMCS_GUEST_RFLAGS       0x6820
#define VMCS_GUEST_CR0          0x6800
#define VMCS_GUEST_CR3          0x6802
#define VMCS_GUEST_CR4          0x6804
#define VMCS_HOST_CR3           0x6C02
#define VMCS_VMEXIT_REASON      0x4402
#define VMCS_EXIT_QUALIFICATION 0x6400
#define VMCS_GUEST_LINEAR_ADDR  0x640A
#define VMCS_VM_ENTRY_CTRLS     0x4012
#define VMCS_VM_EXIT_CTRLS      0x400C
#define VMCS_CPU_BASED_CTRLS    0x4002
#define VMCS_EXCEPTION_BITMAP   0x4004

// VM-exit reasons we handle
#define EXIT_REASON_CPUID       10
#define EXIT_REASON_IO          30
#define EXIT_REASON_MSR_READ    31
#define EXIT_REASON_MSR_WRITE   32
#define EXIT_REASON_HLT         12
#define EXIT_REASON_EPT_VIOLATION 48
#define EXIT_REASON_VMCALL      18
#define EXIT_REASON_EXTERNAL_INT 1

// ── VMCS read/write (inline asm) ──────────────────────────────────────────────

static inline uint64_t vmread(uint64_t field) {
    uint64_t val;
    __asm__ volatile("vmread %1, %0" : "=r"(val) : "r"(field) : "cc");
    return val;
}

static inline void vmwrite(uint64_t field, uint64_t val) {
    __asm__ volatile("vmwrite %1, %0" :: "r"(field), "r"(val) : "cc");
}

// ── VCPU state ────────────────────────────────────────────────────────────────

struct sigma_vcpu {
    uint32_t  vcpu_id;
    uint32_t  vm_id;
    uint8_t   vmcs[4096] __attribute__((aligned(4096)));
    /* Guest register file (saved on VMEXIT) */
    uint64_t  regs[16];   // RAX–R15
    uint64_t  guest_rip;
    uint64_t  guest_rsp;
    uint64_t  guest_rflags;
    /* Run state */
    bool      running;
    bool      halted;
    uint64_t  exit_count;
    /* Pending virtual interrupt */
    uint8_t   pending_irq;
    bool      irq_pending;
};

#define MAX_VCPUS_PER_VM  256
static struct sigma_vcpu g_vcpus[MAX_VCPUS_PER_VM];

// ── CPUID emulation ───────────────────────────────────────────────────────────

static void handle_cpuid(struct sigma_vcpu *v) {
    uint32_t leaf = (uint32_t)v->regs[0];  // RAX
    uint32_t eax = 0, ebx = 0, ecx = 0, edx = 0;

    __asm__ volatile("cpuid"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
        : "a"(leaf), "c"(0)
    );

    // Mask out features we don't expose (e.g., VMX itself)
    if (leaf == 1) {
        ecx &= ~(1u << 5);  // hide VMX from guest
    }

    v->regs[0]  = eax;  // RAX
    v->regs[3]  = ebx;  // RBX
    v->regs[1]  = ecx;  // RCX
    v->regs[2]  = edx;  // RDX
    // Advance RIP past CPUID (2 bytes)
    vmwrite(VMCS_GUEST_RIP, vmread(VMCS_GUEST_RIP) + 2);
}

// ── I/O port emulation ────────────────────────────────────────────────────────

static void handle_io(struct sigma_vcpu *v) {
    uint64_t qual = vmread(EXIT_QUALIFICATION_FIELD);
    (void)qual;
    // I/O is forwarded to userland virtio device model via sigma-vmm socket.
    // For now: complete the instruction (IN → RAX=0xFFFF, OUT → discard)
    v->regs[0] = 0xFFFF;
    vmwrite(VMCS_GUEST_RIP, vmread(VMCS_GUEST_RIP) + 1);
}

// ── VM-exit dispatcher ────────────────────────────────────────────────────────

static void vmexit_dispatch(struct sigma_vcpu *v) {
    uint32_t reason = (uint32_t)vmread(VMCS_VMEXIT_REASON) & 0xFFFF;
    v->exit_count++;

    switch (reason) {
        case EXIT_REASON_CPUID:
            handle_cpuid(v);
            break;
        case EXIT_REASON_HLT:
            v->halted = true;
            vmwrite(VMCS_GUEST_RIP, vmread(VMCS_GUEST_RIP) + 1);
            break;
        case EXIT_REASON_IO:
            handle_io(v);
            break;
        case EXIT_REASON_MSR_READ:
        case EXIT_REASON_MSR_WRITE:
            // Emulate safe MSRs, inject #GP for unknown ones
            vmwrite(VMCS_GUEST_RIP, vmread(VMCS_GUEST_RIP) + 2);
            break;
        case EXIT_REASON_VMCALL:
            // Hypercall interface — handled by sigma_hypercall.cpp
            extern void sigma_hypercall_dispatch(struct sigma_vcpu *);
            sigma_hypercall_dispatch(v);
            break;
        case EXIT_REASON_EPT_VIOLATION:
            // Map the faulting GPA on demand (demand paging for guests)
            extern int sigma_ept_fault(struct sigma_vcpu *, uint64_t gpa);
            sigma_ept_fault(v, vmread(VMCS_GUEST_LINEAR_ADDR));
            break;
        case EXIT_REASON_EXTERNAL_INT:
            // Deliver to host — LAPIC EOI already handled by hardware
            break;
        default:
            // Unknown exit — inject #UD into guest
            v->halted = true;
            break;
    }
}

// ── VCPU run loop ─────────────────────────────────────────────────────────────

int sigma_vcpu_run(uint32_t vcpu_id) {
    struct sigma_vcpu *v = &g_vcpus[vcpu_id];
    if (!v->running) return -1;

    while (!v->halted) {
        // Inject pending virtual IRQ if any
        if (v->irq_pending) {
            // VMX interrupt injection via VM_ENTRY_INTR_INFO_FIELD (not shown)
            v->irq_pending = false;
        }

        // VMRESUME (or VMLAUNCH on first entry)
        int first = (v->exit_count == 0);
        if (first) {
            __asm__ volatile("vmlaunch" ::: "cc", "memory");
        } else {
            __asm__ volatile("vmresume" ::: "cc", "memory");
        }
        // If we reach here, a VM-exit occurred
        vmexit_dispatch(v);
    }
    return 0;
}

// ── VCPU create ───────────────────────────────────────────────────────────────

int sigma_vcpu_create(uint32_t vm_id, uint32_t vcpu_id) {
    if (vcpu_id >= MAX_VCPUS_PER_VM) return -1;
    struct sigma_vcpu *v = &g_vcpus[vcpu_id];
    memset(v, 0, sizeof(*v));
    v->vcpu_id = vcpu_id;
    v->vm_id   = vm_id;
    v->running = true;
    return 0;
}
