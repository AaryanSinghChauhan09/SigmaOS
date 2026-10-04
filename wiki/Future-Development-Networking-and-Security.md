# Future Development: Networking and Security

**Status:** Proposal. Compatibility models and policy data are not equivalent to packet processing, syscall enforcement, or cryptographic verification.

## Scope

Connect packet filtering, network namespaces, remote communication, and process confinement through explicit enforcement points. Existing code areas include [`network`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/network), [`net`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/net), [`capsicum.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/capsicum.rs), [`landlock.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/landlock.rs), and [`pledge.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/pledge.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux | nftables/eBPF hooks, network namespaces, and WireGuard | Where are packet and namespace policies enforced, and how are hooks bounded? |
| OpenBSD | PF, pledge, unveil, and conservative defaults | Can policy be expressed simply and fail closed at syscall and packet boundaries? |
| FreeBSD | Capsicum capabilities, jails, and VNET | Can process and network isolation share explicit resource ownership? |
| NetBSD | NPF and rump-kernel isolation | Can network components be tested independently of privileged hardware? |

## Proposed work sequence

1. **Map the data path.** Document packet ingress through routing, filtering, sockets, and egress; identify which components currently enforce policy.
2. **Separate rule storage from enforcement.** A ruleset editor or firewall model must not be described as filtering traffic unless packets pass through it.
3. **Define isolation boundaries.** Specify namespace ownership, capability inheritance, teardown, and behavior when a process exits unexpectedly.
4. **Harden parsing.** Apply length, state, and resource limits to protocol parsers; add fuzz targets for untrusted packets and policy input.
5. **Integrate cryptography through providers.** Use reviewed implementations with explicit key handling, entropy requirements, and signature verification. Until then, report the provider as unavailable.

## Completion criteria

- Deny-by-default rules are enforced on the actual packet path and have explicit precedence semantics.
- Namespace and capability checks are applied at every relevant boundary, not only represented in configuration structs.
- Invalid rules, packets, credentials, and keys fail closed and do not return simulated success.
- Network and crypto claims identify the integrated provider and describe supported algorithms accurately.
- Security documentation distinguishes policy models, prototypes, and runtime enforcement.

## Maintenance

Record threat model changes and enforcement call sites. Do not call an interface a sandbox, firewall, VPN, or verifier until it controls the corresponding runtime operation.
