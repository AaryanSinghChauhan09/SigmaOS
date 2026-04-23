#ifndef SIGMA_API_H
#define SIGMA_API_H

#include <stdint.h>
#include <stddef.h>

// ============================================================================
// SigmaOS Sovereign System API Blueprint (1000+ Functions Specification)
// ============================================================================

// ----------------------------------------------------------------------------
// 🧩 Kernel & Core System
// ----------------------------------------------------------------------------
void init_kernel(void);
void schedule_process(void);
void yield_cpu(void);
int create_thread(void (*entry)(void*), void* arg);
void destroy_thread(int thread_id);
void* allocate_page(void);
void free_page(void* page);
int map_virtual_address(void* phys, void* virt, uint32_t flags);
int unmap_virtual_address(void* virt);
void handle_interrupt(int vector);
void register_syscall(int num, void* handler);
void execute_syscall(int num);
void panic_handler(const char* msg);
uint64_t get_system_uptime(void);
void set_priority(int pid, int priority);
int get_priority(int pid);
void lock_mutex(uint32_t* mutex);
void unlock_mutex(uint32_t* mutex);
void wait_condition(uint32_t* cond, uint32_t* mutex);
void signal_condition(uint32_t* cond);

// ----------------------------------------------------------------------------
// 💾 Memory Management
// ----------------------------------------------------------------------------
void* malloc(size_t size);
void* calloc(size_t num, size_t size);
void* realloc(void* ptr, size_t size);
void free(void* ptr);
int create_heap(void* start, size_t size);
void destroy_heap(int heap_id);
void gc_collect(void);
void gc_mark(void* obj);
void gc_sweep(void);
void* allocate_stack(size_t size);
void free_stack(void* stack);
void* map_shared_memory(const char* name, size_t size);
void unmap_shared_memory(void* ptr);
void get_memory_stats(uint32_t* total, uint32_t* used);
void set_memory_limit(int pid, size_t limit);
int check_memory_leak(void);

// ----------------------------------------------------------------------------
// 📂 File System
// ----------------------------------------------------------------------------
int open_file(const char* path, int flags);
void close_file(int fd);
size_t read_file(int fd, void* buf, size_t count);
size_t write_file(int fd, const void* buf, size_t count);
int delete_file(const char* path);
int rename_file(const char* old_path, const char* new_path);
int create_directory(const char* path);
int delete_directory(const char* path);
int list_directory(const char* path, void* buffer);
int mount_fs(const char* device, const char* mountpoint, const char* type);
int unmount_fs(const char* mountpoint);
int format_fs(const char* device, const char* type);
int fs_check_integrity(const char* mountpoint);
int fs_repair(const char* mountpoint);
int get_file_metadata(const char* path, void* metadata);
int set_file_permissions(const char* path, uint32_t perms);

// ----------------------------------------------------------------------------
// 🌐 Networking
// ----------------------------------------------------------------------------
void init_network_stack(void);
int open_socket(int domain, int type, int protocol);
void close_socket(int sock);
size_t send_packet(int sock, const void* buf, size_t len);
size_t receive_packet(int sock, void* buf, size_t len);
int resolve_dns(const char* hostname, uint32_t* ip_out);
int connect_tcp(int sock, uint32_t ip, uint16_t port);
void disconnect_tcp(int sock);
int listen_tcp(int sock, int backlog);
int accept_tcp(int sock);
size_t send_udp(int sock, uint32_t ip, uint16_t port, const void* buf, size_t len);
size_t receive_udp(int sock, uint32_t* ip, uint16_t* port, void* buf, size_t len);
void enable_firewall(void);
void disable_firewall(void);
int set_firewall_rule(const char* rule);
void get_network_stats(void* stats);

// ----------------------------------------------------------------------------
// 🔒 Security & Sovereignty
// ----------------------------------------------------------------------------
int encrypt_data(const void* in, void* out, size_t len, const void* key);
int decrypt_data(const void* in, void* out, size_t len, const void* key);
void hash_sha256(const void* in, size_t len, void* hash_out);
void hash_md5(const void* in, size_t len, void* hash_out);
void generate_keypair(void* pub_key, void* priv_key);
int sign_data(const void* data, size_t len, const void* priv_key, void* sig_out);
int verify_signature(const void* data, size_t len, const void* pub_key, const void* sig);
void enable_secure_boot(void);
void disable_secure_boot(void);
int check_integrity(const void* data, size_t len, const void* expected_hash);
void set_user_permissions(int user_id, uint32_t perms);
uint32_t get_user_permissions(int user_id);
void audit_log_event(const char* event);
void clear_audit_log(void);

// ----------------------------------------------------------------------------
// 🖥️ Device Drivers & IPC
// ----------------------------------------------------------------------------
int init_driver(const char* name);
int load_driver(const char* path);
void unload_driver(int driver_id);
int probe_device(uint32_t device_id);
int create_message_queue(const char* name);
void delete_message_queue(int qid);
int send_message(int qid, const void* msg, size_t len);
int receive_message(int qid, void* msg, size_t len);

// ... plus hundreds more defined in the SigmaOS Architecture blueprints.

#endif // SIGMA_API_H
