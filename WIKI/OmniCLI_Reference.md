# OmniCLI Zero-Dependency Reference Guide

SigmaOS operates via the `SovereignOmniCLI`, a native C11 dispatcher that handles all kernel triggers natively. Due to our zero-dependency constraints, we do not use bash wrappers.

### Available System Invokers

#### System Operations (`sys`)

- **`sigma sys kill <pid>`**: Instantly terminates processes via raw `_sigma_sys_kill_pid` C hooks bypassing standard DAEMON constraints.

#### UI Orchestration (`ui`)

- **`sigma ui open <app_name>`**: Spins up WebAssembly UI contexts mapped from C directly to hardware buffers.
- **`sigma ui close <app_name>`**: Destroys the UI shard dynamically.
- **`sigma ui minimize <app_name>`**: Temporarily suspends UI memory projections.

#### Artificial Intelligence (`ai`)

- **`sigma ai <prompt>`**: Passes unformatted text straight to the local `SovereignAIKernel_ExecutePrompt` logic. Execution occurs directly in `kernel/ai_ml/`.

#### Machine Learning (`ml`)

- **`sigma ml <dataset>`**: Invokes `SovereignML_RunInference()` logic. Fully integrated on local silicon with zero python wrapper dependencies.

#### Legal Compliance Database (`law`)

- **`sigma law <section_number>`**: Executes `SovereignIndianLaw_Query()` parsing local C-struct definitions of compliance logs dynamically, guaranteeing offline availability.

#### Defense Connectivity (`net`)

- **`sigma net`**: Performs `SovereignNet_ZeroTrustHandshake()`. Instantiates a `SovereignNetZenith` network shard to securely encrypt outward traffic at the hardware frame level before reaching standard proxy routes.

#### Advanced Data Analytics (`ds`)

- **`sigma ds <query>`**: Starts pure C-driven histogram and tensor math across mapped buffers using `SovereignDataScience_RunAnalysis()`.

> Note: All commands map back directly to `sovereign_tools/SovereignOmniCLI.c`. Any modification must compile through the C11 toolchain structure.
