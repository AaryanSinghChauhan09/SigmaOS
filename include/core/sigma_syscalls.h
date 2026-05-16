/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SYSCALL INTERFACE (Z-SYSCALL)
 * =========================================================================
 * Defines the public ABI for userland shards to interact with the kernel.
 * Includes module loading for developer extensions.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALLS_H
#define SIGMA_SYSCALLS_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SYS_READ      0x01
#define SYS_WRITE     0x02
#define SYS_FORK      0x03
#define SYS_EXEC      0x04
#define SYS_MMAP      0x09
#define SYS_LOAD_MOD  0x80  /* Load User-Defined Shard Module */
#define SYS_PQC_CRYPT 0x81  /* Execute PQC primitives securely */

/* Syscall wrapper implementation */
sigma_isize sys_read(sigma_u32 fd, void* buffer, sigma_size_t count);
sigma_isize sys_write(sigma_u32 fd, const void* buffer, sigma_size_t count);
sigma_u32   sys_fork(void);
sigma_status sys_exec(const char* path, char* const argv[]);
void*       sys_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);

/* Shard Module API (Extensibility) */
typedef struct {
    const char* module_name;
    void (*init_hook)(void);
    void (*exit_hook)(void);
    void* custom_syscall_table;
} sigma_module_t;

sigma_status register_device(const char* device_name, void* operations);
sigma_status load_shard_module(sigma_module_t* module);
sigma_status pq_encrypt(const void* data, sigma_size_t size, void* out_buffer);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SYSCALLS_H */
