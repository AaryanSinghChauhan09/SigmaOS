# SigmaOS AI Agent Constrained Application Protocol (CoAP) Management Directive (`AGENTS_COAP_MANAGEMENT.md`)

This document defines technical directives, IoT resource endpoint rules, and communication protocols for AI agents managing the Constrained Application Protocol (CoAP) in SigmaOS.

---

## 1. Core Principles for CoAP Management

CoAP (RFC 7252) provides lightweight, low-overhead RESTful messaging over UDP for constrained IoT environments in SigmaOS. AI agents modifying or extending CoAP interfaces must observe the following rules:

1. **Resource Endpoint Handling (`CoAPResource`, `SimpleCoAPResource`):**
   - CoAP resource endpoints must implement the `CoAPResource` trait.
   - Request URI paths and payloads must be validated before processing to prevent path traversal or buffer read overruns in constrained memory devices.

2. **Method Dispatch & Error Mapping (`CoAPMethod`, `CoAPError`):**
   - Standard RESTful request methods (`GET`, `POST`, `PUT`, `DELETE`) must map to `CoAPMethod` enum variants.
   - Resource access or request failures must return explicit `CoAPError` codes (`Success`, `NotFound`, `RequestFailed`) rather than panicking in IoT kernel contexts.

3. **Resource Observation (`observe`):**
   - Subscriptions to observable resource endpoints (`observe`) must track client registration states and dispatch state delta updates efficiently without saturating network bandwidth.

4. **Zero-Dependency `#![no_std]` Compatibility:**
   - CoAP message encoding, option parsing, and resource registries in `src/iot/coap.rs` must maintain zero-dependency `#![no_std]` design compliance.

---

## 2. Pre-Commit CoAP Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] CoAP server resource registration (`add_resource`, `remove_resource`) handles non-existent IDs cleanly.
- [ ] Request URI path and payload inputs are validated against buffer bounds.
- [ ] CoAP client request/observe invocations handle transport errors safely.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
