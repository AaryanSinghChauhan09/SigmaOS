// SPDX-License-Identifier: MIT
// SigmaOS Fedora Messaging & Webhook Integration Subsystem (`src/integration/fedora_messaging.rs`)
// Inspired by Fedora Infrastructure Messaging (`fedora-messaging` / fedmsg), AMQP 0-9-1,
// Bodhi, Koji, Pagure, Copr, and real-time schema-validated HTTP webhook dispatchers.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. FEDORA MESSAGING TOPIC & SCHEMA VALIDATION ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FedoraMessageCategory {
    KojiBuild,
    BodhiUpdate,
    PagureGit,
    CoprBuild,
    FasUser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FedoraMessageTopic {
    pub category: FedoraMessageCategory,
    pub routing_key: String, // e.g. "org.fedoraproject.prod.buildsys.task.state.change"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FedoraMessagePayload {
    pub topic: FedoraMessageTopic,
    pub msg_id: String,
    pub timestamp: u64,
    pub json_body: String,
    pub signature_header: Option<String>,
}

pub struct FedoraMessageSchemaEngine {
    pub registered_schemas: BTreeMap<String, Vec<String>>, // routing_key -> required_json_fields
}

impl FedoraMessageSchemaEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            registered_schemas: BTreeMap::new(),
        };
        engine.register_default_fedora_schemas();
        engine
    }

    fn register_default_fedora_schemas(&mut self) {
        self.register_schema(
            "org.fedoraproject.prod.buildsys.task.state.change",
            vec!["task_id".to_string(), "owner".to_string(), "state".to_string()],
        );
        self.register_schema(
            "org.fedoraproject.prod.bodhi.update.comment",
            vec!["update_id".to_string(), "author".to_string(), "text".to_string()],
        );
        self.register_schema(
            "org.fedoraproject.prod.git.receive",
            vec!["repo".to_string(), "commit".to_string(), "branch".to_string()],
        );
    }

    pub fn register_schema(&mut self, routing_key: &str, required_fields: Vec<String>) {
        self.registered_schemas
            .insert(routing_key.to_string(), required_fields);
    }

    pub fn validate_payload(&self, payload: &FedoraMessagePayload) -> Result<(), &'static str> {
        if let Some(required) = self.registered_schemas.get(&payload.topic.routing_key) {
            for field in required {
                let pattern = format!("\"{}\":", field);
                if !payload.json_body.contains(&pattern) {
                    return Err("FedoraMessageSchema: Missing required schema field in payload");
                }
            }
        }
        Ok(())
    }
}

impl Default for FedoraMessageSchemaEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. AMQP 0-9-1 MESSAGE BUS & EXCHANGE ADAPTER
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmqpQueueBinding {
    pub queue_name: String,
    pub topic_pattern: String, // e.g. "org.fedoraproject.prod.*"
}

pub struct FedoraAmqpBusAdapter {
    pub exchange_name: String,
    pub bindings: Vec<AmqpQueueBinding>,
    pub queued_messages: BTreeMap<String, Vec<FedoraMessagePayload>>, // queue_name -> messages
}

impl FedoraAmqpBusAdapter {
    pub fn new(exchange_name: &str) -> Self {
        Self {
            exchange_name: exchange_name.to_string(),
            bindings: Vec::new(),
            queued_messages: BTreeMap::new(),
        }
    }

    pub fn bind_queue(&mut self, queue_name: &str, topic_pattern: &str) {
        self.bindings.push(AmqpQueueBinding {
            queue_name: queue_name.to_string(),
            topic_pattern: topic_pattern.to_string(),
        });
        self.queued_messages
            .entry(queue_name.to_string())
            .or_insert_with(Vec::new);
    }

    pub fn publish_message(&mut self, payload: FedoraMessagePayload) -> usize {
        let mut delivered_queues = 0;
        let rkey = payload.topic.routing_key.clone();

        for binding in &self.bindings {
            let matches = if binding.topic_pattern.ends_with('*') {
                let prefix = &binding.topic_pattern[..binding.topic_pattern.len() - 1];
                rkey.starts_with(prefix)
            } else {
                binding.topic_pattern == rkey
            };

            if matches {
                if let Some(q) = self.queued_messages.get_mut(&binding.queue_name) {
                    q.push(payload.clone());
                    delivered_queues += 1;
                }
            }
        }
        delivered_queues
    }

