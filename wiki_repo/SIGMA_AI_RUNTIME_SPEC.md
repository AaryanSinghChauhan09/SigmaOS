# SigmaAI Local Runtime Specification

## 1. Core Vision
SigmaOS integrates AI not as a cloud-dependent afterthought, but as a deeply embedded, local-first runtime. By utilizing heavily quantized open-source models, SigmaOS provides secure, private automation, primarily focusing on Natural Language to CLI translations and system administration assistance.

## 2. Implementation (`userland/sigma_ai/src/lib.rs`)
- **Quantization:** The runtime is optimized for INT4 and INT8 quantized models to ensure they can run entirely on local CPUs/NPUs without requiring massive amounts of RAM.
- **Signed Model Marketplace:** Users cannot arbitrarily download unverified weights from the internet and execute them in the system context. Models must be cryptographically signed by the SigmaOS trusted keyring, ensuring the supply chain of the model weights is untampered.

## 3. NL -> CLI with Dry-Run Safety
When a user asks SigmaOS "Find all logs containing authentication failures":
1. The AI translates this to `grep "auth fail" /var/log/*`.
2. **Dry-Run Safety Engine:** The AI does *not* execute the command blindly. It stages the command into a safety buffer and prompts the user for explicit elevation and execution confirmation.

## 4. Privacy and Provenance
- Zero data leaves the machine. All inference is offline-first.
- The OS logs the provenance of AI-generated actions. If an AI suggests a command that mutates system state, the action is logged in a secure audit trail, noting that the command was AI-generated rather than user-typed.
