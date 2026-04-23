# Sovereign Token Resource Economy

SigmaOS implements an internal resource economy where every process must hold a cryptographically signed token to access CPU cycles, memory pages, or I/O bandwidth.

Located in `modules/core/kernel/sovereign_tokens.c`.

## Token Types

| Token | Constant | Controls |
| :--- | :--- | :--- |
| Memory Token | `TOKEN_TYPE_MEMORY` | Physical page access rights |
| CPU Token | `TOKEN_TYPE_CPU` | Nanoseconds of CPU time |
| I/O Token | `TOKEN_TYPE_IO` | Bytes of I/O bandwidth |

## Lifecycle

```
MINT → SPEND → EXPIRE (or DELEGATE)
```

1. **Mint**: The kernel issues a token via `token_mint()`, stamping it with a cryptographic signature. No process can forge a token.
2. **Spend**: A process authorizes resource use via `token_spend()`. The signature is re-verified before access is granted.
3. **Delegate**: A process can transfer part of its token to another via `token_delegate()` — enabling secure, capability-bounded resource sharing between processes.
4. **Expire**: The scheduler's `token_enforce_expiries()` hook automatically revokes tokens when their deadline is reached. All expiry events are logged in the Tamper-Proof Audit Chain.

## Integration with Memory-as-Contracts

Memory contracts (`mem_contracts.c`) can require a corresponding `TOKEN_TYPE_MEMORY` to be held before issuing a physical page lease, making the entire memory system fully accountable and zero-trust by default.
