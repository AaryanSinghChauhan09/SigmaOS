# SigmaOS Roadmap: Semantic Application Search
Search installed apps, files, and settings using natural-language queries in the Zenith Launcher.
## Goals
- Vector embedding of app names and descriptions stored in sigma_db
- Cosine similarity ranking without GPU requirement
## Key Milestones
- [ ] MiniLM-L6 embedding model integration
- [ ] Inverted index with TF-IDF fallback
- [ ] Live result ranking in launcher UI