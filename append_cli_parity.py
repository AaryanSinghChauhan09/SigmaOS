import os
import subprocess

def append_cli_modularity_docs():
    repo_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    os.chdir(repo_dir)
    guide_path = "os_guide.md"

    new_content = """

## 🧩 GRANULAR CLI MODULARITY & GUI PARITY
SigmaOS is engineered so that **every single GUI action is perfectly mirrored by a low-level CLI Omni-Shell command.** This design completely decentralizes the OS into callable micro-shards, allowing power users to script and automate entire graphical workflows natively from the terminal.

### 1. CLI / GUI Parity Integration
If a novice user clicks "Change Wallpaper" in the GUI Architect, the OS translates that action into the exact equivalent CLI command natively: `sigma-ui wallpaper set file.jpg`. This extends to everything:
- **GUI:** User switches to Gaming Mode.
- **CLI Equivalent:** `sigma-persona switch gamer --lazy` (Flushes unrelated memory, summons GPU interrupts).
- **GUI:** User runs a system update.
- **CLI Equivalent:** `sigma-update --mode ubuntu` (Translates the graphical sequence directly into the specific distro personality protocol).

### 2. Base Command Catalog Extension
The Omni-Shell provides ~200 highly optimized Base Commands traversing all logical subsystems:

#### System & Kernel Operations
- `sigma-shard load kernel --minimal`
- `sigma-shard swap scheduler --latency` (Hotswap CPU schedulers dynamically).
- `sigma-shard heal kernel` (Self-monitor and hot-patch failing memory sectors).

#### Automation & AI Routines
- `sigma-auto recipe apply <file.yaml>` (Load declarative configurations natively).
- `sigma-auto hook battery --on-change` (Establish event-driven hardware triggers instead of loops).
- `sigma-ai tune --ml` / `sigma-ai optimize workload`
- `sigma-ai prefetch net --time 09:00` (Pre-loads networking hardware into cache before daily execution).

#### Customization & Personalization
- `sigma-ui morph tiling` (Instantly rewrite the window manager logic).
- `sigma-ui theme auto --day-night`
- `sigma-perf governor performance` / `sigma-perf governor balanced`

#### Developer & Storage Controls
- `sigma-sec persona researcher` (Enforce strict SELinux/AppArmor parameters globally).
- `sigma-file compress <path>` / `sigma-file extract <archive>`
- `sigma-app launch <name> --secure`

### 3. Infinite Scalability via Flag Expansion
By leveraging **Flags**, **Personas**, and **Distro Modes**, a single concept creates an exponential number of specific commands:
- `sigma-shard load net --persona gamer` (Loads networking optimized for UDP packet latency).
- `sigma-shard load net --mode arch` (Executes Arch-style rolling network parameters).
- `sigma-shard load net --secure --minimal` (Loads tightly sandboxed networking without graphical overhead).

This structure guarantees that SigmaOS operates reliably at scale, remaining lightweight, adaptive, and infinitely customizable through CLI orchestration natively.
"""

    with open(guide_path, "a", encoding="utf-8") as f:
        f.write(new_content)

    print("Appended CLI Modularity and Parity documentation to os_guide.md.")

    # Commit and push
    try:
        subprocess.run(["git", "add", "os_guide.md"], check=True)
        subprocess.run(["git", "commit", "-m", "Incorporate CLI-GUI Parity and Fractional Modularity Expansions"], check=True)
        subprocess.run(["git", "push"], check=True)
        print("Successfully synced CLI Parity with GitHub.")
    except Exception as e:
        print(f"Git operations failed: {e}")

if __name__ == "__main__":
    append_cli_modularity_docs()
