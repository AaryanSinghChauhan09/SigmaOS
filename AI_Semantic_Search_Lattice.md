# Semantic Search Lattice: Local Vector-Based File Search Specification

> **Status**: 🔄 Active | **Component**: `SigmaSemanticSearch` / `MiniLM-L6` | **Phase**: Phase 3 — Advanced AI Capabilities

---

## 1. Executive Summary

Traditional file systems search by exact filename or content keyword. The **Semantic Search Lattice** understands the *meaning* of a query, allowing users to find files by describing what they contain in natural language (e.g., "the tax document from last March" or "my Python script that processes CSV files").

The system uses **MiniLM-L6**, a 22MB embedding model that converts file contents and query strings into 384-dimensional vectors. Files are indexed incrementally in the background by a background daemon (`sigma-indexd`), and semantic similarity is computed using cosine distance against a local flat-file HNSW vector index.

No data is sent to external services — all indexing and retrieval is fully local.

---

## 2. Architecture

### 2.1 System Components

```
┌─────────────────────────────────────────────────────────────────┐
│                   SEMANTIC SEARCH LATTICE                       │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                 sigma-indexd DAEMON                     │    │
│  │  Watches filesystem events (inotify/kqueue)             │    │
│  │  → Extracts text from: .txt, .md, .pdf, .rs, .py ...   │    │
│  │  → Embeds content → 384-dim float vector               │    │
│  │  → Stores in local HNSW index (~5MB/1000 files)        │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              ▲ indexes                          │
│                              │                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    USER QUERY FLOW                      │    │
│  │  "find my tax document from March"                      │    │
│  │       │                                                 │    │
│  │       ▼                                                 │    │
│  │  MiniLM-L6 embed ──▶ 384-dim query vector               │    │
│  │       │                                                 │    │
│  │       ▼                                                 │    │
│  │  HNSW cosine search ──▶ Top-10 nearest files            │    │
│  │       │                                                 │    │
│  │       ▼                                                 │    │
│  │  Re-rank by metadata (date, extension, path)            │    │
│  │       │                                                 │    │
│  │       ▼                                                 │    │
│  │  Return ranked file list with score + preview           │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Rust Implementation

```rust
// userland/system_api/ai_integration/semantic_search.rs
// SPDX-License-Identifier: MIT

pub struct SigmaSemanticSearch {
    embedder: SigmaInference,   // MiniLM-L6 via SigmaGGML
    index:    HnswIndex,        // Local vector index on disk
    text_ext: TextExtractor,    // PDF, DOCX, code file content extraction
}

pub struct SearchResult {
    pub path:     PathBuf,
    pub score:    f32,          // 0.0-1.0 cosine similarity
    pub excerpt:  String,       // Matching text snippet
    pub modified: SystemTime,
}

impl SigmaSemanticSearch {
    /// Full-text semantic query
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query_vec = self.embedder.embed(query);
        let candidates = self.index.search(&query_vec, limit * 5); // over-fetch for re-ranking

        self.rerank_with_metadata(candidates, query, limit)
    }

    /// Index a single file — called by sigma-indexd
    pub fn index_file(&mut self, path: &Path) -> Result<()> {
        let text = self.text_ext.extract(path)?;
        let chunks = chunk_text(&text, 256); // 256-token overlapping windows

        for chunk in chunks {
            let vec = self.embedder.embed(&chunk.text);
            self.index.insert(IndexEntry {
                id:      path_hash(path, chunk.offset),
                vec,
                path:    path.to_owned(),
                excerpt: chunk.text.clone(),
            })?;
        }
        Ok(())
    }

    fn rerank_with_metadata(
        &self,
        mut candidates: Vec<IndexEntry>,
        query: &str,
        limit: usize,
    ) -> Vec<SearchResult> {
        // Boost recent files, penalize archive directories
        candidates.sort_by(|a, b| {
            let score_a = a.similarity + recency_boost(a.modified) - archive_penalty(&a.path);
            let score_b = b.similarity + recency_boost(b.modified) - archive_penalty(&b.path);
            score_b.partial_cmp(&score_a).unwrap()
        });
        candidates.into_iter().take(limit).map(SearchResult::from).collect()
    }
}
```

---

## 3. Indexing Strategy & Supported Formats

| File Type | Extraction Method | Average Tokens |
|:----------|:------------------|:---------------|
| `.txt`, `.md` | Direct UTF-8 | ~500 |
| `.pdf` | pdftotext (sigma-pdftools) | ~800 |
| `.rs`, `.py`, `.go` | AST-aware chunking | ~400 |
| `.docx`, `.odt` | XML unpack | ~600 |
| `.html` | Stripped DOM text | ~300 |
| Image files | OCR (Tesseract shard) | ~200 |

> [!NOTE]
> Indexing runs at `nice +19` (lowest CPU priority) and automatically pauses when battery is below 20% or CPU utilization exceeds 70%.

---

## 4. Usage Examples

```bash
# Semantic file search
$ sigma find "the tax document from last March"
Σ [INFO] Semantic search... (22ms, 3,412 indexed files)

  [0.94] /home/user/Documents/taxes/2024_march_return.pdf  (2024-03-28)
         "2024 Federal Tax Return — Filed March 28..."
  [0.87] /home/user/Downloads/tax_return_draft.docx        (2024-03-15)
         "Draft — 2024 Annual Tax Filing..."
  [0.72] /home/user/finance/receipts/march_2024.xlsx       (2024-03-01)

Open first match? [y/N]

# Developer search — understands code intent
$ sigma find "my Python script that reads CSV and outputs charts"
Σ [INFO] Semantic search...

  [0.91] /home/user/code/data_viz.py  (2025-11-12)
         "import pandas as pd; import matplotlib.pyplot..."
```

---

## 5. Privacy & Storage
- The HNSW index is stored at `/home/user/.cache/sigma/semantic-index/`
- The index contains file vectors and path hashes only — **not file content**
- Users can exclude directories via `~/.config/sigma/search-exclude.toml`
- Index is fully user-owned; deleting it forces a full re-index

---

## 6. References & Standards
- MiniLM-L6-v2 by Microsoft Research (MIT License)
- HNSW: Hierarchical Navigable Small World Graphs — Malkov & Yashunin, 2018
- Semantic search best practices — Hugging Face documentation
