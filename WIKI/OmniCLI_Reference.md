# OmniCLI Zero-Dependency Reference Guide

SigmaOS operates via the `SovereignOmniCLI`, a native C11 dispatcher that handles all kernel triggers natively. Due to our zero-dependency constraints, we do not use bash wrappers.

### Available System Invokers

#### System Operations (`sys`)

- **`sigma sys kill <pid>`**: Instantly terminates processes via raw `_sigma_sys_kill_pid` C hooks bypassing standard DAEMON constraints.

#### Virtual File System (`fs`)

- **`sigma fs ls <target>`**: Directly queries the SigmaOS mapped-memory directory trees identically to standard Linux `ls`.
- **`sigma fs read <target>`**: Reads the raw buffer shards of an exact file seamlessly like `cat`.

#### UI Orchestration (`ui`)

- **`sigma ui open <app_name>`**: Spins up WebAssembly UI contexts mapped from C directly to hardware buffers.
- **`sigma ui close <app_name>`**: Destroys the UI shard dynamically.
- **`sigma ui minimize <app_name>`**: Temporarily suspends UI memory projections.

#### Artificial Intelligence (`ai`)

- **`sigma ai <prompt>`**: Passes unformatted text straight to the local `SovereignAIKernel_ExecutePrompt` logic. Execution occurs directly in `kernel/ai_ml/`.

#### Package Management (`pkg`)

- **`sigma pkg install <target>`**: Simulates APT and NixOS declarative configuration fetches by compiling target shards natively into memory instantly.

#### Defensive / Offensive Cyber (`cyber`)

- **`sigma cyber scan <target>`**: Deploys native C memory scanners targeting foreign infrastructure without leaning onto Kali Linux style Python hacking suites.

#### Workspace / Developer Tools (`work`)

- **`sigma work edit <file>`**: Spins up the Zenith Editor using zero-latency framebuffers, replacing Vim/VSCode natively.
- **`sigma work vcs`**: Executes memory-snapshot state saving natively, neutralizing the need for heavy Git object trees.
- **`sigma work mux`**: Multiplexes shell UI blocks directly in memory. Detaches seamlessly like TMUX.

#### Data Persistence & Caching (`db`)

- **`sigma db query`**: Directly queries RAII structs to bypass PostgreSQL / Redis protocol overhead. Zero SQL string parsing latency.

#### Site Reliability & Delivery (`cicd` / `monitor`)

- **`sigma cicd`**: Hot-reloads all running shards simultaneously. Rivals Jenkins and Kubernetes rolling updates implicitly.
- **`sigma monitor`**: Dumps native C11 hardware logs directly matching Prometheus/Grafana functionality instantly.

#### Machine Learning (`ml`)

- **`sigma ml <dataset>`**: Invokes `SovereignML_RunInference()` logic. Fully integrated on local silicon with zero python wrapper dependencies.

#### Legal Compliance Database (`law`)

- **`sigma law <section_number>`**: Executes `SovereignIndianLaw_Query()` parsing local C-struct definitions of compliance logs dynamically, guaranteeing offline availability.

#### Defense Connectivity (`net`)

- **`sigma net`**: Performs `SovereignNet_ZeroTrustHandshake()`. Instantiates a `SovereignNetZenith` network shard to securely encrypt outward traffic at the hardware frame level before reaching standard proxy routes.

#### Advanced Data Analytics (`ds`)

- **`sigma ds <query>`**: Starts pure C-driven histogram and tensor math across mapped buffers using `SovereignDataScience_RunAnalysis()`.

> Note: All commands map back directly to `sovereign_tools/SovereignOmniCLI.c`. Any modification must compile through the C11 toolchain structure.
