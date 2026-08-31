# SigmaOS Zero-Trust Networking

## Overview

SigmaOS implements a full Zero-Trust network architecture based on the principle: **"Never trust, always verify"**. All network connections—even internal ones—are authenticated, authorized, and encrypted.

## Architecture

    +------------------+     mTLS      +--------------------+
    |  User Process    |<------------>|  NetworkBolt Agent  |
    +------------------+              +--------------------+
                                              |
                                        Policy Engine
                                              |
                                  +-------------------+
                                  |  Zero-Trust Proxy |
                                  +-------------------+
                                              |
                                      External Network

## Core Components

### 1. NetworkBolt Daemon (`src/network/zero_trust.rs`)

*   Intercepts all outbound connections from user processes
*   Enforces per-process, per-user network policies
*   Performs mTLS certificate validation
*   Logs all connection attempts to Sentinel SIEM

### 2. Policy Engine

*   YAML-based policy definitions in `/etc/sigma/network-policies/`
*   Supports allowlist / denylist rules
*   Integrates with SELinux labels and process capabilities
*   Dynamic policy updates without restart

### 3. Certificate Infrastructure

*   Per-process client certificates (Kyber-768 post-quantum)
*   Automatic certificate rotation every 24h
*   OCSP stapling for revocation checks
*   Hardware TPM binding for certificate storage

## Configuration

### Basic Policy Example (`/etc/sigma/network-policies/default.yaml`)

```yaml
version: v1
kind: NetworkPolicy
spec:
  default_action: deny
  rules:
    - name: allow-dns
      protocol: udp
      port: 53
      action: allow
      
    - name: allow-https
      protocol: tcp
      port: 443
      action: allow
      require_mtls: true
      
    - name: block-telemetry
      destination: "*.telemetry.microsoft.com"
      action: deny
```

### Process-level Isolation

Each process gets its own network namespace with:

*   Dedicated virtual interface
*   Egress filtering based on binary hash
*   Ingress filtering based on SELinux label

## Security Features

| Feature | Description |
|---------|-------------|
| mTLS everywhere | All connections require mutual TLS authentication |
| Process identity | Certificate bound to process binary hash (TPM2) |
| Post-quantum TLS | TLS 1.3 + Kyber-768 hybrid key exchange |
| DNS-over-HTTPS | All DNS encrypted via DoH to trusted resolvers |
| QUIC support | HTTP/3 with built-in 0-RTT replay protection |
| Traffic shaping | Per-process bandwidth limits and QoS |
| Deep packet inspection | eBPF-based L7 protocol analysis |
| Anomaly detection | ML-based baseline deviation alerting |

## Network Router Integration

The `feat/zero-trust-network-router` module (merged) adds:

*   **Software-defined router** with policy enforcement at routing level
*   **BGP peering** with route filtering and RPKI validation
*   **SD-WAN** capability for multi-path redundancy
*   **Wireguard mesh** for internal cluster communication

## Monitoring & Observability

```bash
# View active connections with trust scores
sigma-net connections

# Check policy violations
sigma-net audit --last 1h

# Real-time network graph
sigma-net graph --live

# Block a process from network access
sigma-net isolate <pid>
```

## Future: Homomorphic Network Inspection

Research roadmap includes performing network packet analysis on encrypted traffic using homomorphic encryption techniques, allowing threat detection without decrypting traffic contents.
