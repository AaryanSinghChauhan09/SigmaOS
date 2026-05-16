# S-MAC: Mandatory Access Control

SigmaOS enforces a zero-trust security model via the **Sovereign MAC** shard.

## Architecture

Every shard in the lattice is assigned a **Capability Token**. Access to resources (Memory, I/O, Network) is validated in real-time against the **Security Lattice**.

## Features

- **PQC Policies**: Security policies are signed using CRYSTALS-Dilithium to prevent adversarial tampering.

- **Micro-Segmentation**: Shards are isolated from each other by default; communication requires an explicit "Lattice Bridge".

- **Audit Logging**: Every access request is logged to the `S-LOG` shard for forensic analysis.
