# Contributing to SigmaOS

SigmaOS is a sovereign project. We value industrial-grade code, zero-dependency engineering, and transparency.

## ⚖️ General Guidelines

- **Zero-Dependency**: Do not include external monolithic libraries (glibc, boost, etc.).

- **Silicon-Up**: Write code that interacts directly with the lattice/hal when possible.

- **PQC-First**: All networked shards must use Dilithium-5/Kyber-1024 signing.

## 🛠 Branching Strategy

- `main`: The Industrial Gold build. Stable and certified.

- `rolling`: Continuous updates and experimental shards.

- `release/*`: Format-specific production builds.

## 📝 Commit Standards

- Use imperative mood ("feat: add shard" not "added shard").

- Reference the shard ID (e.g., `[S-NET]`).

## 🖇 Pull Request Process

1. Synchronize your local lattice with the current `rolling` branch.

2. Run the Matrix Testing Algorithm (`tests/matrix_test.py`).

3. Ensure 100% documentation parity in the Wiki.

## 🛡 Security Reports

Report vulnerabilities directly to the Sovereign Security Shard (`security@sigmaos.org`) using PQC-GPG encryption.

---

### Your contributions define the future of sovereignty
