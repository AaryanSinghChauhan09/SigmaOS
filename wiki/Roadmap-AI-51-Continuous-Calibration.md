# SigmaOS Roadmap: Continuous Calibration of Local LLMs
Enable background self-calibration algorithms to prevent drift in model confidence scoring.
## Goals
- Compute temperature scaling parameters dynamically from local user feedback cycles.
- Integrate verification steps inside local_llm.rs query routing.
## Key Milestones
- [ ] Expected Calibration Error (ECE) metric tracking
- [ ] Optimization solver for temperature scaling
- [ ] Self-calibrating log-probability extraction