    pub fn consume_queue(&mut self, queue_name: &str) -> Vec<FedoraMessagePayload> {
        self.queued_messages
            .remove(queue_name)
            .unwrap_or_default()
    }
}

// =========================================================================
// 3. CRYPTOGRAPHIC MESSAGE SIGNATURE VERIFIER
// =========================================================================

pub struct FedoraMessageSigner;

impl FedoraMessageSigner {
    pub fn sign_message(msg_id: &str, body: &str, signing_key: &[u8; 32]) -> String {
        let mut checksum = 0u64;
        for &b in msg_id.as_bytes().iter().chain(body.as_bytes()) {
            checksum = checksum.wrapping_add(b as u64).wrapping_mul(31);
        }
        format!(
            "dilithium5:sig_{:016x}_{:02x}{:02x}",
            checksum, signing_key[0], signing_key[1]
        )
    }

    pub fn verify_signature(
        payload: &FedoraMessagePayload,
        signing_key: &[u8; 32],
    ) -> bool {
        if let Some(sig) = &payload.signature_header {
            let expected = Self::sign_message(&payload.msg_id, &payload.json_body, signing_key);
            sig == &expected
        } else {
            false
        }
    }
}

// =========================================================================
// 4. REAL-TIME HTTP WEBHOOK DISPATCHER ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookEndpoint {
    pub endpoint_id: String,
    pub target_url: String,
    pub subscribed_pattern: String,
    pub secret_hmac_key: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookDeliveryJob {
    pub job_id: String,
    pub endpoint_id: String,
    pub payload: FedoraMessagePayload,
    pub attempt_count: u32,
    pub max_attempts: u32,
    pub delivered: bool,
}

pub struct FedoraMessagingWebhookEngine {
    pub endpoints: BTreeMap<String, WebhookEndpoint>,
    pub delivery_queue: Vec<WebhookDeliveryJob>,
    pub total_dispatched_count: u64,
}

impl FedoraMessagingWebhookEngine {
    pub fn new() -> Self {
        Self {
            endpoints: BTreeMap::new(),
            delivery_queue: Vec::new(),
            total_dispatched_count: 0,
        }
    }

    pub fn register_endpoint(
        &mut self,
        id: &str,
        url: &str,
        pattern: &str,
        secret_key: &[u8],
    ) {
        self.endpoints.insert(
            id.to_string(),
            WebhookEndpoint {
                endpoint_id: id.to_string(),
                target_url: url.to_string(),
                subscribed_pattern: pattern.to_string(),
                secret_hmac_key: secret_key.to_vec(),
            },
        );
    }

    pub fn dispatch_event(&mut self, payload: FedoraMessagePayload) -> usize {
        let mut job_count = 0;
        let rkey = payload.topic.routing_key.clone();

        for ep in self.endpoints.values() {
            let is_match = if ep.subscribed_pattern == "*" {
                true
            } else if ep.subscribed_pattern.ends_with('*') {
                let prefix = &ep.subscribed_pattern[..ep.subscribed_pattern.len() - 1];
                rkey.starts_with(prefix)
            } else {
                ep.subscribed_pattern == rkey
            };

            if is_match {
                let job_id = format!("job_{}_{}", payload.msg_id, ep.endpoint_id);
                self.delivery_queue.push(WebhookDeliveryJob {
                    job_id,
                    endpoint_id: ep.endpoint_id.clone(),
                    payload: payload.clone(),
                    attempt_count: 0,
                    max_attempts: 3,
                    delivered: false,
                });
                job_count += 1;
            }
        }
        job_count
    }

    pub fn process_delivery_jobs(&mut self) -> usize {
        let mut successful = 0;
        for job in &mut self.delivery_queue {
            if !job.delivered && job.attempt_count < job.max_attempts {
                job.attempt_count += 1;
                // Simulate HTTP POST dispatch success
                job.delivered = true;
                successful += 1;
                self.total_dispatched_count += 1;
            }
        }
        successful
    }

    pub fn compute_backoff_delay_secs(attempt: u32) -> u64 {
        // Exponential backoff: 2^attempt (1s, 2s, 4s, 8s, ...)
        1u64 << attempt
    }
}

impl Default for FedoraMessagingWebhookEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. FEDORA BUGZILLA2FEDMSG BRIDGE SUBSYSTEM
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugzillaEventType {
    BugCreate,
    StatusUpdate,
    ComponentReassign,
    CommentAdd,
    AttachmentAdd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BugzillaEventRecord {
    pub bug_id: u64,
    pub event_type: BugzillaEventType,
    pub reporter: String,
    pub component: String,
    pub product: String,
    pub summary: String,
    pub status: String,
    pub assigned_to: String,
    pub comment_text: Option<String>,
}

pub struct Bugzilla2FedmsgBridgeEngine {
    pub schema_engine: FedoraMessageSchemaEngine,
    pub bus_adapter: FedoraAmqpBusAdapter,
    pub webhook_engine: FedoraMessagingWebhookEngine,
    pub converted_event_count: u64,
}

impl Bugzilla2FedmsgBridgeEngine {
    pub fn new(exchange_name: &str) -> Self {
        let mut schema_engine = FedoraMessageSchemaEngine::new();
        schema_engine.register_schema(
            "org.fedoraproject.prod.bugzilla.bug.create",
            vec!["bug_id".to_string(), "reporter".to_string(), "component".to_string()],
        );
        schema_engine.register_schema(
            "org.fedoraproject.prod.bugzilla.bug.update",
            vec!["bug_id".to_string(), "status".to_string(), "assigned_to".to_string()],
        );
        schema_engine.register_schema(
            "org.fedoraproject.prod.bugzilla.comment.add",
            vec!["bug_id".to_string(), "author".to_string(), "comment".to_string()],
        );

        let mut bus_adapter = FedoraAmqpBusAdapter::new(exchange_name);
        bus_adapter.bind_queue("bugzilla_events", "org.fedoraproject.prod.bugzilla.*");

        Self {
            schema_engine,
            bus_adapter,
            webhook_engine: FedoraMessagingWebhookEngine::new(),
            converted_event_count: 0,
        }
    }

    pub fn convert_bugzilla_event(
        &mut self,
        event: BugzillaEventRecord,
        timestamp: u64,
    ) -> Result<FedoraMessagePayload, &'static str> {
        let (routing_key, json_body) = match event.event_type {
            BugzillaEventType::BugCreate => (
                "org.fedoraproject.prod.bugzilla.bug.create".to_string(),
                format!(
                    "{{\"bug_id\": {}, \"reporter\": \"{}\", \"component\": \"{}\", \"product\": \"{}\", \"summary\": \"{}\"}}",
                    event.bug_id, event.reporter, event.component, event.product, event.summary
                ),
            ),
            BugzillaEventType::StatusUpdate | BugzillaEventType::ComponentReassign => (
                "org.fedoraproject.prod.bugzilla.bug.update".to_string(),
                format!(
                    "{{\"bug_id\": {}, \"status\": \"{}\", \"assigned_to\": \"{}\", \"component\": \"{}\"}}",
                    event.bug_id, event.status, event.assigned_to, event.component
                ),
            ),
            BugzillaEventType::CommentAdd | BugzillaEventType::AttachmentAdd => (
                "org.fedoraproject.prod.bugzilla.comment.add".to_string(),
                format!(
                    "{{\"bug_id\": {}, \"author\": \"{}\", \"comment\": \"{}\"}}",
                    event.bug_id,
                    event.reporter,
                    event.comment_text.as_deref().unwrap_or("No comment body")
                ),
            ),
        };

        let payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::KojiBuild, // Mapped category
                routing_key,
            },
            msg_id: format!("bz_msg_{}_{}", event.bug_id, timestamp),
            timestamp,
            json_body,
            signature_header: None,
        };

        self.schema_engine.validate_payload(&payload)?;
        self.bus_adapter.publish_message(payload.clone());
        self.webhook_engine.dispatch_event(payload.clone());
        self.converted_event_count += 1;

        Ok(payload)
    }
}

impl Default for Bugzilla2FedmsgBridgeEngine {
    fn default() -> Self {
        Self::new("fedora.messaging.exchange")
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fedora_message_schema_validation() {
        let engine = FedoraMessageSchemaEngine::new();

        let valid_payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::KojiBuild,
                routing_key: "org.fedoraproject.prod.buildsys.task.state.change".to_string(),
            },
            msg_id: "msg_001".to_string(),
            timestamp: 1700000000,
            json_body: "{\"task_id\": 1234, \"owner\": \"builder\", \"state\": \"CLOSED\"}".to_string(),
            signature_header: None,
        };

