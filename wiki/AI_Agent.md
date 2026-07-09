# SigmaOS Natural Language CLI Agent (SigmaAI Shell)

## Overview

SigmaOS incorporates a Natural Language-to-CLI shell agent (`SigmaAI Shell`) built directly into the terminal emulator. The agent parses user intent, translates it into safe command-line executions, and validates them against system capability tokens before execution, protecting the user from destructive commands.

### Key Features

- **Natural Language Interface**: Interact with your system using plain language
- **Local AI Runtime**: ONNX-based models for offline inference
- **Safety Validation**: Capability-based safety checks before execution
- **Sandbox Execution**: Commands run in isolated sandbox
- **Multi-Language Support**: Support for Indic languages
- **Interactive Confirmation**: User approval for sensitive operations
- **Self-Healing**: Auto-correction of command syntax errors

## Architecture

### Safety Verification Flow

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

### Component Architecture

```
┌─────────────────────────────────────────┐
│         SigmaAI Shell Agent             │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Intent   │ Command  │ Safety       │ │
│  │ Parser   │ Generator│ Validator    │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      AI Runtime (ONNX)                  │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Model    │ Inference│ Quantization │ │
│  │ Loader   │ Engine   │ Support     │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Capability System                  │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Token    │ Policy   │ Audit        │ │
│  │ Manager  │ Engine   │ Logger       │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Sandbox Execution                  │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Landlock │ seccomp  │ Namespace    │ │
│  │ Sandbox  │ Filters  │ Isolation    │ │
│  └──────────┴──────────┴──────────────┘ │
└─────────────────────────────────────────┘
```

## Configuration

### Agent Configuration

**File**: `/etc/sigma/agent.conf`

```toml
[agent]
enabled = true
model_path = "/usr/share/sigma-ai/models/phi-3-mini-q4.onnx"
safety_threshold = 0.85
interactive_confirmation = true
language = "en_US"
max_history = 100

[permissions]
blocked_commands = ["rm -rf /", "mkfs", "dd", ":(){:|:&};:"]
require_confirmation = ["rm", "mv", "chmod", "chown"]
allow_network = false
allow_device_access = false

[sandbox]
enabled = true
isolation_level = "strict"
allow_network = false
allow_filesystem = "/home/user"
```

### Model Configuration

**File**: `/etc/sigma/models/phi-3-mini.conf`

```toml
[model]
name = "phi-3-mini"
version = "3.8b"
quantization = "q4"
context_length = 2048
max_tokens = 512

[inference]
batch_size = 1
temperature = 0.7
top_p = 0.9
top_k = 40
repetition_penalty = 1.0

[optimization]
use_gpu = true
num_threads = 4
memory_limit = "2GB"
```

## Technical Implementation

### Intent Parser

```rust
// userland/system_api/ai_integration/src/sigma_ai_agent.rs
use onnxruntime::{Environment, Session, Tensor};

pub struct SigmaAIAgent {
    model_path: String,
    session: Session,
    safety_threshold: f32,
    capability_manager: CapabilityManager,
}

impl SigmaAIAgent {
    pub fn new(model_path: &str, safety_threshold: f32) -> Result<Self, AgentError> {
        let env = Environment::builder().build()?;
        let session = env.new_session(&model_path)?;
        
        Ok(Self {
            model_path: model_path.to_string(),
            session,
            safety_threshold,
            capability_manager: CapabilityManager::new(),
        })
    }
    
    pub fn parse_intent_to_cmd(&self, prompt: &str) -> Result<String, AgentError> {
        let system_instructions = "Translate the prompt to a safe POSIX command.";
        let formatted_prompt = format!("{} Prompt: {}", system_instructions, prompt);
        
        // Run inference
        let raw_output = self.infer(&formatted_prompt)?;
        
        // Validate command safety before returning
        if self.is_command_malicious(&raw_output) {
            return Err(AgentError::MaliciousCommandBlocked);
        }
        
        Ok(raw_output)
    }
    
    fn infer(&self, prompt: &str) -> Result<String, AgentError> {
        // Tokenize input
        let input_ids = self.tokenize(prompt)?;
        
        // Create input tensor
        let input_tensor = Tensor::from_vec(input_ids)?;
        
        // Run inference
        let outputs = self.session.run(vec![input_tensor])?;
        
        // Decode output
        let output_text = self.decode_output(outputs)?;
        
        Ok(output_text)
    }
    
    fn is_command_malicious(&self, command: &str) -> bool {
        // Check against blocked commands
        let blocked_commands = vec!["rm -rf /", "mkfs", "dd", ":(){:|:&};:"];
        
        for blocked in &blocked_commands {
            if command.contains(blocked) {
                return true;
            }
        }
        
        // Check safety score
        let safety_score = self.compute_safety_score(command);
        safety_score < self.safety_threshold
    }
    
    fn compute_safety_score(&self, command: &str) -> f32 {
        // Simple heuristic-based safety scoring
        let mut score = 1.0;
        
        // Penalize destructive operations
        if command.contains("rm") || command.contains("delete") {
            score -= 0.3;
        }
        
        // Penalize system modifications
        if command.contains("chmod") || command.contains("chown") {
            score -= 0.2;
        }
        
        // Penalize network operations
        if command.contains("curl") || command.contains("wget") {
            score -= 0.1;
        }
        
        score.max(0.0)
    }
}
```

### Safety Validator

