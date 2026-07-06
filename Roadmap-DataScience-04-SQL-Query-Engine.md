# SigmaOS Roadmap: Embedded SQL Query Engine
Query sigma_db and system telemetry using SQL-like syntax.
## Goals
- SELECT / WHERE / GROUP BY / ORDER BY support
- Query execution on static-array backed tables
## Key Milestones
- [ ] SQL lexer and parser (recursive descent)
- [ ] Query planner (table scan + filter)
- [ ] Result serialisation to JSON