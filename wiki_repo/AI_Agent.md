# SigmaOS Natural Language CLI Agent (SigmaAI Shell)

## Overview
SigmaOS incorporates a Natural Language-to-CLI shell agent (`SigmaAI Shell`) built directly into the terminal emulator. The agent parses user intent, translates it into safe command-line executions, and validates them against system capability tokens before execution, protecting the user from destructive commands.

## Architecture & Safety Verification
```
 [User Prompt (e.g., "compress downloads")]
                     │
                     ▼
       [Local ONNX Model parser]
                     │
                     ▼
          [Proposed CLI Command]
         ("tar -czf downloads.tar.gz")
                     │
                     ▼
         [Capability-Token Check] ──► Violates Policy? ──► Terminate
                     │
                     ▼ Passes
        [Execution inside Sandboxed Shell]
```

## System Properties
The shell agent parameters are defined in `/etc/sigma/agent.conf`:
```toml
[agent]
enabled = true
model_path = "/usr/share/sigma-ai/models/phi-3-mini-q4.onnx"
safety_threshold = 0.85
interactive_confirmation = true

[permissions]
blocked_commands = ["rm -rf /", "mkfs", "dd"]
```

## Technical Implementation
The translation parser maps raw strings to executable shell tokens using localized model weights.

```rust
// agents/sigma_ai_agent.rs
pub fn parse_intent_to_cmd(prompt: &str, runtime: &SigmaAIRuntime) -> Result<String, AgentError> {
    let system_instructions = "Translate the prompt to a safe POSIX command.";
    let formatted_prompt = format!("{} Prompt: {}", system_instructions, prompt);
    let raw_output = runtime.infer(&formatted_prompt)?;
    
    // Validate command safety before returning
    if is_command_malicious(&raw_output) {
        return Err(AgentError::MaliciousCommandBlocked);
    }
    Ok(raw_output)
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Intent mapping engine and local command translation CLI.
- **Phase 2 (Months 3-6)**: Sandbox execution sandbox for testing proposed commands.
- **Phase 3 (Months 6-9)**: Multi-step script generator with interactive step-by-step debugger.
- **Phase 4 (Months 9-12)**: Self-healing shell integration that auto-corrects command syntax errors.
