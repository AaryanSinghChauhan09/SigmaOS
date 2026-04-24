
# Memory-as-Contracts


SigmaOS introduces a radical USP: **Cryptographic Memory Leasing**.
Located in `modules/core/kernel/mem_contracts.c`.

Instead of blindly allocating memory via `malloc()`, processes negotiate a cryptographically signed contract with the kernel.


## How it Works

1. **The Lease**: Process requests N pages. Kernel generates a contract containing a `base_page`, `num_pages`, `expiry_tick`, and a cryptographic `signature`.
2. **Dynamic Quotas**: A contract can specify a strict expiration time.
3. **Verification**: When memory is accessed (or validated via the Capability Scheduler), the kernel verifies the contract's signature to ensure the process actually holds a valid lease.
4. **Revocation Hooks**: The scheduler automatically revokes the contract when the `expiry_tick` is reached, stripping the process's capability to access those physical pages.


## Accountability

Every contract issuance and expiration is logged into the **Tamper-Proof Audit Chain**, establishing a permanent, unforgeable history of which process owned which block of memory at any given nanosecond.


## The Economic Model

This forms the basis for a future **resource economy**. CPU cycles, I/O bandwidth, and memory can all be "leased" via internal sovereign tokens, where runaway processes simply run out of tokens and have their memory contracts safely revoked by the microkernel.