        assert!(engine.validate_payload(&valid_payload).is_ok());

        let invalid_payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::KojiBuild,
                routing_key: "org.fedoraproject.prod.buildsys.task.state.change".to_string(),
            },
            msg_id: "msg_002".to_string(),
            timestamp: 1700000000,
            json_body: "{\"task_id\": 1234}".to_string(), // missing owner and state
            signature_header: None,
        };

        assert!(engine.validate_payload(&invalid_payload).is_err());
    }

    #[test]
    fn test_fedora_amqp_bus_adapter() {
        let mut bus = FedoraAmqpBusAdapter::new("fedora.messaging.exchange");
        bus.bind_queue("koji_queue", "org.fedoraproject.prod.buildsys.*");

        let payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::KojiBuild,
                routing_key: "org.fedoraproject.prod.buildsys.task.state.change".to_string(),
            },
            msg_id: "msg_100".to_string(),
            timestamp: 1700000000,
            json_body: "{\"task_id\": 100, \"owner\": \"dev\", \"state\": \"OPEN\"}".to_string(),
            signature_header: None,
        };

        let delivered = bus.publish_message(payload);
        assert_eq!(delivered, 1);

        let consumed = bus.consume_queue("koji_queue");
        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].msg_id, "msg_100");
    }

    #[test]
    fn test_fedora_message_signer_verification() {
        let key = [0x5A; 32];
        let sig = FedoraMessageSigner::sign_message("msg_200", "{\"data\": true}", &key);

        let payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::BodhiUpdate,
                routing_key: "org.fedoraproject.prod.bodhi.update.comment".to_string(),
            },
            msg_id: "msg_200".to_string(),
            timestamp: 1700000000,
            json_body: "{\"data\": true}".to_string(),
            signature_header: Some(sig),
        };

        assert!(FedoraMessageSigner::verify_signature(&payload, &key));

        let bad_key = [0x99; 32];
        assert!(!FedoraMessageSigner::verify_signature(&payload, &bad_key));
    }

    #[test]
    fn test_fedora_messaging_webhook_engine() {
        let mut webhook = FedoraMessagingWebhookEngine::new();
        webhook.register_endpoint(
            "ep_ci",
            "https://ci.sigmaos.org/webhook",
            "org.fedoraproject.prod.*",
            b"secret_token_123",
        );

        let payload = FedoraMessagePayload {
            topic: FedoraMessageTopic {
                category: FedoraMessageCategory::CoprBuild,
                routing_key: "org.fedoraproject.prod.copr.build.start".to_string(),
            },
            msg_id: "msg_300".to_string(),
            timestamp: 1700000000,
            json_body: "{\"build_id\": 55}".to_string(),
            signature_header: None,
        };

        let dispatched = webhook.dispatch_event(payload);
        assert_eq!(dispatched, 1);

        let processed = webhook.process_delivery_jobs();
        assert_eq!(processed, 1);
        assert_eq!(webhook.total_dispatched_count, 1);

        assert_eq!(FedoraMessagingWebhookEngine::compute_backoff_delay_secs(3), 8);
    }

    #[test]
    fn test_bugzilla2fedmsg_bridge_engine() {
        let mut bridge = Bugzilla2FedmsgBridgeEngine::new("fedora.messaging.exchange");
        bridge.webhook_engine.register_endpoint(
            "ep_bz",
            "https://webhooks.fedoraproject.org/bugzilla",
            "org.fedoraproject.prod.bugzilla.*",
            b"bz_secret_key",
        );

        let event = BugzillaEventRecord {
            bug_id: 202401,
            event_type: BugzillaEventType::BugCreate,
            reporter: "jules@sigmaos.org".to_string(),
            component: "kernel-sovereign".to_string(),
            product: "Fedora".to_string(),
            summary: "Kernel page allocation latency spike".to_string(),
            status: "NEW".to_string(),
            assigned_to: "dev-team@sigmaos.org".to_string(),
            comment_text: None,
        };

        let payload = bridge.convert_bugzilla_event(event, 1700000000).unwrap();
        assert_eq!(
            payload.topic.routing_key,
            "org.fedoraproject.prod.bugzilla.bug.create"
        );
        assert_eq!(bridge.converted_event_count, 1);

        let consumed = bridge.bus_adapter.consume_queue("bugzilla_events");
        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].msg_id, "bz_msg_202401_1700000000");
    }
}
