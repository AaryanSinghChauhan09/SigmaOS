# SigmaOS Roadmap: AI-Assisted Kernel Debugger
Use LLM to analyse GDB backtraces and suggest root-cause fixes.
## Goals
- Parse GDB RSP output and feed to local LLM context
- Generate fix suggestions with confidence scores
## Key Milestones
- [ ] GDB backtrace structured parser
- [ ] Targeted fix-suggestion prompt template
- [ ] Inline suggestion overlay in Zenith editor