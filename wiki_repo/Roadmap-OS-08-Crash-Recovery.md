# SigmaOS Roadmap: Automated Crash Recovery
Automatically restart failed services and restore state after kernel panic.
## Goals
- Watchdog timer with exponential backoff restart
- State snapshot before every critical operation
## Key Milestones
- [ ] Service dependency graph in sigma_logic
- [ ] Exponential backoff restart policy
- [ ] State restore from SovereignFS snapshot