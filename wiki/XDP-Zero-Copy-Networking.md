# XDP Zero-Copy Networking

SigmaOS implements XDP (eXpress Data Path) for ultra-high-performance packet processing at the earliest possible point in the networking stack, enabling zero-copy packet filtering and processing.

## Overview

XDP provides:
- Zero-copy packet processing
- Early packet processing (before socket buffer allocation)
- eBPF program attachment
- Multiple operation modes (native, skb, offloaded)
- Per-CPU processing queues
- Drop, pass, redirect, and TX operations
- Hardware offload support

## Architecture

### XDP Processing Pipeline
```
NIC → XDP Program → eBPF JIT → Action
                      ↓
           Drop / Pass / Redirect / TX
```

### XDP Modes
- **XDP_ABORTED**: Drop packet and raise exception
- **XDP_DROP**: Silently drop packet
- **XDP_PASS**: Pass packet to normal stack
- **XDP_TX**: Transmit packet out same interface
- **XDP_REDIRECT**: Redirect to another interface or socket

## Implementation

### XDP Program Loader
```rust
// src/networking/xdp/loader.rs
pub struct XdpLoader {
    programs: BTreeMap<i32, XdpProgram>,
    ifaces: BTreeMap<String, i32>,
}

#[derive(Debug, Clone)]
pub struct XdpProgram {
    pub id: i32,
    pub bytecode: Vec<u8>,
    pub jit_code: Option<Vec<u8>>,
    pub mode: XdpMode,
    pub if_index: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpMode {
    Native,
    Skb,
    Offloaded,
}

impl XdpLoader {
    pub fn load(&mut self, bytecode: Vec<u8>, mode: XdpMode) -> Result<i32, XdpError> {
        // Verify eBPF program
        let verifier = EbpfVerifier::new();
        verifier.verify(&bytecode)?;
        
        // JIT compile
        let jit_code = self.jit_compile(&bytecode)?;
        
        // Load into kernel
        let prog_fd = unsafe {
            bpf_prog_load(BPF_PROG_TYPE_XDP, &bytecode)
        };
        
        if prog_fd < 0 {
            return Err(XdpError::LoadFailed);
        }
        
        let program = XdpProgram {
            id: prog_fd,
            bytecode,
            jit_code: Some(jit_code),
            mode,
            if_index: None,
        };
        
        let id = program.id;
        self.programs.insert(id, program);
        Ok(id)
    }

    pub fn attach(&mut self, prog_id: i32, iface: &str) -> Result<(), XdpError> {
        let program = self.programs.get_mut(&prog_id)
            .ok_or(XdpError::NotFound)?;
        
        // Get interface index
        let if_index = self.get_if_index(iface)?;
        program.if_index = Some(if_index);
        
        // Attach XDP program to interface
        let result = unsafe {
            bpf_set_link_xdp_fd(prog_id, program.mode as i32, if_index)
        };
        
        if result < 0 {
            return Err(XdpError::AttachFailed);
        }
        
        self.ifaces.insert(iface.to_string(), prog_id);
        Ok(())
    }

    pub fn detach(&mut self, iface: &str) -> Result<(), XdpError> {
        let if_index = self.get_if_index(iface)?;
        
        // Detach XDP program
        let result = unsafe {
            bpf_set_link_xdp_fd(-1, 0, if_index)
        };
        
        if result < 0 {
            return Err(XdpError::DetachFailed);
        }
        
        self.ifaces.remove(iface);
        Ok(())
    }
}
```

### XDP Context
```rust
// src/networking/xdp/context.rs
pub struct XdpContext {
    pub data: *mut u8,
    pub data_end: *mut u8,
    pub data_meta: *mut u8,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
}

impl XdpContext {
    pub fn packet_length(&self) -> usize {
        (self.data_end as usize) - (self.data as usize)
    }

    pub fn read_bytes(&self, offset: usize, buf: &mut [u8]) -> Result<usize, XdpError> {
        let packet_len = self.packet_length();
        if offset >= packet_len {
            return Ok(0);
        }
        
        let available = packet_len - offset;
        let to_read = buf.len().min(available);
        
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.data.add(offset),
                buf.as_mut_ptr(),
                to_read
            );
        }
        
        Ok(to_read)
    }

    pub fn adjust_head(&mut self, offset: i32) -> Result<(), XdpError> {
        if offset < 0 {
            return Err(XdpError::InvalidOffset);
        }
        
        unsafe {
            *self.data_meta = offset as u8;
        }
        
        Ok(())
    }
}
```

### XDP Operations
```rust
// src/networking/xdp/ops.rs
impl XdpContext {
    pub fn pass(&self) -> XdpAction {
        XdpAction::Pass
    }

    pub fn drop(&self) -> XdpAction {
        XdpAction::Drop
    }

    pub fn tx(&self) -> XdpAction {
        XdpAction::Tx
    }

    pub fn redirect(&self, if_index: u32) -> XdpAction {
        XdpAction::Redirect(if_index)
    }

    pub fn redirect_map(&self, map_fd: i32, key: u32) -> XdpAction {
        XdpAction::RedirectMap(map_fd, key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Aborted,
    Drop,
    Pass,
    Tx,
    Redirect(u32),
    RedirectMap(i32, u32),
}
```

