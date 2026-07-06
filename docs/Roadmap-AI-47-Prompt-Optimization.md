# SigmaOS Roadmap: Automatic Local Prompt Optimization
Run genetic algorithms to automatically optimize prompt templates for local SLMs.
## Goals
- Systematically mutate prompt wording to maximize LLM parse rate and JSON correctness.
- Maintain a local leaderboard of prompt templates.
## Key Milestones
- [ ] Prompt mutations generator
- [ ] Success/fail tracking evaluation harness
- [ ] Auto-update runtime prompts in local_llm.rs
"@

"Roadmap-AI-48-Personal-Knowledge-Graph.md" = @"
# SigmaOS Roadmap: Unified Personal Knowledge Graph (PKG)
Construct a private relational graph of your local files, code projects, and tasks.
## Goals
- Graph database nodes and edges representing files, folders, commits, and meetings.
- Fast subgraph search query engine.
## Key Milestones
- [ ] PKG schema design in sigma_db
- [ ] Automatic file indexing to graph pipeline
- [ ] Natural-language query interface