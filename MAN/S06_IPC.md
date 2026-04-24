
# S-IPC(7) | SigmaOS Inter-Shard Communication



## NAME

s-ipc - Persistence-backed, capability-based communication lattice.


## DESCRIPTION

**s-ipc** provides the primary communication bus for the SigmaOS microkernel. It utilizes **Atomic Sequence-IDs** and a **Monotonic Replay Log** to ensure crash-resilient messaging between shards.


## CAPABILITIES

All communication is protected by capabilities. A shard must possess the `CAP_IPC_SEND` or `CAP_IPC_RECV` token for a specific channel to interact with it.


## PERSISTENCE

If **S06_Persistence** is active, all IPC messages are logged to the Sovereign Replay Buffer. In the event of a system crash, **S-IPC** will automatically replay the sequence-ID log to restore shard state consistency.


## SEE ALSO

`s-cli(1)`, `s-init(1)`
