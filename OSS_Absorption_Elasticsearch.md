# OSS Absorption: Elasticsearch — Distributed Search Engine

> **Status**: 📋 Planned | **Source Project**: Elasticsearch / OpenSearch | **Target Shard**: `SigmaOS Semantic Search Lattice`

---

## 1. Executive Summary

Elasticsearch (and its open-source fork, OpenSearch) is a distributed, RESTful search and analytics engine built on Apache Lucene. It is the industry standard for full-text search, log analytics, and observability data querying, known for its horizontally scalable sharding architecture.

SigmaOS absorbs Elasticsearch's **inverted index search model**, **JSON document indexing**, and **distributed sharding** into the `SigmaOS Semantic Search Lattice`, providing system-wide full-text and vector search.

---

## 2. Key Features to Absorb

### 2.1 System-Wide Inverted Index

Every file created in the `SigmaFS` user directories is asynchronously parsed and added to a system-wide inverted index. This enables instantaneous full-text search across documents, emails, and code.

```bash
$ sigma search "cap-based security" --type document
Σ [SEARCH] Found 3 results (4ms):
  1. /home/user/docs/architecture.md (score: 2.4)
  2. /home/user/downloads/paper.pdf (score: 1.8)
  3. /etc/sigma/policies.toml (score: 0.9)
```

### 2.2 Distributed Sharding & Replicas

For multi-node SigmaOS clusters, the search index is automatically divided into shards and replicated across nodes. If a node fails, the cluster automatically promotes replica shards to primary status, ensuring zero downtime.

```bash
$ sigma search cluster health
Σ [SEARCH] Cluster Health: GREEN
  Nodes:   3
  Indices: 14
  Shards:  42 primary, 42 replica
```

### 2.3 Hybrid Vector Search (BM25 + kNN)

SigmaOS extends the traditional BM25 full-text scoring with k-Nearest Neighbor (kNN) vector search (powered by local LLM embeddings), allowing users to search by *concept* rather than exact keywords.

---

## 3. References & Standards

- OpenSearch — `opensearch.org` (Apache-2.0)
- Apache Lucene — `lucene.apache.org`
