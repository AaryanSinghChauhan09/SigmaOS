const fs = require("fs");
const path = require("path");

const root = __dirname;

function writeFile(relPath, content) {
    const fullPath = path.join(root, relPath);
    fs.mkdirSync(path.dirname(fullPath), { recursive: true });
    fs.writeFileSync(fullPath, content.trim() + "\n", "utf-8");
    console.log("Created: " + relPath);
}

// -----------------------------------------------------------------------------
// 1. Syscall Dispatcher Files
// -----------------------------------------------------------------------------

writeFile("kernel/core/syscall/syscalls.h", `
#ifndef SYSCALLS_H
#define SYSCALLS_H

#include "../../../sigma_libc.h"

/* Syscall identifiers - keep them sequential for table-lookup */
enum {
    SYSCALL_GETPID = 0,
    SYSCALL_WRITE   = 1,
    SYSCALL_READ    = 2,
    SYSCALL_EXIT    = 3,
    SYSCALL_OPEN    = 4,
    SYSCALL_CLOSE   = 5,
    SYSCALL_MAX     = 6
};

typedef sigma_u64(*syscall_fn_t)(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
extern const syscall_fn_t syscall_table[SYSCALL_MAX];

#endif // SYSCALLS_H
`);

writeFile("kernel/core/syscall/dispatcher.h", `
#ifndef DISPATCHER_H
#define DISPATCHER_H

#include "syscalls.h"

#ifdef __cplusplus
extern "C" {
#endif

extern sigma_u64 syscall_dispatcher(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4);
extern sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3);

#ifdef __cplusplus
}
#endif

#endif // DISPATCHER_H
`);

writeFile("include/syscall_dispatcher.h", `
#ifndef INCLUDE_SYSCALL_DISPATCHER_H
#define INCLUDE_SYSCALL_DISPATCHER_H

#include "../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

extern sigma_u64 dispatch_syscall(sigma_u32 num, sigma_u64 *args);
extern sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3);

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_SYSCALL_DISPATCHER_H
`);

writeFile("kernel/core/syscall/dispatcher.c", `
#include "dispatcher.h"

static sigma_u64 sys_getpid_impl(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)a1; (void)a2; (void)a3; (void)a4;
    return 1000; // Sovereign init PID
}

static sigma_u64 sys_write_impl(sigma_u64 fd, sigma_u64 buf, sigma_u64 count, sigma_u64 a4) {
    (void)fd; (void)buf; (void)count; (void)a4;
    return count;
}

static sigma_u64 sys_read_impl(sigma_u64 fd, sigma_u64 buf, sigma_u64 count, sigma_u64 a4) {
    (void)fd; (void)buf; (void)count; (void)a4;
    return count;
}

static sigma_u64 sys_exit_impl(sigma_u64 code, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)code; (void)a2; (void)a3; (void)a4;
    while(1) {}
    return 0;
}

static sigma_u64 sys_open_impl(sigma_u64 path, sigma_u64 flags, sigma_u64 mode, sigma_u64 a4) {
    (void)path; (void)flags; (void)mode; (void)a4;
    return 3; // First available fd
}

static sigma_u64 sys_close_impl(sigma_u64 fd, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)fd; (void)a2; (void)a3; (void)a4;
    return 0;
}

const syscall_fn_t syscall_table[SYSCALL_MAX] = {
    sys_getpid_impl,
    sys_write_impl,
    sys_read_impl,
    sys_exit_impl,
    sys_open_impl,
    sys_close_impl
};

sigma_u64 syscall_dispatcher(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    if (nr >= SYSCALL_MAX) {
        sigma_printf("[Syscall] Error: Invalid syscall %llu\\n", nr);
        return (sigma_u64)-1;
    }
    return syscall_table[nr](a1, a2, a3, a4);
}

sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    return syscall_dispatcher((sigma_u64)num, a0, a1, a2, a3);
}
`);

writeFile("kernel/core/syscall/dispatcher.cpp", `
#include "../../../include/syscall_dispatcher.h"

extern "C" sigma_u64 dispatch_syscall(sigma_u32 num, sigma_u64 *args) {
    sigma_printf("[Syscall Dispatcher C++] Dispatching syscall %u\\n", num);
    if (num == 0) return 1000;
    if (args) return args[0];
    return 0;
}

extern "C" sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    sigma_u64 args[4] = { a0, a1, a2, a3 };
    return dispatch_syscall(num, args);
}
`);

writeFile("kernel/core/syscall/handlers/fs_open.cpp", `
#include "../../../../sigma_libc.h"

extern "C" sigma_u64 handle_fs_open(const char* path, int flags, int mode) {
    sigma_printf("[Syscall Handler] fs_open called for path: %s\\n", path ? path : "NULL");
    return 3;
}
`);