```rust
// userland/system_api/ai_integration/src/safety_validator.rs
pub struct SafetyValidator {
    capability_manager: CapabilityManager,
    policy_engine: PolicyEngine,
}

impl SafetyValidator {
    pub fn new() -> Self {
        SafetyValidator {
            capability_manager: CapabilityManager::new(),
            policy_engine: PolicyEngine::new(),
        }
    }
    
    pub fn validate_command(&self, command: &str, process_id: u64) -> Result<ValidationResult, ValidationError> {
        // Parse command
        let parsed_command = self.parse_command(command)?;
        
        // Check against policy
        let policy_result = self.policy_engine.check(&parsed_command)?;
        
        if !policy_result.allowed {
            return Ok(ValidationResult {
                allowed: false,
                reason: policy_result.reason,
                requires_confirmation: false,
            });
        }
        
        // Check capabilities
        let capability_result = self.check_capabilities(&parsed_command, process_id)?;
        
        if !capability_result.allowed {
            return Ok(ValidationResult {
                allowed: false,
                reason: capability_result.reason,
                requires_confirmation: false,
            });
        }
        
        Ok(ValidationResult {
            allowed: true,
            reason: String::new(),
            requires_confirmation: self.requires_confirmation(&parsed_command),
        })
    }
    
    fn requires_confirmation(&self, command: &ParsedCommand) -> bool {
        let sensitive_commands = vec!["rm", "mv", "chmod", "chown", "dd"];
        sensitive_commands.contains(&command.program.as_str())
    }
}
```

### Sandbox Execution

```rust
// userland/system_api/ai_integration/src/sandbox.rs
use landlock::{Ruleset, Access, PathBeneath};

pub struct CommandSandbox {
    ruleset: Ruleset,
}

impl CommandSandbox {
    pub fn new(allowed_paths: &[PathBuf]) -> Result<Self, SandboxError> {
        let mut ruleset = Ruleset::new()
            .handle_access(Access::FS_READ_FILE)
            .handle_access(Access::FS_WRITE_FILE);
        
        for path in allowed_paths {
            let path_beneath = PathBeneath::new(path.clone(), Access::all());
            ruleset.add_rule(path_beneath)?;
        }
        
        Ok(Self { ruleset })
    }
    
    pub fn execute_command(&self, command: &str) -> Result<CommandResult, SandboxError> {
        // Apply sandbox restrictions
        self.ruleset.apply()?;
        
        // Execute command
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;
        
        Ok(CommandResult {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
```

## Multi-Language Support

### Supported Languages

- **English (en_US)**: Primary language
- **Hindi (hi_IN)**: Devanagari script support
- **Bengali (bn_IN)**: Bengali script support
- **Tamil (ta_IN)**: Tamil script support
- **Telugu (te_IN)**: Telugu script support

### Language Configuration

```toml
[language]
default = "en_US"
fallback = "en_US"
auto_detect = true

[models.en_US]
path = "/usr/share/sigma-ai/models/phi-3-mini-en-q4.onnx"

[models.hi_IN]
path = "/usr/share/sigma-ai/models/phi-3-mini-hi-q4.onnx"
```

## Usage Examples

### Basic Usage

```bash
# Start SigmaAI Shell
sigma-ai-shell

# Natural language commands
> compress my downloads folder
# Executes: tar -czf downloads.tar.gz ~/Downloads/

> show system memory usage
# Executes: free -h

> list all running processes
# Executes: ps aux

> install git package
# Executes: sigpkg install git
```

### Interactive Confirmation

```bash
> delete all files in /tmp
⚠️  This command will delete files in /tmp
Proposed command: rm -rf /tmp/*
Continue? [y/N]: y
# Executes: rm -rf /tmp/*
```

### Multi-Step Operations

```bash
> set up a development environment for Rust
Step 1: Install Rust toolchain
Proposed: sigpkg install rust cargo
Continue? [y/N]: y

Step 2: Configure Rust
Proposed: rustup default stable
Continue? [y/N]: y

Step 3: Set up PATH
Proposed: export PATH=$HOME/.cargo/bin:$PATH
Continue? [y/N]: y
``

## Best Practices

### Development

1. **Safety First**: Always validate commands before execution
2. **Clear Intent**: Use clear, unambiguous natural language
3. **Confirmation**: Require confirmation for destructive operations
4. **Sandboxing**: Execute commands in isolated sandbox

### Configuration

1. **Model Selection**: Choose appropriate model for use case
2. **Safety Threshold**: Adjust safety threshold based on user expertise
3. **Language Support**: Enable appropriate language models
4. **Capability Limits**: Set appropriate capability limits

### Security

1. **Validation**: Validate all commands before execution
2. **Sandboxing**: Use sandbox for command execution
3. **Audit Logging**: Log all command executions
4. **User Consent**: Require user consent for sensitive operations

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Intent mapping engine
- Local command translation CLI
- Basic safety validation
- ONNX model integration

### Phase 2 (Months 3-6)
- Sandbox execution environment
- Advanced safety validation
- Capability-based access control
- Interactive confirmation system

### Phase 3 (Months 6-9)
- Multi-step script generator
- Interactive step-by-step debugger
- Multi-language support
- Indic language models

### Phase 4 (Months 9-12)
- Self-healing shell integration
- Auto-correction of syntax errors
- Advanced context awareness
- Learning from user behavior

## References

- [ONNX Runtime](https://onnxruntime.ai/)
- [Phi-3 Model](https://huggingface.co/microsoft/Phi-3-mini-4k-instruct)
- [Landlock Documentation](https://www.kernel.org/doc/html/latest/userspace-api/landlock.html)
- [Natural Language Processing](https://www.nltk.org/)
- [CLI Design Patterns](https://clig.dev/)
