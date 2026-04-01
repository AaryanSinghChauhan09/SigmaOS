// -----------------------------------------------------------------------------
// SigmaOS Sovereign System Cleaner & Task Manager (Zero-Dependency C11)
// -----------------------------------------------------------------------------
// Native implementations of Disk Cleanup, Temp File Cleaner, and Task Manager.
// Built entirely on raw x86-64 Linux syscalls. NO libc dependencies.
// -----------------------------------------------------------------------------

#define SYS_READ      0
#define SYS_WRITE     1
#define SYS_OPEN      2
#define SYS_CLOSE     3
#define SYS_STAT      4
#define SYS_FSTAT     5
#define SYS_LSTAT     6
#define SYS_UNLINK    87
#define SYS_KILL      62
#define SYS_EXIT      60
#define SYS_GETDENTS64 217
#define SYS_NANOSLEEP 35

#define O_RDONLY    00000000
#define O_DIRECTORY 00200000
#define AT_FDCWD    -100

// Basic types
typedef unsigned long long uint64_t;
typedef long long int64_t;
typedef unsigned int uint32_t;
typedef int int32_t;
typedef unsigned short uint16_t;
typedef unsigned char uint8_t;

// Structs
struct linux_dirent64 {
    uint64_t d_ino;
    int64_t  d_off;
    unsigned short d_reclen;
    unsigned char  d_type;
    char           d_name[];
};

struct timespec {
    int64_t tv_sec;
    int64_t tv_nsec;
};

// -----------------------------------------------------------------------------
// Core Syscall Wrappers
// -----------------------------------------------------------------------------
static inline int64_t syscall1(int64_t n, int64_t a1) {
    int64_t ret;
    __asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1) : "rcx", "r11", "memory");
    return ret;
}

static inline int64_t syscall2(int64_t n, int64_t a1, int64_t a2) {
    int64_t ret;
    __asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1), "S"(a2) : "rcx", "r11", "memory");
    return ret;
}

static inline int64_t syscall3(int64_t n, int64_t a1, int64_t a2, int64_t a3) {
    int64_t ret;
    __asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1), "S"(a2), "d"(a3) : "rcx", "r11", "memory");
    return ret;
}

// -----------------------------------------------------------------------------
// String Utilities
// -----------------------------------------------------------------------------
int sigma_strlen(const char *str) {
    int len = 0;
    while (str[len]) len++;
    return len;
}

void sigma_print(const char *str) {
    syscall3(SYS_WRITE, 1, (int64_t)str, sigma_strlen(str));
}

void sigma_print_int(int64_t num) {
    char buf[32];
    int i = 30;
    int is_neg = 0;
    if (num < 0) { is_neg = 1; num = -num; }
    if (num == 0) { buf[i--] = '0'; }
    buf[31] = '\0';
    while (num > 0) {
        buf[i--] = (num % 10) + '0';
        num /= 10;
    }
    if (is_neg) { buf[i--] = '-'; }
    sigma_print(&buf[i + 1]);
}

int sigma_strcmp(const char *s1, const char *s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

int sigma_atoi(const char *str) {
    int res = 0;
    for (int i = 0; str[i] >= '0' && str[i] <= '9'; ++i) {
        res = res * 10 + str[i] - '0';
    }
    return res;
}

// -----------------------------------------------------------------------------
// Task Manager Utility
// -----------------------------------------------------------------------------
void task_manager_kill(const char* pid_str) {
    int pid = sigma_atoi(pid_str);
    if (pid > 0) {
        int64_t res = syscall2(SYS_KILL, pid, 9); // SIGKILL
        if (res == 0) {
            sigma_print("[+] TASK MANAGER: Successfully terminated PID ");
            sigma_print_int(pid);
            sigma_print("\n");
        } else {
            sigma_print("[-] TASK MANAGER: Failed to terminate PID.\n");
        }
    }
}

// -----------------------------------------------------------------------------
// Temp File Cleaner / Disk Cleanup Utility
// -----------------------------------------------------------------------------
void clean_directory(const char* path) {
    int fd = syscall3(SYS_OPEN, (int64_t)path, O_RDONLY | O_DIRECTORY, 0);
    if (fd < 0) {
        sigma_print("[-] CLEANER: Cannot access directory: ");
        sigma_print(path);
        sigma_print("\n");
        return;
    }

    char buf[1024];
    int chars_read;
    int files_cleaned = 0;

    while ((chars_read = syscall3(SYS_GETDENTS64, fd, (int64_t)buf, sizeof(buf))) > 0) {
        for (int bpos = 0; bpos < chars_read;) {
            struct linux_dirent64 *d = (struct linux_dirent64 *)(buf + bpos);
            char *d_name = d->d_name;

            // Skip "." and ".."
            if (sigma_strcmp(d_name, ".") != 0 && sigma_strcmp(d_name, "..") != 0) {
                // Construct full path for deletion (basic concat, unsafe for complex paths but fast here)
                char full_path[512] = {0};
                int i = 0, j = 0;
                while (path[i] && i < 255) full_path[i] = path[i], i++;
                full_path[i++] = '/';
                while (d_name[j] && i < 510) full_path[i++] = d_name[j++];
                
                // Attempt to remove file natively
                int64_t del_res = syscall1(SYS_UNLINK, (int64_t)full_path);
                if (del_res == 0) {
                    files_cleaned++;
                }
            }
            bpos += d->d_reclen;
        }
    }
    syscall1(SYS_CLOSE, fd);
    
    sigma_print("[+] CLEANER: Swept directory -> ");
    sigma_print(path);
    sigma_print(" | Files Removed: ");
    sigma_print_int(files_cleaned);
    sigma_print("\n");
}

void execute_disk_cleanup() {
    sigma_print("\n[=] INITIATING DISK CLEANUP & TEMP FILE PURGE (C11 NATIVE)\n");
    // Standard temp paths to clean
    clean_directory("/tmp");
    clean_directory("/var/tmp");
    clean_directory("/var/cache/sigma-pkg"); // Mock package cache
    sigma_print("[=] DISK CLEANUP COMPLETE.\n");
}

// -----------------------------------------------------------------------------
// Automation Daemon
// -----------------------------------------------------------------------------
void start_automation_daemon() {
    sigma_print("[+] AUTOMATION: System Cleanup Daemon starting in background...\n");
    
    struct timespec ts;
    ts.tv_sec = 3600; // Sleep for 1 hour before next cleanup
    ts.tv_nsec = 0;

    // INFINITE BACKGROUND LOOP
    while(1) {
        execute_disk_cleanup();
        sigma_print("[+] AUTOMATION: System Cleaned. Sleeping for 1 Hour...\n");
        syscall2(SYS_NANOSLEEP, (int64_t)&ts, 0); 
    }
}

// -----------------------------------------------------------------------------
// Entry Point (Zero-Dependency)
// -----------------------------------------------------------------------------
void _start() {
    // Basic argument parsing emulation
    // In actual implementation, we read ARGC/ARGV from stack.
    // For now, assume this tool acts as a multiplexer based on invocation.
    
    sigma_print("\n=== SIGMAOS SOVEREIGN MAINTENANCE TOOL ===\n");
    sigma_print("Automated Disk Cleanup, Task Manager & Temp Cache Cleaner\n");
    sigma_print("Execution logic handled flawlessly without libc.\n");
    
    // Example: Trigger immediate disk cleanup manually
    execute_disk_cleanup();
    
    // Example: Automatically enter background automation loop
    // start_automation_daemon(); 

    syscall1(SYS_EXIT, 0); // Graceful termination
}
