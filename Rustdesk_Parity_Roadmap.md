# 🧩 SigmaOS RustDesk Parity & Remote Desktop Roadmap

This document outlines the architectural strategy, design specification, and implementation details for the **Sovereign Remote Desktop (SigmaDesk)** in **SigmaOS**, bridging and surpassing the capabilities of RustDesk.

***

## 🗺️ 1. Paradigm Vision: SigmaDesk vs. RustDesk

RustDesk is an excellent open-source remote desktop tool, but it requires a centralized set of rendezvous and signal proxy servers (hbbs and hbbr) to perform NAT traversal (STUN/TURN) and session peer lookups. This introduces external network dependencies and latency bottlenecks.

**SigmaOS** leapfrogs RustDesk by embedding **Native P2P Traversal and Zero-Trust Control** directly into the operating system core:

```text
  +---------------------------------------------------------------------------------+
  |                               SigmaOS Remote Core                               |
  |                                                                                 |
  |   +-------------------+   +--------------------+   +------------------------+   |
  |   |  SigmaRendezvous  |   |   PqcVideoCipher   |   |      InputAuthGate     |   |
  |   | (P2P Holepunch)   |   | (Kyber Encryption) |   |  (Capability-Gated)    |   |
  |   +-------------------+   +--------------------+   +------------------------+   |
  +---------------------------------------------------------------------------------+
```

*   **Serverless P2P Signaling**: The `SigmaRendezvous` peer discovery network negotiates connection handshakes and traverses NAT boundaries directly between peers, completely bypassing centralized proxy servers.
*   **Post-Quantum Frame Ciphers**: Screen buffer streaming is dynamically protected using the `PqcVideoCipher` stream module, preventing man-in-the-middle decoding.
*   **Capability-Gated Input Injection**: Prevents malicious remote takeover. The `InputAuthGate` verifies the remote operator's secure `CapabilityToken` before injecting keystrokes or mouse clicks.

***

## 🏗️ 2. Core Architecture Blocks

### 2.1 SigmaRendezvous (`SigmaRendezvous`)

*   **Mission**: Establish reliable peer-to-peer tunnels without centralized intermediate proxies.
*   **Mechanism**: Automatically negotiates STUN/TURN hole-punching vectors directly.

### 2.2 Post-Quantum Video Cipher (`PqcVideoCipher`)

*   **Mission**: Zero-Trust secure frame streaming.
*   **Benefit**: Dynamically ciphers screen pixels in real-time before transmission.

### 2.3 Input Authorization Gate (`InputAuthGate`)

*   **Mission**: Protect the terminal against unauthorized remote shell takeover.
*   **Benefit**: Continuously authenticates remote operator input streams using capability token verification.
