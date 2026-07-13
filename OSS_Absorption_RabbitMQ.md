# OSS Absorption: RabbitMQ — Message Queuing & Brokerage

> **Status**: 📋 Planned | **Source Project**: RabbitMQ | **Target Shard**: `SigmaOS Message Bus`

---

## 1. Executive Summary

RabbitMQ is a widely deployed open-source message broker that supports multiple messaging protocols (AMQP, MQTT, STOMP).

SigmaOS absorbs the **flexible exchange-based routing model (Direct, Fanout, Topic, Headers)** of RabbitMQ, embedding it into the native IPC message router (`sigma-router`) to handle complex service communication topologies.

---

## 2. Key Features Absorbed

### 2.1 Exchange-Based Message Routing

Instead of simple point-to-point sockets, applications can register exchange boundaries where messages are routed to multiple consumer queues based on pattern matching.

```rust
// kernel/ipc/router.rs
// SPDX-License-Identifier: MIT

pub enum ExchangeType {
    Direct,
    Fanout,
    Topic,
}

pub struct MessageExchange {
    pub name: String,
    pub ex_type: ExchangeType,
    pub bindings: Vec<Binding>,
}
```

---

## 3. References & Standards

- RabbitMQ — `rabbitmq.com` (Mozilla Public License 2.0)
- AMQP Specification
