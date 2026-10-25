// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - LangChain-style LLM pipeline with RAG and memory
// Principles: MODULARITY, COMPOSITION OVER INHERITANCE, INVERSION OF CONTROL
use alloc::{string::String, vec::Vec, collections::BTreeMap};
#[derive(Debug,Clone)] pub enum Memory { Buffer { max_tokens: u32 }, Summary { model_id: String }, VectorStore { collection: String } }
#[derive(Debug,Clone)] pub struct Document { pub id: String, pub content: String, pub metadata: BTreeMap<String,String>, pub embedding: Vec<f32> }
impl Document {
    pub fn new(id: &str, content: &str) -> Self { Self { id: id.into(), content: content.into(), metadata: BTreeMap::new(), embedding: Vec::new() } }
    pub fn with_metadata(mut self, k: &str, v: &str) -> Self { self.metadata.insert(k.into(), v.into()); self }
}
#[derive(Debug,Clone)] pub struct EmbeddingModel { pub model_id: String, pub dims: u32 }
impl EmbeddingModel {
    pub fn new(model_id: &str, dims: u32) -> Self { Self { model_id: model_id.into(), dims } }
    pub fn embed(&self, text: &str) -> Vec<f32> {
        // Deterministic stub: hash chars into floats
        let mut v = alloc::vec![0.0f32; self.dims as usize];
        for (i,c) in text.bytes().enumerate() { v[i % self.dims as usize] += c as f32 / 256.0; }
        let norm: f32 = v.iter().map(|x| x*x).sum::<f32>().sqrt().max(1e-8);
        v.iter_mut().for_each(|x| *x /= norm);
        v
    }
}
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() { return 0.0; }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x,y)| x*y).sum();
    let na: f32 = a.iter().map(|x| x*x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x*x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 { 0.0 } else { dot / (na * nb) }
}
pub struct VectorIndex { pub documents: Vec<Document>, pub embedding_model: EmbeddingModel }
impl VectorIndex {
    pub fn new(model: EmbeddingModel) -> Self { Self { documents: Vec::new(), embedding_model: model } }
    pub fn add_document(&mut self, mut doc: Document) { if doc.embedding.is_empty() { doc.embedding = self.embedding_model.embed(&doc.content); } self.documents.push(doc); }
    pub fn similarity_search(&self, query: &str, top_k: usize) -> Vec<(&Document, f32)> {
        let q_emb = self.embedding_model.embed(query);
        let mut scored: Vec<(&Document, f32)> = self.documents.iter().map(|d| (d, cosine_similarity(&q_emb, &d.embedding))).collect();
        scored.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }
    pub fn len(&self) -> usize { self.documents.len() }
}
#[derive(Debug,Clone)] pub struct ToolDefinition { pub name: String, pub description: String, pub schema: String }
#[derive(Debug,Clone)] pub enum LangChainTool { WebSearch { query: String }, FileRead { path: String }, FileWrite { path: String, content: String }, CodeExec { code: String, lang: String }, DbQuery { sql: String } }
impl LangChainTool { pub fn execute_stub(&self) -> String { match self { Self::WebSearch { query } => alloc::format!("search results for: {}", query), Self::FileRead { path } => alloc::format!("contents of {}", path), Self::FileWrite { path, .. } => alloc::format!("wrote to {}", path), Self::CodeExec { .. } => String::from("execution output"), Self::DbQuery { sql } => alloc::format!("results of: {}", sql) } } }
pub struct RagPipeline { pub index: VectorIndex }
impl RagPipeline {
    pub fn new(model: EmbeddingModel) -> Self { Self { index: VectorIndex::new(model) } }
    pub fn index_document(&mut self, content: &str, metadata: BTreeMap<String,String>) { let mut doc = Document::new(&alloc::format!("doc_{}", self.index.len()), content); doc.metadata = metadata; self.index.add_document(doc); }
    pub fn retrieve(&self, query: &str, top_k: usize) -> Vec<(&Document, f32)> { self.index.similarity_search(query, top_k) }
    pub fn augment_prompt(&self, query: &str, retrieved: &[(&Document, f32)]) -> String { let ctx = retrieved.iter().map(|(d,_)| d.content.as_str()).collect::<Vec<_>>().join("
---
"); alloc::format!("Context:
{}

Query: {}", ctx, query) }
}
pub struct ConversationChain { pub memory: Memory, pub history: Vec<(String,String)>, pub tools: Vec<ToolDefinition>, pub max_history: usize }
impl ConversationChain {
    pub fn new(memory: Memory) -> Self { Self { memory, history: Vec::new(), tools: Vec::new(), max_history: 20 } }
    pub fn add_turn(&mut self, user: &str, assistant: &str) { if self.history.len() >= self.max_history { self.history.remove(0); } self.history.push((user.into(), assistant.into())); }
    pub fn get_context_window(&self, max_chars: u32) -> String { let mut out = String::new(); let max = max_chars as usize; for (u,a) in self.history.iter().rev() { let turn = alloc::format!("User: {}
Assistant: {}
", u, a); if out.len() + turn.len() > max { break; } out = turn + &out; } out }
    pub fn register_tool(&mut self, tool: ToolDefinition) { self.tools.push(tool); }
    pub fn invoke_tool(&self, tool: &LangChainTool) -> String { tool.execute_stub() }
    pub fn turn_count(&self) -> usize { self.history.len() }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_embed_deterministic() { let m = EmbeddingModel::new("test", 8); let e1 = m.embed("hello"); let e2 = m.embed("hello"); assert_eq!(e1, e2); }
    #[test] fn test_vector_index_retrieval() { let model = EmbeddingModel::new("mini", 16); let mut idx = VectorIndex::new(model); idx.add_document(Document::new("d1", "Rust programming language")); idx.add_document(Document::new("d2", "Python scripting")); idx.add_document(Document::new("d3", "Rust memory safety")); let results = idx.similarity_search("Rust safety features", 2); assert_eq!(results.len(), 2); }
    #[test] fn test_rag_pipeline() { let model = EmbeddingModel::new("mini", 16); let mut rag = RagPipeline::new(model); rag.index_document("SigmaOS is a modular operating system", BTreeMap::new()); rag.index_document("DuckDB is an in-process OLAP database", BTreeMap::new()); let results = rag.retrieve("operating system", 1); assert_eq!(results.len(), 1); let prompt = rag.augment_prompt("what is SigmaOS?", &results); assert!(prompt.contains("Query:")); }
    #[test] fn test_conversation_chain() { let mut chain = ConversationChain::new(Memory::Buffer { max_tokens: 2048 }); chain.add_turn("hello", "hi there"); chain.add_turn("what is 2+2", "4"); assert_eq!(chain.turn_count(), 2); let ctx = chain.get_context_window(1000); assert!(ctx.contains("hello")); }
    #[test] fn test_tool_invocation() { let chain = ConversationChain::new(Memory::Buffer { max_tokens: 1024 }); let result = chain.invoke_tool(&LangChainTool::WebSearch { query: String::from("rust async") }); assert!(result.contains("rust async")); }
}
