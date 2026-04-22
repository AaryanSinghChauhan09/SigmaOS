# 🔌 SigmaOS: Inter-Process Communication (IPC) Deep-Dive

## 🔳 The Sovereign Message Bus (S00)
The heart of the Sovereign Lattice's modularity is the **Message Bus**. Unlike monolithic kernels where subsystems share memory without boundaries, SigmaOS enforces strict communication through the IPC layer.

## ⚙️ How it Works
1. **Addressing**: Every suite (S00–S33) has a unique identity. Shards within a suite inherit the suite's identity for routing.
2. **Message Format**:
   - `SenderID`: Source suite.
   - `ReceiverID`: Destination suite.
   - `Type`: Message opcode.
   - `Payload`: 256-byte static buffer (zero-copy when possible).

## 🛡️ Zero-Trust Enforcement
The **S10 Registry** and **S08 Compliance** suites audit every message. 
- **Namespacing**: Shards are confined to their namespace. Handshakes between namespaces require cryptographic verification via the **S30 Supremacy Signature**.
- **Auditing**: Every cross-suite message is recorded by the **Sovereign Audit Tool**.

## 🚀 Future Performance: Hyper-Link
For low-latency execution, the **S20 Interconnect** provides "Hyper-Link" channels that bypass the standard bus for atomic synchronization between high-throughput clusters (e.g., SIMD to Transcendence).

---
**Architect Tip:** Keep messages small. For large data transfers, use the **S01 Modular Paging** service to pass page ownership.
