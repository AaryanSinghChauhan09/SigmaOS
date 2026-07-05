# smoltcp TCP/IP Stack Integration

## Overview

[smoltcp](https://github.com/smoltcp-rs/smoltcp) (MIT) is a pure-Rust, `no_std`-compatible TCP/IP stack. SigmaOS uses smoltcp as the **canonical networking implementation** for the microkernel, RTOS, and embedded profiles, replacing the `kernel/net/` placeholder stubs.

---

## Why smoltcp

| Property | Value |
|---|---|
| Language | Pure Rust, zero unsafe networking code |
| `no_std` | Yes — usable in bare-metal RTOS profile |
| License | MIT — compatible with SigmaOS MIT stack |
| Protocols | TCP, UDP, ICMP, IPv4/v6, DHCPv4, DNS |
| Performance | Suitable for embedded; not line-rate on 10G |

For the cloud profile (where line-rate matters), a DPDK-backed ring adapter can sit below smoltcp while keeping the same API surface.

---

## Feature Flags (Cargo.toml)

```toml
[dependencies]
smoltcp = { version = "=0.11.0", default-features = false, features = [
    "medium-ethernet",
    "proto-ipv4",
    "proto-ipv6",
    "socket-tcp",
    "socket-udp",
    "socket-icmp",
    "socket-dns",
    "proto-dhcpv4",
    "async",
] }
```

---

## Interface Adapter: SigmaOS HAL NIC → smoltcp Device Trait

The SigmaOS HAL exposes a `NicDevice` trait. The adapter wraps it into smoltcp's `Device` trait:

```rust
// kernel/net/smoltcp_adapter.rs

use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;
use crate::hal::NicDevice;

pub struct SigmaSmoltcpDevice<N: NicDevice> {
    nic: N,
}

impl<N: NicDevice> Device for SigmaSmoltcpDevice<N> {
    type RxToken<'a> = SigmaRxToken where N: 'a;
    type TxToken<'a> = SigmaTxToken<'a, N> where N: 'a;

    fn receive(&mut self, _timestamp: Instant)
        -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)>
    {
        if let Some(pkt) = self.nic.poll_rx() {
            Some((SigmaRxToken(pkt), SigmaTxToken { nic: &mut self.nic }))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(SigmaTxToken { nic: &mut self.nic })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1500;
        caps
    }
}

pub struct SigmaRxToken(Vec<u8>);
impl RxToken for SigmaRxToken {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(mut self, f: F) -> R {
        f(&mut self.0)
    }
}

pub struct SigmaTxToken<'a, N: NicDevice> { nic: &'a mut N }
impl<'a, N: NicDevice> TxToken for SigmaTxToken<'a, N> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut buf = vec![0u8; len];
        let r = f(&mut buf);
        self.nic.transmit(&buf);
        r
    }
}
```

---

## smoltcp Interface Setup (QEMU virtio-net)

```rust
// kernel/net/init.rs

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};

pub fn init_networking(nic: impl NicDevice) -> Interface {
    let device = SigmaSmoltcpDevice { nic };
    let mac = EthernetAddress([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    let config = Config::new(mac.into());
    let mut iface = Interface::new(config, &mut device, smoltcp::time::Instant::ZERO);
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs.push(IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)).unwrap();
    });
    iface.routes_mut().add_default_ipv4_route(Ipv4Address::new(10, 0, 2, 2)).unwrap();
    iface
}
```

---

## Profiles That Use smoltcp

| Profile | Use case |
|---|---|
| Microkernel | Primary TCP/IP — all kernel net traffic |
| RTOS | UDP + ICMP only (minimal profile) |
| Embedded | `no_std` bare-metal networking |
| Cloud | smoltcp + DPDK ring adapter (high throughput) |

---

## Exit Criteria

- `ping 8.8.8.8` works from `sigma-sh` in QEMU (virtio-net backend).

- `sigma-curl https://example.com` completes an HTTP GET using smoltcp + rustls.

- `cargo test -p sigma-net` passes all smoltcp integration tests.
