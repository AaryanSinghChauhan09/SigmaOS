# SigmaOS Roadmap: Natural Language â†’ CLI Translator
Translate plain-English sentences into shell commands using the embedded LLM.
## Goals
- sigma-ai "list all files modified today" â†’ ls -lt --time-style=+%F | grep 
- Zero network calls; all inference on-device
## Key Milestones
- [ ] Prompt engineering template for command generation
- [ ] Safety filter to block destructive commands without confirmation
- [ ] Command preview mode before execution