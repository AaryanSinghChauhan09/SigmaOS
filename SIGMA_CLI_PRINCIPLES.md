# The Principles of the SigmaOS CLI (SigmaShell)

SigmaOS rejects the 1970s UNIX philosophy that "everything is a text stream." To build the ultimate modern OS, our CLI operates on a new set of uncompromising principles, combining the best aspects of UNIX minimalism and modern object-oriented paradigms.

### Principle 1: Object-Oriented Pipelining (The Anti-Text Doctrine)
**Competitor USP Absorbed: PowerShell**
Piping in SigmaShell (`|`) does not send chaotic byte streams that require fragile `awk`/`grep` parsing. It transmits strictly defined **C11 Memory Structs** and **JSON Objects**. 
*If you pipe `siglist` to `sigacrypt`, the target command receives a memory-safe file pointer and capability token, not a string of text. This makes pipelines 10,000x faster and mathematically immune to string-parsing exploits.*

### Principle 2: Zero-Trust Immutability 
**Competitor USP Absorbed: CoreOS / macOS SIP**
Every CLI command running in SigmaOS (like `sigaguard` or `shardctl`) must present its **Capability Ticket**. The CLI cannot bypass the secure boot or biometric enclave. There is no omnipotent `sudo`. Root is replaced by cryptographic biometric thresholds.

### Principle 3: Idempotent & State-Aware Execution
**Competitor USP Absorbed: NixOS / Ansible**
Commands like `sigpkg` or `sigadisk` declare the *desired state*. If you run a command to partition a encrypted storage drive, the CLI recognizes if the state is already achieved and cleanly exits, preventing data-destroying duplication. You script the infrastructure, not the steps.

### Principle 4: Universal Semantic Filtering (`sigastream`)
**Competitor USP Absorbed: `jq` / `sed` / `awk`**
Instead of forcing developers to master three different regex tools, SigmaShell unifies mutation under `sigastream`. Because everything is an object, `sigastream` queries data using direct pathing (e.g. `siglist | sigastream select .network_stack.threads > 5`) completely eliminating brittle regex logic from shell scripts.

### Principle 5: Asynchronous By Default
**Competitor USP Absorbed: Node.js / IOCP**
Standard UNIX commands block the main thread. SigmaShell commands natively background themselves to the `AdvancedScheduler` when waiting on I/O. Firing off 1,000 `sigafetch` (our `curl` analog) commands won't freeze the shell; they report back via callback UI notifications.