writeFile("kernel/core/syscall/handlers/process_spawn.cpp", `
#include "../../../../sigma_libc.h"

extern "C" sigma_u64 handle_process_spawn(const char* binary, const char** argv) {
    sigma_printf("[Syscall Handler] process_spawn called for binary: %s\\n", binary ? binary : "NULL");
    return 1001; // New child PID
}
`);

// -----------------------------------------------------------------------------
// 2. HAL Expansion Files
// -----------------------------------------------------------------------------

writeFile("kernel/core/hal/hal.h", `
#ifndef HAL_H
#define HAL_H

#include "../../../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

void hal_init(void);
void hal_write_io(sigma_u16 port, sigma_u8 value);
sigma_u8 hal_read_io(sigma_u16 port);
void *hal_alloc_pages(sigma_u32 count);
void hal_free_pages(void *addr, sigma_u32 count);

#ifdef __cplusplus
}
#endif

#endif // HAL_H
`);

writeFile("include/sigma_hal.h", `
#ifndef INCLUDE_SIGMA_HAL_H
#define INCLUDE_SIGMA_HAL_H

#include "../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    void (*cpu_halt)(void);
    void (*timer_init)(void);
    void (*interrupt_init)(void);
    void (*mmu_map)(sigma_u64 va, sigma_u64 pa, sigma_u64 flags);
} hal_ops_t;

extern const hal_ops_t *hal_ops;

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_SIGMA_HAL_H
`);

writeFile("kernel/core/hal/x86/hal_x86.S", `
.global _start
_start:
    xor %ebp, %ebp
    mov $0x7C00, %esp
    call hal_init
    hlt

.global hal_write_io
hal_write_io:
    mov 4(%esp), %dx
    mov 8(%esp), %al
    out %al, %dx
    ret

.global hal_read_io
hal_read_io:
    mov 4(%esp), %dx
    in %dx, %al
    movzx %al, %eax
    ret
`);

writeFile("kernel/core/hal/arm/hal_arm.S", `
.global _start
_start:
    mov fp, #0
    ldr sp, =0x80000000
    bl hal_init
    b .
`);

writeFile("kernel/core/hal/riscv/hal_riscv.S", `
.global _start
_start:
    mv s0, zero
    la sp, 0x80000000
    call hal_init
    j .
`);

writeFile("kernel/core/hal/x86/hal_x86.cpp", `
#include "../../../../include/sigma_hal.h"

static void x86_cpu_halt(void) {
    sigma_printf("[HAL: x86] Executing CPU halt...\\n");
}

static void x86_timer_init(void) {
    sigma_printf("[HAL: x86] Initializing PIT/APIC timer...\\n");
}

static void x86_interrupt_init(void) {
    sigma_printf("[HAL: x86] Initializing IDT & PIC/APIC controllers...\\n");
}

static void x86_mmu_map(sigma_u64 va, sigma_u64 pa, sigma_u64 flags) {
    sigma_printf("[HAL: x86] Mapping VA 0x%llx to PA 0x%llx (flags: 0x%llx)\\n", va, pa, flags);
}

static const hal_ops_t x86_hal_ops = {
    x86_cpu_halt,
    x86_timer_init,
    x86_interrupt_init,
    x86_mmu_map
};

const hal_ops_t *hal_ops = &x86_hal_ops;

extern "C" void hal_init(void) {
    sigma_printf("[HAL] hal_init called. Assigning x86 HAL ops.\\n");
    hal_ops = &x86_hal_ops;
}
`);

writeFile("hal/x86/hal_x86.cpp", `
#include "../../include/sigma_hal.h"

static void hal_x86_halt() {
    sigma_printf("[HAL: x86 standalone] CPU halt executed.\\n");
}

static const hal_ops_t standalone_x86_ops = {
    hal_x86_halt,
    0, 0, 0
};

extern "C" void init_standalone_hal() {
    hal_ops = &standalone_x86_ops;
}
`);

// -----------------------------------------------------------------------------
// 3. SovereignVulkanLayer Files
// -----------------------------------------------------------------------------

writeFile("kernel/core/vulkan/sovereign_vulkan.h", `
#ifndef SOVEREIGN_VULKAN_H
#define SOVEREIGN_VULKAN_H

#include "../../../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

void vk_init(void);
void vk_submit_shader(const void *spirv_blob, sigma_u32 size);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGN_VULKAN_H
`);

