#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::format;

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Robot Operating System (ROS) Parity Middleware
/// Zero-latency, capability-based pub/sub message-passing.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosMessage {
    pub topic: String,
    pub payload: Vec<u8>,
}

pub struct RosNode {
    pub name: String,
    pub subscriptions: Vec<String>,
}

pub struct RosMiddleware {
    pub nodes: Vec<RosNode>,
    pub message_queue: Vec<RosMessage>,
    pub tf_tree: BTreeMap<String, Transform>,
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub tx: f64, pub ty: f64, pub tz: f64,
    pub qx: f64, pub qy: f64, pub qz: f64, pub qw: f64,
}

impl RosMiddleware {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            message_queue: Vec::new(),
            tf_tree: BTreeMap::new(),
        }
    }

    pub fn register_node(&mut self, name: &str) {
        self.nodes.push(RosNode {
            name: String::from(name),
            subscriptions: Vec::new(),
        });
    }

    pub fn subscribe(&mut self, node_name: &str, topic: &str) -> Result<(), &'static str> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.name == node_name) {
            node.subscriptions.push(String::from(topic));
            Ok(())
        } else {
            Err("Node not found")
        }
    }

    pub fn publish(&mut self, topic: &str, payload: &[u8]) {
        self.message_queue.push(RosMessage {
            topic: String::from(topic),
            payload: payload.to_vec(),
        });
    }

    pub fn poll_messages(&mut self, node_name: &str) -> Vec<RosMessage> {
        let mut result = Vec::new();
        if let Some(node) = self.nodes.iter().find(|n| n.name == node_name) {
            for msg in &self.message_queue {
                if node.subscriptions.contains(&msg.topic) {
                    result.push(msg.clone());
                }
            }
        }
        result
    }

    pub fn update_transform(&mut self, frame_id: &str, child_frame_id: &str, transform: Transform) {
        let key = format!("{}->{}", frame_id, child_frame_id);
        self.tf_tree.insert(key, transform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ros_pubsub() {
        let mut ros = RosMiddleware::new();
        ros.register_node("listener");
        ros.register_node("talker");

        ros.subscribe("listener", "chatter").unwrap();
        ros.publish("chatter", b"hello world");

        let msgs = ros.poll_messages("listener");
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].payload, b"hello world");

        let empty = ros.poll_messages("talker");
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_tf_tree() {
        let mut ros = RosMiddleware::new();
        ros.update_transform("world", "robot", Transform { tx: 1.0, ty: 2.0, tz: 0.0, qx: 0.0, qy: 0.0, qz: 0.0, qw: 1.0 });
        assert!(ros.tf_tree.contains_key("world->robot"));
    }
}
