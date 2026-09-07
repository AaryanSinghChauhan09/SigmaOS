# AI Agent Development Instructions for Database & Storage Engine Operations (`src/storage/` & `src/filesystem/sqlite.rs`)

This document provides directives for embedded relational SQL query engines, SQLite B-tree page storage, VDBE virtual machine execution, NoSQL document stores, and ACID transaction journal safety in SigmaOS.

## Subsystem Architecture & Directives

1. **Embedded SQLite & VDBE Execution (`src/filesystem/sqlite.rs` & `vdbe_doom.rs`)**
   - Implements native B-tree page storage (`1024` to `65536` byte page sizes) and SQL parsing/compilation into Virtual Database Engine (VDBE) opcodes.
   - SQL queries MUST use parameterized statement binding (`?1`, `:param`) to eliminate SQL injection vulnerabilities.

2. **Relational & NoSQL Engine Abstractions (`src/storage/sql_engine.rs` & `nosql_engine.rs`)**
   - Support ACID transaction isolation (`ReadCommitted`, `Serializable`).
   - Write-Ahead Logging (WAL) journals must perform explicit `fsync` flushes prior to committing transaction page headers to guarantee durability across sudden power loss events.

3. **In-Memory Caching & Indexing (`src/storage/search.rs`)**
   - B-tree and B+ tree index lookups execute in $O(\log N)$ time.
   - Maintain lock-free or read-write lock (`RwLock`) access on index pages to allow concurrent read queries.

4. **Verification**
   - Validate database changes using `cargo check --lib`.
