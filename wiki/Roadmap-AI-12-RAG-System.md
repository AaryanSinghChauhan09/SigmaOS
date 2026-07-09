# SigmaOS Roadmap: Retrieval-Augmented Generation (RAG)
Ground LLM responses in local documents (man pages, wikis, code).
## Goals
- Vector store of chunked OS documentation in sigma_db
- Top-K retrieval injected into LLM context window
## Key Milestones
- [ ] Text chunking and embedding pipeline
- [ ] HNSW approximate nearest-neighbour index
- [ ] RAG prompt assembly and LLM call