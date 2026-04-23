# Algorithmic Automations & UX

SigmaOS employs advanced bare-metal algorithms to achieve performance and UX fluidity that standard heuristic-based operating systems cannot match.

Located in `modules/core/kernel/ai_scheduler_v2.c` and `modules/ui/ux_heuristics.c`.

## Competitive Advantages (USPs)

### 1. Q-Learning Kernel Scheduler
- **Standard OS**: Linux uses CFS (Completely Fair Scheduler), which uses static mathematical formulas (virtual runtime) to guess how much CPU a process needs. It struggles with sudden spikes in UI latency.
- **SigmaOS USP**: The kernel integrates a **Reinforcement Learning (Q-Learning)** algorithm directly into the scheduler. 
  - **State**: Page faults and IPC frequency.
  - **Reward**: Minimizing UI latency and CPU stalls.
  - **Action**: Dynamically increasing or decreasing a process's CPU timeslice.
  The scheduler mathematically *learns* the optimal timeslice for every workload over time.

### 2. Predictive UX Automation (Markov Chains)
- **Standard OS**: When you click an app, the OS begins allocating memory and reading from the disk. This creates launch latency.
- **SigmaOS USP**: The Zenith UI incorporates a **Markov Chain transition matrix**. The kernel algorithmically tracks which apps you open sequentially (e.g., you usually open your Compiler immediately after your IDE). When the algorithm hits a high confidence threshold, it proactively provisions physical memory contracts and pre-caches the next application *before you even click on it*. This creates the illusion of zero-latency application launching.
