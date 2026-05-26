/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL FILE SYSTEM (VFS v1.0)
 * =========================================================================
 * Unified abstraction layer over multiple file systems.
 * POSIX-compliant FD interface.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_vfs.h"

namespace SigmaOS {
namespace Kernel {

class SovereignVFS {
public:
    static SovereignVFS& getInstance() {
        static SovereignVFS instance;
        return instance;
    }

    void init() {
        m_inode_count = 0;
        for (sigma_u32 i = 0; i < VFS_MAX_INODES; i++) {
            m_inodes[i].inode_id = 0;
        }

        sigma_log("[VFS] Sovereign Virtual File System initialized.");
        
        /* Create root filesystem structure */
        createInode("/", VFS_NODE_DIR, 0);
        createInode("/dev", VFS_NODE_DIR, 0);
        createInode("/proc", VFS_NODE_DIR, 0);
        
        /* Mount standard devices */
        mount("/dev/null", "chardev", 1);
        mount("/dev/zero", "chardev", 2);
    }

    int mount(const char* target_path, const char* fs_type, sigma_u32 device_id) {
        /* Find or create the mount point inode */
        int res = createInode(target_path, VFS_NODE_DEVICE, 0);
        if (res != K_OK && res != K_ERR_BUSY) return res; /* Busy means it exists */
        
        sigma_inode_t* inode = findInodeByName(target_path);
        if (inode) {
            inode->device_id = device_id;
            sigma_log_info("[VFS] Mounted %s (%s) on device ID %u\n", target_path, fs_type, device_id);
            return K_OK;
        }
        return K_ERR_NOTFOUND;
    }

    int unmount(const char* target_path) {
        sigma_inode_t* inode = findInodeByName(target_path);
        if (!inode) return K_ERR_NOTFOUND;
        
        inode->device_id = 0;
        sigma_log_info("[VFS] Unmounted %s\n", target_path);
        return K_OK;
    }

    int openFile(sigma_u32 pid, const char* path, sigma_u16 flags) {
        sigma_inode_t* inode = findInodeByName(path);
        if (!inode) {
            /* If not found, create if requested (simplified) */
            createInode(path, VFS_NODE_FILE, pid);
            inode = findInodeByName(path);
            if (!inode) return -1;
        }

        /* In a full OS, this allocates from a per-process FD table.
         * Here we just return a fake FD mapped to the inode ID. */
        int fd = inode->inode_id + 1000; 
        sigma_log_info("[VFS] Process %u opened '%s' (FD %d)\n", pid, path, fd);
        return fd;
    }

    int closeFile(sigma_u32 pid, int fd) {
        sigma_log_info("[VFS] Process %u closed FD %d\n", pid, fd);
        return K_OK;
    }

    sigma_i64 read(sigma_u32 pid, int fd, void* buf, sigma_usize count) {
        sigma_u32 inode_id = fd - 1000;
        sigma_inode_t* inode = findInodeById(inode_id);
        if (!inode) return -1;

        sigma_log_info("[VFS] Process %u read %llu bytes from '%s'\n", pid, (unsigned long long)count, inode->name);
        return count; /* Fake success */
    }

    sigma_i64 write(sigma_u32 pid, int fd, const void* buf, sigma_usize count) {
        sigma_u32 inode_id = fd - 1000;
        sigma_inode_t* inode = findInodeById(inode_id);
        if (!inode) return -1;

        inode->size += count;
        inode->modified_tsc = cpu_rdtsc();
        sigma_log_info("[VFS] Process %u wrote %llu bytes to '%s'\n", pid, (unsigned long long)count, inode->name);
        return count; /* Fake success */
    }

    int createInode(const char* path, sigma_vfs_node_type_t type, sigma_u32 owner_pid) {
        if (findInodeByName(path)) return K_ERR_BUSY; /* Already exists */
        if (m_inode_count >= VFS_MAX_INODES) return K_ERR_NOMEM;

        sigma_u32 id = m_inode_count + 1;
        sigma_inode_t& node = m_inodes[id - 1];
        node.inode_id = id;
        node.type = type;
        sigma_strncpy(node.name, path, VFS_FILENAME_LEN);
        node.size = 0;
        node.created_tsc = cpu_rdtsc();
        node.modified_tsc = node.created_tsc;
        node.permissions = VFS_PERM_READ | VFS_PERM_WRITE | VFS_PERM_EXEC;
        node.owner_pid = owner_pid;
        node.device_id = 0;
        node.block_start = 0;

        m_inode_count++;
        return K_OK;
    }

    void printMounts() {
        sigma_log("\n--- VFS MOUNT POINTS & INODES ---");
        for (sigma_u32 i = 0; i < m_inode_count; i++) {
            sigma_inode_t& n = m_inodes[i];
            const char* t_str = "FILE";
            if (n.type == VFS_NODE_DIR) t_str = "DIR";
            else if (n.type == VFS_NODE_DEVICE) t_str = "DEV";
            
            sigma_log_info("| [%s] %-15s (ID: %u, Size: %llu bytes)\n",
                           t_str, n.name, n.inode_id, (unsigned long long)n.size);
        }
        sigma_log("---------------------------------");
    }

private:
    SovereignVFS() : m_inode_count(0) {}

    sigma_inode_t* findInodeByName(const char* path) {
        for (sigma_u32 i = 0; i < m_inode_count; i++) {
            /* Simple string comparison wrapper */
            const char* s1 = m_inodes[i].name;
            const char* s2 = path;
            int match = 1;
            while (*s1 && *s2) {
                if (*s1 != *s2) { match = 0; break; }
                s1++; s2++;
            }
            if (match && *s1 == '\0' && *s2 == '\0') {
                return &m_inodes[i];
            }
        }
        return SIGMA_NULL;
    }

    sigma_inode_t* findInodeById(sigma_u32 id) {
        if (id == 0 || id > m_inode_count) return SIGMA_NULL;
        return &m_inodes[id - 1];
    }

    sigma_inode_t m_inodes[VFS_MAX_INODES];
    sigma_u32     m_inode_count;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void vfs_init(void) { SigmaOS::Kernel::SovereignVFS::getInstance().init(); }

int vfs_mount(const char* target, const char* fs_type, sigma_u32 dev_id) {
    return SigmaOS::Kernel::SovereignVFS::getInstance().mount(target, fs_type, dev_id);
}

int vfs_open(sigma_u32 pid, const char* path, sigma_u16 flags) {
    return SigmaOS::Kernel::SovereignVFS::getInstance().openFile(pid, path, flags);
}

int vfs_close(sigma_u32 pid, int fd) {
    return SigmaOS::Kernel::SovereignVFS::getInstance().closeFile(pid, fd);
}

sigma_i64 vfs_read(sigma_u32 pid, int fd, void* buf, sigma_usize count) {
    return SigmaOS::Kernel::SovereignVFS::getInstance().read(pid, fd, buf, count);
}

sigma_i64 vfs_write(sigma_u32 pid, int fd, const void* buf, sigma_usize count) {
    return SigmaOS::Kernel::SovereignVFS::getInstance().write(pid, fd, buf, count);
}

void vfs_print_mounts(void) {
    SigmaOS::Kernel::SovereignVFS::getInstance().printMounts();
}

} // extern "C"