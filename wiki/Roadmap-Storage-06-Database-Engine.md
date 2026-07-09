# SigmaOS Roadmap: Embedded Relational Database (sigma-sql)
A full ACID-compliant relational database engine for system applications.
## Goals
- B-tree index with WAL (Write-Ahead Log)
- MVCC for concurrent read isolation
## Key Milestones
- [ ] B-tree page cache (no_std)
- [ ] WAL log record format
- [ ] MVCC snapshot isolation level