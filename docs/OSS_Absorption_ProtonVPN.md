# 🧩 Cleanroom Absorption: ProtonVPN

SigmaOS implements **SigmaTunnel VPN**, a highly secure, quantum-resistant virtual private network protocol stack built directly into the microkernel.

---

## 🎯 Target Architecture: ProtonVPN

ProtonVPN focuses on multi-hop Secure Core routing, zero-logs policy, automatic kill-switches, DNS leak protection, and NetShield content blocking.

### Gaps in Legacy ProtonVPN:
- Runs entirely in user-space with significant context-switching penalties.
- Uses legacy cryptographic key exchanges vulnerable to future quantum decryption.
- Relies on standard OS socket interfaces prone to DNS and routing leaks.

---

## 🔒 SigmaOS Sovereign Features

### 1. Quantum-Safe Cryptography
- Integrates CRYSTALS-Kyber (Kyber-1024) for quantum-resistant key encapsulation during the initial handshake, and Dilithium-5 for session signing.

### 2. Microkernel Socket Integration
- Built directly into the zero-trust IP routing stack, guaranteeing zero DNS leaks and implementing an absolute kernel-enforced hardware Kill-Switch.

### 3. Per-Process Isolation
- Processes must possess an authorized `CapabilityToken` with network binding rights to route outside the VPN tunnel.

---

## 📊 Absorption Matrix

| Capability | ProtonVPN | SigmaTunnel VPN |
|------------|-----------|-----------------|
| Zero-Logs Core | ✅ | ✅ Enforced via immutable logging |
| Ad / Tracker Filtering | ✅ (NetShield) | ✅ DNS Lattice Level |
| Secure Core Multi-hop | ✅ | ✅ |
| Post-Quantum Cryptography | ❌ | ✅ Kyber-1024 |
| Microkernel Level Performance | ❌ | ✅ Zero-copy context switching |
| Zero DNS Leaks | Partial | ✅ Kernel-Guaranteed |
