# SigmaOS Roadmap: Whisper Voice Command Shell
Enable spoken natural-language commands to control the OS without network access.
## Goals
- Run openai/whisper-tiny and whisper-base locally
- Pipe recognised text to SigmaAI Agent CLI translator
## Key Milestones
- [ ] Ring-buffer audio capture from HAL audio driver
- [ ] Offline VAD (Voice Activity Detection) stub
- [ ] Whisper GGUF inference integration
- [ ] Command dispatch via existing IPC channel