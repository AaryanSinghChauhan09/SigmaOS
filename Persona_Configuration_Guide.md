# Persona Configuration Guide: Contextual Sovereignty

In SigmaOS, you do not simply manage "User Accounts." You manage **Personas**. 
Standard operating systems (like Linux or Windows) use a static permission model (`root` vs `user`), where the underlying system behaves exactly the same regardless of what you are doing.

SigmaOS utilizes **Contextual Hypervisor Boundaries**. When you shift a Persona, the entire Operating System—from the CPU governors in Ring-0 down to the UI DOM—dynamically morphs to optimize for your exact intent.

---

## 🎭 The 5 Core System Personas

| Persona Identity | Execution Focus | Hardware / Kernel Behavior Shifts |
|---|---|---|
| **The Developer** | Maximize compilation speed and shell access. | • Unlocks full `C11` build capabilities in `omni_shell`.<br>• Allocates 80% RAM directly to the `SovereignBuildMaster`.<br>• Bypasses all aggressive sandboxing for trusted local workspace directories. |
| **The Gamer** | Absolute zero-latency input and maximized framerates. | • Halts all background indexing or VFS backup daemons instantly.<br>• Routs GPU mapping directly to the framebuffer (bypassing the DOM compositor if needed).<br>• Sets CPU governors to absolute maximum TSC thresholds. |
| **The Forensic Analyst** | Strict machine isolation, tracking, and packet auditing. | • Engages the `Amnesic_Forensic_Scrubbing_Algorithm` on all temporary memory allocations.<br>• Forces every incoming/outgoing network packet through zero-trust DMA bounds.<br>• Prevents any binary execution not explicitly signed in the kernel block-list. |
| **The Researcher** | Maximize parallel AI model processing and Data Science (DSA) topologies. | • Activates the `BioInformatics_Shard` AVX-512 pipelines by default.<br>• Unlocks the `sigma-ai distribute` omni-prompt for massive cross-LLM comparisons.<br>• Allocates VFS block deduplication limits for massive datasets. |
| **The Student (Academy)** | Distraction elimination and focus isolation. | • Executes the `academy.c` shard, enforcing strict UI-lockout on non-educational processes.<br>• Disables complex terminal commands to maintain a simplified workspace.<br>• Activates the internal AI-Tutor daemon for instantaneous query interception. |

---

## ⚙️ How to Orchestrate Personas

You do not need to reboot the server to shift a Persona. It is an instantaneous, hot-swappable namespace shift managed via the **Omni Shell**.

### Switching Context
```bash
# Shift the entire machine hypervisor into Developer Mode
sigma_invoke persona --shift developer

# Enter absolute lockout focus mode
invoke academy
```

### Auto-Scheduling (The Future Roadmap)
*As outlined in our OS Development roadmap, SigmaOS will eventually support predictive Persona shifting:*
- **Time-Based:** Automatically morphing from *Researcher Mode* during the day into *Gamer Mode* at 21:00, altering CPU voltages automatically.
- **Payload-Based:** Detecting a `.c` file compilation and instantly raising the process privileges to mimic the *Developer* context for that specific PID.

## 🛡️ Capability Bounding (Strict Limits)

When a Persona is engaged, its limits are absolute. Even if a malicious payload successfully executes, a payload trapped inside the **Student** persona physically cannot issue a raw `SYS_READ` into the network stack, because the kernel `keyboard_master.c` hardware interrupts simply refuse to route it to the network DMA ring.

SigmaOS Personas enforce security at the silicon level, not just the file-permission level.
