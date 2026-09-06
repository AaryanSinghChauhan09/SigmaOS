# SigmaOS API Reference

## Core Modules

### sigma::kernel

```rust
// Process management
pub mod process {
    pub fn fork() -> Result<Pid, SysError>;
    pub fn spawn(path: &str, args: &[&str]) -> Result<Child, SysError>;
    pub fn waitpid(pid: Pid) -> Result<ExitStatus, SysError>;
    pub fn getpid() -> Pid;
    pub fn getppid() -> Pid;
    pub fn kill(pid: Pid, signal: Signal) -> Result<(), SysError>;
}

// Memory management
pub mod memory {
    pub fn mmap(addr: Option<*mut u8>, len: usize, prot: Prot, flags: MapFlags) -> Result<*mut u8, SysError>;
    pub fn munmap(addr: *mut u8, len: usize) -> Result<(), SysError>;
    pub fn mprotect(addr: *mut u8, len: usize, prot: Prot) -> Result<(), SysError>;
    pub fn brk(addr: usize) -> Result<usize, SysError>;
}
```

### sigma::filesystem

```rust
pub mod vfs {
    pub fn open(path: &str, flags: OpenFlags) -> Result<FileDescriptor, VfsError>;
    pub fn read(fd: FileDescriptor, buf: &mut [u8]) -> Result<usize, VfsError>;
    pub fn write(fd: FileDescriptor, buf: &[u8]) -> Result<usize, VfsError>;
    pub fn close(fd: FileDescriptor) -> Result<(), VfsError>;
    pub fn stat(path: &str) -> Result<FileStat, VfsError>;
    pub fn mkdir(path: &str, mode: u32) -> Result<(), VfsError>;
    pub fn unlink(path: &str) -> Result<(), VfsError>;
    pub fn rename(from: &str, to: &str) -> Result<(), VfsError>;
    pub fn readdir(path: &str) -> Result<Vec<DirEntry>, VfsError>;
    pub fn mount(device: &str, mount_point: &str, fstype: &str, flags: MountFlags) -> Result<(), VfsError>;
    pub fn umount(mount_point: &str) -> Result<(), VfsError>;
}
```

### sigma::network

```rust
pub mod socket {
    pub fn socket(family: SocketFamily, type_: SocketType, proto: i32) -> Result<SocketFd, NetError>;
    pub fn bind(fd: SocketFd, addr: &SocketAddr) -> Result<(), NetError>;
    pub fn listen(fd: SocketFd, backlog: i32) -> Result<(), NetError>;
    pub fn accept(fd: SocketFd) -> Result<(SocketFd, SocketAddr), NetError>;
    pub fn connect(fd: SocketFd, addr: &SocketAddr) -> Result<(), NetError>;
    pub fn send(fd: SocketFd, buf: &[u8], flags: i32) -> Result<usize, NetError>;
    pub fn recv(fd: SocketFd, buf: &mut [u8], flags: i32) -> Result<usize, NetError>;
    pub fn close(fd: SocketFd) -> Result<(), NetError>;
}
```

### sigma::crypto

```rust
pub mod pqc {
    // Post-quantum key generation
    pub fn dilithium5_keygen() -> (PublicKey, SecretKey);
    pub fn dilithium5_sign(sk: &SecretKey, msg: &[u8]) -> Signature;
    pub fn dilithium5_verify(pk: &PublicKey, msg: &[u8], sig: &Signature) -> bool;
    
    pub fn kyber1024_keygen() -> (EncapsKey, DecapsKey);
    pub fn kyber1024_encapsulate(ek: &EncapsKey) -> (Ciphertext, SharedSecret);
    pub fn kyber1024_decapsulate(dk: &DecapsKey, ct: &Ciphertext) -> SharedSecret;
}

pub mod symmetric {
    pub fn aes256gcm_encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8], aad: &[u8]) -> Vec<u8>;
    pub fn aes256gcm_decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

pub mod hash {
    pub fn sha256(data: &[u8]) -> [u8; 32];
    pub fn sha3_256(data: &[u8]) -> [u8; 32];
    pub fn blake3(data: &[u8]) -> [u8; 32];
}

pub mod kdf {
    pub fn argon2id(password: &[u8], salt: &[u8; 32], m_cost: u32, t_cost: u32) -> [u8; 32];
    pub fn hkdf_sha256(ikm: &[u8], salt: &[u8], info: &[u8], output: &mut [u8]);
}
```

### sigma::security

```rust
pub mod input_validation {
    pub fn validate_path(path: &[u8]) -> Result<(), ValidationError>;
    pub fn validate_filename(name: &[u8]) -> Result<(), ValidationError>;
    pub fn validate_ipv4(addr: &[u8]) -> Result<(), ValidationError>;
    pub fn validate_ipv6(addr: &[u8]) -> Result<(), ValidationError>;
    pub fn validate_hostname(host: &[u8]) -> Result<(), ValidationError>;
    pub fn validate_username(name: &[u8]) -> Result<(), ValidationError>;
    pub fn checked_add_usize(a: usize, b: usize) -> Result<usize, ValidationError>;
    pub fn checked_mul_usize(a: usize, b: usize) -> Result<usize, ValidationError>;
}

pub mod mac {
    pub fn check_access(subject: &SecurityContext, object: &SecurityContext, perm: Permission) -> bool;
    pub fn get_context(path: &str) -> Result<SecurityContext, MacError>;
    pub fn set_context(path: &str, ctx: &SecurityContext) -> Result<(), MacError>;
}
```

### sigma::package

```rust
pub mod manager {
    pub fn install(packages: &[&str]) -> Result<Transaction, PkgError>;
    pub fn remove(packages: &[&str]) -> Result<Transaction, PkgError>;
    pub fn update_lists() -> Result<(), PkgError>;
    pub fn upgrade_all() -> Result<Transaction, PkgError>;
    pub fn search(query: &str) -> Vec<PackageInfo>;
    pub fn info(name: &str) -> Result<PackageInfo, PkgError>;
    pub fn list_installed() -> Vec<InstalledPackage>;
    pub fn query_file(path: &str) -> Option<String>;
    pub fn verify(name: &str) -> Result<VerifyResult, PkgError>;
    pub fn history() -> Vec<Transaction>;
    pub fn rollback(transaction_id: u64) -> Result<(), PkgError>;
}
```

## Error Types

| Error | Description |
|-------|-------------|
| `SysError` | System call error |
| `VfsError` | Filesystem error |
| `NetError` | Network error |
| `CryptoError` | Cryptographic operation error |
| `PkgError` | Package manager error |
| `ValidationError` | Input validation error |
| `MacError` | MAC policy violation |
| `DriverError` | Hardware driver error |
| `KernelError` | Kernel internal error |

## Constants

```rust
// Common permissions
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32   = 2;
pub const O_CREAT: u32  = 0x40;
pub const O_TRUNC: u32  = 0x200;
pub const O_APPEND: u32 = 0x400;

// Signal numbers
pub const SIGTERM: i32 = 15;
pub const SIGKILL: i32 = 9;
pub const SIGSTOP: i32 = 19;
pub const SIGCONT: i32 = 18;
```
