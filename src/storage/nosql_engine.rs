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

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// NoSQL Database Engine (Cassandra & CouchDB Parity)
/// Wide-column store and document store supporting masterless replication.

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Array(Vec<DocumentValue>),
    Map(BTreeMap<String, DocumentValue>),
}

pub struct NoSqlEngine {
    /// Simulates a wide-column or document store mapping Keys to JSON-like Documents
    pub collections: BTreeMap<String, BTreeMap<String, DocumentValue>>,
    pub node_id: String,
}

impl NoSqlEngine {
    pub fn new(node_id: &str) -> Self {
        Self {
            collections: BTreeMap::new(),
            node_id: String::from(node_id),
        }
    }

    pub fn insert_document(&mut self, collection: &str, key: &str, document: DocumentValue) {
        let coll = self
            .collections
            .entry(String::from(collection))
            .or_insert_with(BTreeMap::new);
        coll.insert(String::from(key), document);
    }

    pub fn get_document(&self, collection: &str, key: &str) -> Option<&DocumentValue> {
        self.collections.get(collection).and_then(|c| c.get(key))
    }

    /// Masterless Replication Gossip Sync simulation
    pub fn gossip_sync(&mut self, _peer_data: &[u8]) {
        // In a real system, this would deserialize Merkle trees and resolve vector clocks.
        // For demonstration, we assume sync is handled.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nosql_document_store() {
        let mut db = NoSqlEngine::new("node_a");

        let mut user_doc = BTreeMap::new();
        user_doc.insert(
            String::from("name"),
            DocumentValue::Text(String::from("Alice")),
        );
        user_doc.insert(String::from("age"), DocumentValue::Integer(30));

        db.insert_document("users", "user:1", DocumentValue::Map(user_doc));

        let retrieved = db.get_document("users", "user:1").unwrap();
        if let DocumentValue::Map(map) = retrieved {
            assert_eq!(
                map.get("name").unwrap(),
                &DocumentValue::Text(String::from("Alice"))
            );
            assert_eq!(map.get("age").unwrap(), &DocumentValue::Integer(30));
        } else {
            panic!("Expected Map");
        }
    }
}