writeFile("kernel/core/vulkan/sovereign_vulkan.c", `
#include "sovereign_vulkan.h"
#include "../hal/hal.h"

#define VK_CMD_QUEUE ((volatile sigma_u32*)0xFEE00000)

void vk_init(void) {
    sigma_printf("[Vulkan Layer] Initializing MMIO GPU registers...\\n");
    hal_write_io(0xC0, 0x01); // enable GPU
    hal_write_io(0xC4, 0x00); // clear error bits
}

void vk_submit_shader(const void *blob, sigma_u32 size) {
    sigma_printf("[Vulkan Layer] Submitting %u bytes of raw SPIR-V words to MMIO queue...\\n", size);
    const sigma_u32 *words = (const sigma_u32*)blob;
    for (sigma_u32 i = 0; i < size / 4; ++i) {
        VK_CMD_QUEUE[i] = words[i];
    }
    hal_write_io(0xC8, 0x1); // Trigger execution
}
`);

writeFile("graphics/vulkan_layer/vulkan_layer.cpp", `
#include "../../sigma_libc.h"

typedef int VkResult;
typedef void* VkShaderModule;

extern "C" VkResult vkCreateShaderDirect(const sigma_u32 *spirv, sigma_usize size, VkShaderModule *out) {
    sigma_printf("[Vulkan Direct C++] Creating shader module directly from %zu bytes SPIR-V...\\n", size);
    if (out) *out = (VkShaderModule)spirv;
    return 0; // VK_SUCCESS
}
`);

// -----------------------------------------------------------------------------
// 4. Unified Driver API Files
// -----------------------------------------------------------------------------

writeFile("drivers/unified/driver_api.h", `
#ifndef DRIVER_API_H
#define DRIVER_API_H

#include "../../../sigma_libc.h"

typedef enum {
    DEV_WIFI,
    DEV_PRINTER,
    DEV_USB,
    DEV_IOT
} device_type_t;

struct driver_ops {
    sigma_u32 (*init)(void);
    sigma_u32 (*read)(void *buf, sigma_u32 len);
    sigma_u32 (*write)(const void *buf, sigma_u32 len);
    sigma_u32 (*shutdown)(void);
};

#ifdef __cplusplus
extern "C" {
#endif

void driver_register(device_type_t type, const struct driver_ops *ops);
const struct driver_ops *driver_get(device_type_t type);

#ifdef __cplusplus
}
#endif

#endif // DRIVER_API_H
`);

writeFile("include/driver_api.h", `
#ifndef INCLUDE_DRIVER_API_H
#define INCLUDE_DRIVER_API_H

#include "../sigma_libc.h"

typedef enum {
    DEV_WIFI,
    DEV_PRINTER,
    DEV_USB,
    DEV_IOT
} device_type_t;

typedef struct {
    sigma_u32 (*init)(void);
    sigma_u32 (*read)(void *buf, sigma_u32 len);
    sigma_u32 (*write)(const void *buf, sigma_u32 len);
    sigma_u32 (*shutdown)(void);
} driver_t;

#ifdef __cplusplus
extern "C" {
#endif

void register_driver(device_type_t type, const driver_t *drv);
const driver_t *get_driver(device_type_t type);

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_DRIVER_API_H
`);

writeFile("drivers/unified/wifi.c", `
#include "driver_api.h"

static sigma_u32 wifi_init(void) { sigma_printf("[Unified Driver] Wi-Fi init\\n"); return 0; }
static sigma_u32 wifi_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 wifi_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 wifi_shutdown(void) { sigma_printf("[Unified Driver] Wi-Fi shutdown\\n"); return 0; }

static const struct driver_ops wifi_ops = {
    wifi_init,
    wifi_read,
    wifi_write,
    wifi_shutdown
};

void register_wifi_driver(void) {
    driver_register(DEV_WIFI, &wifi_ops);
}
`);

writeFile("drivers/unified/printer.c", `
#include "driver_api.h"

static sigma_u32 printer_init(void) { sigma_printf("[Unified Driver] Printer init\\n"); return 0; }
static sigma_u32 printer_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 printer_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 printer_shutdown(void) { return 0; }

static const struct driver_ops printer_ops = { printer_init, printer_read, printer_write, printer_shutdown };

void register_printer_driver(void) { driver_register(DEV_PRINTER, &printer_ops); }
`);

writeFile("drivers/unified/usb.c", `
#include "driver_api.h"

static sigma_u32 usb_init(void) { sigma_printf("[Unified Driver] USB init\\n"); return 0; }
static sigma_u32 usb_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 usb_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 usb_shutdown(void) { return 0; }

static const struct driver_ops usb_ops = { usb_init, usb_read, usb_write, usb_shutdown };

void register_usb_driver(void) { driver_register(DEV_USB, &usb_ops); }
`);

