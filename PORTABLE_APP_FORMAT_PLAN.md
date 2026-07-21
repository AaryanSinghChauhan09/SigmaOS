# 📦 Portable App Format Plan (SigmaAppImage) for SigmaOS

This document specifies the format blueprint for SigmaAppImage, a completely decentralized, zero-dependency, self-contained portable package structure for SigmaOS.

---

## 1. Content-Addressed Sandboxed Mounting
SigmaAppImages are compiled as a unified binary containing both the metadata structure and compressed payload. Upon execution, the OS dynamically mounts the binary in a read-only VFS sandbox.

### Rust Implementation (Package Frame Parser)
```rust
pub struct AppImageHeader {
    pub magic: [u8; 4],
    pub payload_offset: u64,
    pub payload_size: u64,
    pub security_bits: u64,
}

impl AppImageHeader {
    pub fn parse(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 24 {
            return Err("Header buffer too small");
        }
        let magic = [buffer[0], buffer[1], buffer[2], buffer[3]];
        if magic != [0x53, 0x41, 0x49, 0x4D] { // "SAIM" magic signature
            return Err("Invalid AppImage magic signature");
        }
        Ok(Self {
            magic,
            payload_offset: u64::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7], buffer[8], buffer[9], buffer[10], buffer[11]]),
            payload_size: u64::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15], buffer[16], buffer[17], buffer[18], buffer[19]]),
            security_bits: u64::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23], buffer[24], buffer[25], buffer[26], buffer[27]]),
        })
    }
}
```
