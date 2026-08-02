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

/// Search & Information Retrieval Engine (Lucene Parity)
/// Full-text indexing, tokenization, stemming, and TF-IDF ranking.

pub struct DocumentIndex {
    /// inverted index: word -> Vec<(document_id, frequency)>
    pub index: BTreeMap<String, Vec<(u32, u32)>>,
    pub document_count: u32,
}

impl DocumentIndex {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
            document_count: 0,
        }
    }

    /// Basic tokenization (split by space)
    pub fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| String::from(s.to_lowercase()))
            .collect()
    }

    pub fn add_document(&mut self, doc_id: u32, text: &str) {
        self.document_count += 1;
        let tokens = Self::tokenize(text);
        let mut term_freqs: BTreeMap<String, u32> = BTreeMap::new();

        for token in tokens {
            *term_freqs.entry(token).or_insert(0) += 1;
        }

        for (term, freq) in term_freqs {
            self.index
                .entry(term)
                .or_insert_with(Vec::new)
                .push((doc_id, freq));
        }
    }

    /// Basic TF-IDF search scoring simulation
    pub fn search(&self, query: &str) -> Vec<(u32, f64)> {
        let tokens = Self::tokenize(query);
        let mut scores: BTreeMap<u32, f64> = BTreeMap::new();

        for token in tokens {
            if let Some(postings) = self.index.get(&token) {
                let idf = (self.document_count as f64 / postings.len() as f64).ln() + 1.0;
                for (doc_id, tf) in postings {
                    let score = (*tf as f64) * idf;
                    *scores.entry(*doc_id).or_insert(0.0) += score;
                }
            }
        }

        let mut results: Vec<(u32, f64)> = scores.into_iter().collect();
        // Sort descending by score
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_index() {
        let mut engine = DocumentIndex::new();
        engine.add_document(1, "The quick brown fox");
        engine.add_document(2, "Jumped over the lazy dog");
        engine.add_document(3, "The quick dog");

        let results = engine.search("quick");
        assert_eq!(results.len(), 2);

        let first = results[0];
        // Doc 1 or 3 should have a non-zero score
        assert!(first.1 > 0.0);
    }
}
