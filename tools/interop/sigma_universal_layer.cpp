/*
 * Σ SigmaOS — sigma_universal_layer: Universal Interoperability Layer
 * Zero-Dependency.
 * 
 * Syscall translation layer (similar to WSL1 or FreeBSD linuxulator).
 * Catches foreign architecture/OS syscalls and translates them to SigmaOS capabilities.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define LINUX_SYS_READ  0
#define LINUX_SYS_WRITE 1
#define LINUX_SYS_OPEN  2
#define LINUX_SYS_CLOSE 3

// SigmaOS native syscall stubs
extern "C" int sigma_sys_read(int fd, void* buf, u64 count);
extern "C" int sigma_sys_write(int fd, const void* buf, u64 count);

/*
 * Trap handler for foreign syscalls (e.g. executed via INT 0x80 or SYSCALL instruction 
 * from a process flagged as 'Foreign/Linux')
 */
extern "C" u64 sigma_interop_translate_linux_syscall(u64 sys_num, u64 arg1, u64 arg2, u64 arg3) {
    // sigma_vga_printf("[Interop] Intercepted Linux syscall: %llu\n", sys_num);
    
    switch (sys_num) {
        case LINUX_SYS_WRITE:
            // Translate Linux write() to SigmaOS write()
            // arg1 = fd, arg2 = buffer ptr, arg3 = count
            // Note: In reality, we'd need to validate pointers against the sandbox profile.
            return sigma_sys_write((int)arg1, (const void*)arg2, arg3);
            
        case LINUX_SYS_READ:
            return sigma_sys_read((int)arg1, (void*)arg2, arg3);
            
        case LINUX_SYS_OPEN:
            sigma_vga_printf("[Interop] Translating Linux open() -> SigmaOS ZKFS VFS\n");
            // Translate paths, handle flags (O_RDONLY -> SIGMA_O_READ)
            return 3; // Dummy FD
            
        case 60: // LINUX_SYS_EXIT
            sigma_vga_printf("[Interop] Process requested Linux exit()\n");
            // Terminate process
            return 0;
            
        default:
            sigma_vga_printf("[Interop] WARNING: Unimplemented Linux syscall %llu\n", sys_num);
            return (u64)-38; // -ENOSYS
    }
}
