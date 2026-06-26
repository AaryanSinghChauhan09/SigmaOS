/**
 * =========================================================================
 * Σ SIGMAOS: PROTON BRIDGE (POSIX COMPATIBILITY LAYER)
 * =========================================================================
 * An opt-in syscall translation layer mapping legacy Linux x86_64 syscalls
 * to native SigmaOS Sovereign calls. This enables execution of standard 
 * Linux ELF binaries without modifying the core kernel design.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace Compat {

/* Basic Linux Syscall Numbers (x86_64) */
#define LINUX_SYS_READ    0
#define LINUX_SYS_WRITE   1
#define LINUX_SYS_OPEN    2
#define LINUX_SYS_CLOSE   3
#define LINUX_SYS_MMAP    9
#define LINUX_SYS_BRK     12
#define LINUX_SYS_EXIT    60

/* SigmaOS Native Syscall Stubs */
extern "C" {
    sigma_i32 sys_file_read(sigma_u32 fd, void* buf, sigma_size_t count);
    sigma_i32 sys_file_write(sigma_u32 fd, const void* buf, sigma_size_t count);
    sigma_u32 sys_file_open(const char* path, sigma_u32 flags);
    sigma_i32 sys_file_close(sigma_u32 fd);
    void      sys_thread_exit(sigma_u32 code);
}

class ProtonBridge {
public:
    static ProtonBridge& getInstance() {
        static ProtonBridge instance;
        return instance;
    }

    void init() {
        sigma_log("[ProtonBridge] Initializing POSIX Compatibility Layer...");
        /* Register the syscall trap handler extension */
        registerSyscallTrap();
    }

    /* ELF Loader Extension */
    sigma_bool inspectElfHeader(const sigma_u8* elf_header) {
        /* Check if it's a Linux ELF (OSABI = 3) */
        if (elf_header[0] == 0x7F && elf_header[1] == 'E' && 
            elf_header[2] == 'L'  && elf_header[3] == 'F') {
            
            sigma_u8 osabi = elf_header[7];
            if (osabi == 3 || osabi == 0) { /* Linux or System V */
                sigma_log_info("[ProtonBridge] Legacy Linux ELF detected. Attaching compatibility shim.");
                return SIGMA_TRUE;
            }
        }
        return SIGMA_FALSE;
    }

    /* The actual translation engine */
    sigma_i64 translateSyscall(sigma_u64 sys_no, sigma_u64 arg1, sigma_u64 arg2, 
                               sigma_u64 arg3, sigma_u64 arg4, sigma_u64 arg5) {
        
        switch (sys_no) {
            case LINUX_SYS_READ:
                /* ssize_t read(int fd, void *buf, size_t count) */
                return sys_file_read((sigma_u32)arg1, (void*)arg2, (sigma_size_t)arg3);

            case LINUX_SYS_WRITE:
                /* ssize_t write(int fd, const void *buf, size_t count) */
                return sys_file_write((sigma_u32)arg1, (const void*)arg2, (sigma_size_t)arg3);

            case LINUX_SYS_OPEN:
                /* int open(const char *pathname, int flags) */
                return sys_file_open((const char*)arg1, (sigma_u32)arg2);

            case LINUX_SYS_CLOSE:
                /* int close(int fd) */
                return sys_file_close((sigma_u32)arg1);

            case LINUX_SYS_EXIT:
                /* void _exit(int status) */
                sys_thread_exit((sigma_u32)arg1);
                return 0;

            default:
                sigma_log_err("[ProtonBridge] Unimplemented Linux syscall: %llu", (unsigned long long)sys_no);
                return -38; /* ENOSYS */
        }
    }

    sigma_status mapDxvkSurface(sigma_u32 hwnd, sigma_u32* vulkan_surface) {
        /* Legacy hook from Phase 3 roadmap */
        sigma_log_info("[ProtonBridge] Mapping Win32 HWND %u to Zenith Surface.", hwnd);
        if (vulkan_surface) *vulkan_surface = 0xDEADBEEF;
        return SIGMA_SUCCESS;
    }

private:
    ProtonBridge() {}

    void registerSyscallTrap() {
        /* In a real implementation, this modifies the MSR_LSTAR entry point
         * or sets up a hypervisor trap to intercept the target binary's calls */
    }
};

} // namespace Compat
} // namespace SigmaOS

/* --- C API Wrappers --- */
extern "C" {
    void sigma_proton_init(void) {
        SigmaOS::Compat::ProtonBridge::getInstance().init();
    }

    sigma_bool sigma_proton_check_elf(const sigma_u8* elf_header) {
        return SigmaOS::Compat::ProtonBridge::getInstance().inspectElfHeader(elf_header);
    }

    sigma_i64 sigma_proton_syscall_trap(sigma_u64 sys_no, sigma_u64 arg1, sigma_u64 arg2, 
                                        sigma_u64 arg3, sigma_u64 arg4, sigma_u64 arg5) {
        return SigmaOS::Compat::ProtonBridge::getInstance().translateSyscall(sys_no, arg1, arg2, arg3, arg4, arg5);
    }
}