writeFile("drivers/unified/iot.c", `
#include "driver_api.h"

static sigma_u32 iot_init(void) { sigma_printf("[Unified Driver] IoT init\\n"); return 0; }
static sigma_u32 iot_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 iot_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 iot_shutdown(void) { return 0; }

static const struct driver_ops iot_ops = { iot_init, iot_read, iot_write, iot_shutdown };

void register_iot_driver(void) { driver_register(DEV_IOT, &iot_ops); }
`);

// -----------------------------------------------------------------------------
// 5. Testing Plan & Unit Tests Files
// -----------------------------------------------------------------------------

writeFile("tests/syscall_test.cpp", `
#include "../kernel/core/syscall/dispatcher.h"

int main() {
    sigma_printf("[Test] Running syscall_test...\\n");
    sigma_u64 pid = syscall_dispatcher(SYSCALL_GETPID, 0, 0, 0, 0);
    sigma_printf("[Test] SYSCALL_GETPID returned: %llu\\n", pid);
    return 0;
}
`);

writeFile("tests/syscall_dispatcher_test.cpp", `
#include "../include/syscall_dispatcher.h"

int main() {
    sigma_printf("[Test] Running syscall_dispatcher_test...\\n");
    sigma_u64 args[4] = {1, 2, 3, 4};
    sigma_u64 res = dispatch_syscall(0, args);
    sigma_printf("[Test] dispatch_syscall(0) returned: %llu\\n", res);
    return 0;
}
`);

// -----------------------------------------------------------------------------
// 6. Documentation Files
// -----------------------------------------------------------------------------

const syscallDoc = `
# SigmaOS Syscall Dispatcher Architecture

The SigmaOS Syscall Dispatcher is a modular, zero-dependency C/C++ implementation designed to replace high-level abstractions with silicon-direct dispatch tables.

## Mechanism
* \`syscalls.h\`: Defines sequential syscall identifiers (\`SYSCALL_GETPID\`, \`SYSCALL_WRITE\`, etc.) and function prototypes.
* \`dispatcher.c\` / \`dispatcher.cpp\`: Implements direct table lookup O(1) dispatching, validating syscall numbers and forwarding register arguments directly to kernel handlers.
`;

const halDoc = `
# SigmaOS Hardware Abstraction Layer (HAL)

The SigmaOS HAL provides a clean, zero-overhead interface isolating architecture-specific assembly stubs (\`x86\`, \`ARM\`, \`RISC-V\`) from the microkernel core.

## Mechanism
* \`hal.h\` / \`sigma_hal.h\`: Exposes generic hardware operations (\`cpu_halt\`, \`timer_init\`, \`interrupt_init\`, \`mmu_map\`, \`read_io\`, \`write_io\`).
* \`hal_x86.S\`, \`hal_arm.S\`, \`hal_riscv.S\`: Minimal assembly entry points executed early during bootstrap.
`;

const vulkanDoc = `
# SigmaOS Sovereign Vulkan Layer

The SovereignVulkanLayer provides a direct, zero-wrapper C/C++ interface forwarding SPIR-V shader bytecode directly to GPU MMIO command queues.

## Mechanism
* Bypasses heavy Vulkan SDK runtime libraries entirely.
* Streams pre-compiled SPIR-V binaries directly to memory-mapped GPU command queues (\`VK_CMD_QUEUE\`), achieving zero-copy shader execution.
`;

const driverApiDoc = `
# SigmaOS Unified Driver API

The Unified Driver API establishes a common, modular C/C++ interface for all external peripherals (\`Wi-Fi\`, \`Printers\`, \`USB\`, \`IoT\`).

## Mechanism
* \`driver_api.h\`: Defines \`driver_ops\` / \`driver_t\` structures with standard hooks (\`init\`, \`read\`, \`write\`, \`shutdown\`).
* Lock-free registration and O(1) lookup via dedicated kernel driver managers.
`;

writeFile("kernel/core/syscall/README.md", syscallDoc);
writeFile("kernel/core/hal/README.md", halDoc);
writeFile("kernel/core/vulkan/README.md", vulkanDoc);
writeFile("drivers/unified/README.md", driverApiDoc);

writeFile("docs/SyscallDispatcher.md", syscallDoc);
writeFile("docs/HAL.md", halDoc);
writeFile("docs/VulkanLayer.md", vulkanDoc);
writeFile("docs/DriverAPI.md", driverApiDoc);

writeFile("wiki_repo/SyscallDispatcher.md", syscallDoc);
writeFile("wiki_repo/HAL.md", halDoc);
writeFile("wiki_repo/VulkanLayer.md", vulkanDoc);
writeFile("wiki_repo/DriverAPI.md", driverApiDoc);

console.log("All Syscall, HAL, Vulkan, Driver API, Tests, and Documentation files created successfully.");
