# SigmaOS AI Agent Cloud Carrier Operation Management Specification

This document specifies mandatory carrier-grade cloud network virtualization rules, high-availability CARP/BGP failover standards, OpenStack Cinder block volume provisioning invariants, and zero-downtime cluster orchestration standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. High-Availability IP Redundancy & BGP Peering
- **FreeBSD CARP Failover Engine (`src/network/distro_net.rs`)**:
  - Virtual Router Redundancy Protocol (CARP/VRRP) must negotiate master/backup state transitions with sub-second heartbeat advertisement timers.
  - Failover state transitions must automatically migrate Virtual IP (VIP) addresses across active carrier nodes without TCP session drops.

## 2. OpenStack Cinder Storage & NGINX Ingress Routing
- **Cinder Volume Provisioning (`src/open_source_os_gap_closure.rs`)**:
  - Cloud volume provisioning (`provision_cinder_volume`) must validate volume ID parameters, capacity limits, and enforce AES/PQC volume encryption masks.
- **Sovereign Ingress Routing**:
  - Ingress controllers must enforce PQC-encrypted TLS termination and tenant isolation rules.

## 3. 5G/6G Cellular Core & Telemetry
- **Cellular Network Functions (`src/unimplemented_features.rs`)**:
  - Mobile carrier variant engines (`SigmaOsMobileVariantEngine`) must manage 5G Sub-6GHz and mmWave network slicing interfaces.
- **OpenTelemetry Collection**:
  - Network metrics, ingress rates, and packet drop counts must stream to `SovereignOpenTelemetryMetricsCollector`.

## 4. AI Agent Cloud Carrier Directives
1. **Zero Downtime Failover**: Failover routines must ensure backup nodes assume VIP routing within < 50ms upon master node heartbeat loss.
2. **Multi-Tenant Isolation**: Carrier network slices and Cinder storage volumes must be cryptographically isolated per tenant ID.