### Packet Parsing Helpers
```rust
// src/networking/xdp/parsing.rs
impl XdpContext {
    pub fn parse_ethernet(&self) -> Result<EthernetHeader, XdpError> {
        let mut header = [0u8; 14];
        self.read_bytes(0, &mut header)?;
        
        Ok(EthernetHeader {
            dst_mac: [header[0], header[1], header[2], header[3], header[4], header[5]],
            src_mac: [header[6], header[7], header[8], header[9], header[10], header[11]],
            ether_type: u16::from_be_bytes([header[12], header[13]]),
        })
    }

    pub fn parse_ip(&self, offset: usize) -> Result<IpHeader, XdpError> {
        let mut header = [0u8; 20];
        self.read_bytes(offset, &mut header)?;
        
        Ok(IpHeader {
            version_ihl: header[0],
            tos: header[1],
            total_length: u16::from_be_bytes([header[2], header[3]]),
            identification: u16::from_be_bytes([header[4], header[5]]),
            flags_fragment: u16::from_be_bytes([header[6], header[7]]),
            ttl: header[8],
            protocol: header[9],
            checksum: u16::from_be_bytes([header[10], header[11]]),
            src_addr: u32::from_be_bytes([header[12], header[13], header[14], header[15]]),
            dst_addr: u32::from_be_bytes([header[16], header[17], header[18], header[19]]),
        })
    }

    pub fn parse_tcp(&self, offset: usize) -> Result<TcpHeader, XdpError> {
        let mut header = [0u8; 20];
        self.read_bytes(offset, &mut header)?;
        
        Ok(TcpHeader {
            src_port: u16::from_be_bytes([header[0], header[1]]),
            dst_port: u16::from_be_bytes([header[2], header[3]]),
            seq_num: u32::from_be_bytes([header[4], header[5], header[6], header[7]]),
            ack_num: u32::from_be_bytes([header[8], header[9], header[10], header[11]]),
            data_offset: header[12] >> 4,
            flags: header[13],
            window: u16::from_be_bytes([header[14], header[15]]),
            checksum: u16::from_be_bytes([header[16], header[17]]),
            urgent_ptr: u16::from_be_bytes([header[18], header[19]]),
        })
    }
}
```

## Configuration

### XDP Configuration
```toml
# /etc/sigmaos/xdp.toml
[defaults]
mode = "native"
queue_count = 4
zero_copy = true

[interfaces]
# Per-interface XDP settings
eth0 = { mode = "native", queue_count = 4 }
eth1 = { mode = "skb", queue_count = 2 }

[performance]
# Performance tuning
batch_size = 64
busy_poll = true
busy_timeout_ms = 20
```

### Runtime Control
```bash
# Load XDP program
sigxdp load program.o

# Attach to interface
sigxdp attach eth0 <prog_id>

# Detach from interface
sigxdp detach eth0

# View XDP statistics
sigxdp stats eth0

# View loaded programs
sigxdp list

# Set XDP mode
sigxdp set-mode eth0 native

# View per-CPU statistics
sigxdp per-cpu eth0
```

## Performance Optimization

### Native Mode
Use native mode for best performance:
```bash
sigxdp set-mode eth0 native
```

### Per-CPU Queues
Increase queue count for multi-core systems:
```bash
sigxdp set-queues eth0 8
```

### Busy Polling
Enable busy polling for ultra-low latency:
```bash
sigxdp set-busy-poll eth0 true
sigxdp set-busy-timeout eth0 20
```

### Zero-Copy
Ensure zero-copy is enabled:
```bash
sigxdp set-zero-copy eth0 true
```

## Troubleshooting

### Program Load Fails
If XDP program fails to load:
1. Check eBPF verification: `sigxdp verify program.o`
2. Check program size: `sigxdp check-size program.o`
3. Review program complexity
4. Check kernel XDP support

### Attach Fails
If attach fails:
1. Check interface exists: `ip link show eth0`
2. Check if XDP already attached: `sigxdp status eth0`
3. Check driver XDP support
4. Try different mode: `sigxdp set-mode eth0 skb`

### Poor Performance
If XDP performance is poor:
1. Check mode: `sigxdp status eth0`
2. Enable native mode: `sigxdp set-mode eth0 native`
3. Increase queue count: `sigxdp set-queues eth0 8`
4. Enable busy polling: `sigxdp set-busy-poll eth0 true`
5. Check CPU affinity

### High CPU Usage
If CPU usage is high:
1. Disable busy polling: `sigxdp set-busy-poll eth0 false`
2. Reduce queue count: `sigxdp set-queues eth0 2`
3. Check for program complexity
4. Use skb mode instead of native

---

**[Networking](Category-Networking)** | **[eBPF JIT](eBPF-JIT-Compilation)** | **[Firewall](PF-Firewall)**
