# SigmaOS Roadmap: LLM Memory Consolidation and Pruning
Consolidate, compress, and prune episodic LLM conversational memory during OS idle cycles.
## Goals
- Auto-extract semantic facts from chat logs, converting to key-value knowledge graphs in sigma_db.
- Vector similarity pruning to maintain constant memory consumption bounds.
## Key Milestones
- [ ] Fact extraction pipeline running on background low-priority queue
- [ ] Graph-based fact deduplication
- [ ] Index compaction in sigma_db
"@

"Roadmap-AI-44-Visual-Debugging.md" = @"
# SigmaOS Roadmap: Visual GUI Debugging Assistant
Interpret desktop UI layout, elements, and draw calls via visual model to detect design bugs.
## Goals
- Screen raster rendering to local ViT visual parser.
- Auto-detect UI overlaps, alignment issues, and color contrast failures locally.
## Key Milestones
- [ ] Screen frame buffer exporter integration
- [ ] Visual QA inference loop
- [ ] Visual linting report overlay