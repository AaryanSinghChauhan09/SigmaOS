# AI Agent Guidelines: Constrained Device Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Constrained Device Management**, low-power IoT/embedded node operation, CoAP (Constrained Application Protocol) messaging, MQTT pub/sub event routing, edge mesh orchestration, and lightweight embedded OS profile tuning in SigmaOS.

SigmaOS provides zero-dependency `#![no_std]` protocols and low-memory operating profiles (`EmbeddedIot`, `SigmaNano`) designed to run efficiently on resource-constrained microcontrollers, RISC-V IoT nodes, and edge gateways.

---

## 1. Constrained Device Architectural Subsystems

AI agents interacting with constrained devices in SigmaOS must interface with the following core subsystems:

| Subsystem / Engine | Location | Description |
| :--- | :--- | :--- |
| **CoAP Messaging Engine (`SimpleCoAPClient`, `SimpleCoAPServer`)** | `src/iot/coap.rs` | UDP-based RFC 7252 CoAP request/response client/server for low-memory sensor nodes. |
| **MQTT Pub/Sub Client (`MqttClient`)** | `src/iot/mqtt.rs` | Lightweight MQTT protocol implementation supporting QoS 0/1/2 messaging and retain flags. |
| **IoT Device Mesh Orchestrator (`IotDeviceMeshOrchestrator`)** | `src/distro/transformation_engine.rs` | Edge mesh registration and real-time telemetry synchronization engine. |
| **Embedded Operating Profile (`EmbeddedIot`)** | `src/distro/nextgen_innovations.rs` | Memory-frugal system profile disabling non-essential services, minimizing heap size, and enabling aggressive power gating. |

---

## 2. Resource Constraints & Operating Profiles

When deploying or configuring SigmaOS on constrained devices, AI agents must enforce strict resource boundaries:

```
+-------------------------------------------------------------------+
| Embedded IoT Profile Constraints (SigmaNano / EmbeddedIot)        |
+-------------------------------------------------------------------+
| Memory Limit (RAM):     < 16 MB (or sub-1MB for bare-metal nodes) |
| Heap Pre-Allocation:    Fixed-size static ring buffers              |
| Protocol Transport:     UDP CoAP / Lightweight MQTT over TLS       |
| Power Management:       Aggressive CPU underclock & clock gating    |
+-------------------------------------------------------------------+
```

---

## 3. Protocol Implementation Protocols

### 1. CoAP Resource Management
- **UDP Efficiency:** Use CoAP for low-overhead RESTful requests (`GET`, `POST`, `PUT`, `DELETE`) over UDP port 5683.
- **Resource Registration:** Register observable resources (`SimpleCoAPResource`) for asynchronous sensor telemetry streaming.

```rust
// Registering a CoAP observable sensor resource in SigmaOS
let resource = SimpleCoAPResource::new(1, b"sensors/temp", true);
server.add_resource(Box::new(resource))?;
```

### 2. MQTT Telemetry Publishing
- Maintain lightweight topic hierarchies (`devices/{dev_id}/telemetry`).
- Keep message payloads compact (e.g., JSON or binary CBOR) to prevent buffer overflows on constrained network buffers.

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes to IoT, CoAP, or constrained device modules:

- [ ] Does code maintain `#![no_std]` compatibility without depending on dynamic heap allocations where static buffers suffice?
- [ ] Are CoAP and MQTT packet parsers protected against malformed or truncated byte inputs?
- [ ] Is power gating and clock underclocking respected under the `EmbeddedIot` profile?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
