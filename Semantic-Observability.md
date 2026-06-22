# AI-Native Semantic Observability

SigmaOS replaces the traditional POSIX observability stack (`syslog`, `journald`, `htop`, `dstat`) with a fundamentally different approach: **Semantic Observability**. 

By natively integrating the **Sigma Inference Engine (SIE)** into the file system and kernel logging layers, SigmaOS translates raw metrics into natural language and semantic vectors.

## Core Components

### 1. `sigma_journal`
The secure, tamper-evident logging daemon. Instead of grepping for errors, administrators can query the journal semantically: `sigma_journal_query("Why did the network drop last night?")`.

### 2. `sigma_semantic_fs_daemon`
A deep indexing daemon that constantly monitors:
- Running process health (CPU/RAM anomalies)
- Configuration file intents
- Hardware status

It vectorizes these system states and writes them into `SemanticFS`.

### 3. `sigma_dashboard` (AI Dashboard)
A GUI application built on `libzenith` that visualizes this data. When a user asks "Why are the fans spinning so loud?", the dashboard queries SemanticFS and generates a natural language diagnosis (e.g., "The compiler is currently consuming 100% CPU on all 8 cores").

## Why Semantic Observability?
Traditional operating systems force the administrator to act as an investigator—piecing together raw logs, metrics, and tracefiles to determine the root cause of an issue. 

SigmaOS shifts this burden to the OS itself. By maintaining a continuous semantic understanding of its own state, SigmaOS provides **Explainable Health**, drastically reducing mean-time-to-resolution (MTTR) for both server administrators and desktop end-users.
