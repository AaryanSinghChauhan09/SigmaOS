# SigmaOS Subsystem API Specifications

This document outlines the API definitions and structures for System Calls, Filesystems, and the Networking Stack in SigmaOS Zenith.

---

## 1. System Call Interface (Syscalls)

All userland interactions with the kernel are handled via the syscall table:

### `sigma_write`
```c
sigma_u64 sigma_write(sigma_i32 fd, const void* buf, sigma_size_t count);
```
- **Description**: Outputs character string buffer directly to the debug terminal device or serial logger.
- **Return**: Number of bytes written on success, negative value on failure.

### `sigma_read`
```c
sigma_u64 sigma_read(sigma_i32 fd, void* buf, sigma_size_t count);
```
- **Description**: Suspends thread execution and reads characters from the PS/2 keyboard buffer.
- **Return**: Bytes read on success, negative error code on failure.

---

## 2. Virtual File System (VFS) APIs

All filesystem drivers must mount their interfaces onto the VFS node structures:

```c
typedef struct vfs_node {
    char name[128];
    sigma_u32 inode_id;
    sigma_size_t size;
    sigma_u32 flags; // File, Directory, Block Device, Character Device
    
    // Callback functions mapping directly to drivers
    sigma_i32 (*read)(struct vfs_node* node, void* buf, sigma_size_t size, sigma_u64 offset);
    sigma_i32 (*write)(struct vfs_node* node, const void* buf, sigma_size_t size, sigma_u64 offset);
} vfs_node_t;
```

### Ext4 Driver Operations
- **`init_ext4`**: Reads block sector 2 to look up superblock properties and verify the `0xEF53` signature.
- **`ext4_read`**: Resolves inode numbers and copies raw blocks from memory segments.

### FAT32 Driver Operations
- **`init_fat32`**: Analyzes the active Extended Boot Record (EBR) mapping data sectors.
- **`fat32_read_file`**: Resolves cluster chain offsets and reads sequential cluster data.

---

## 3. Networking APIs

POSIX-inspired socket communication wrappers:

### Socket Allocation
```c
sigma_i32 net_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol);
```
- **Description**: Allocates a new socket connection slot in the socket list table.
- **Return**: Assigned file descriptor (FD) number, or `-1` if the table is full.

### Establish Connection
```c
sigma_i32 net_connect(sigma_i32 fd, sigma_u32 remote_ip, sigma_u16 remote_port);
```
- **Description**: Initiates the TCP 3-way handshake state machine (SYN -> SYN-ACK -> ACK) to connect the socket to a destination address.
- **Return**: `0` on success, or `-1` if the host is unreachable.

### Data Transmission
```c
sigma_i32 net_send(sigma_i32 fd, const void* data, sigma_size_t size);
```
- **Description**: Packets are routed through the virtual loopback device (`127.0.0.1`) and sent to the destination receive buffers.
- **Return**: Number of bytes sent, or `-1` if the transmission failed.
