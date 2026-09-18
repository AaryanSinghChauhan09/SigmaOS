# AI Agent Constrained Application Protocol (CoAP) Architecture (`docs/AGENTS_COAP_MANAGEMENT.md`)

This guide details the technical architecture, IoT resource interfaces, and AI agent monitoring protocols for Constrained Application Protocol (CoAP) management in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS provides a lightweight CoAP client and server implementation for embedded and IoT workloads:

### A. CoAP Protocol & Resource Endpoint Abstractions
- Located in `src/iot/coap.rs`.
- Defines `CoAPResource`, `SimpleCoAPResource`, `CoAPMethod` (`GET`, `POST`, `PUT`, `DELETE`), and `CoAPError`.
- Provides lightweight RESTful URI routing and payload processing over UDP for resource-constrained devices.

### B. Client & Server Communication
- `CoAPClient` / `SimpleCoAPClient` sends request messages (`send_request`) and manages resource observation (`observe`).
- `CoAPServer` / `SimpleCoAPServer` manages dynamic resource registration (`add_resource`, `remove_resource`) and dispatches incoming request payloads to registered endpoints.

---

## 2. AI Agent Operational Directives

1. **Payload Bounds Checks:** Ensure incoming CoAP request payloads and URI paths enforce strict bounds checking to prevent buffer overruns.
2. **Graceful Error Handling:** Confirm unhandled resource paths return `CoAPError::NotFound` without crashing the IoT service daemon.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm IoT and CoAP unit tests pass.
