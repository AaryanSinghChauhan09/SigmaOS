import os
import subprocess

def append_commands_to_guide():
    repo_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    os.chdir(repo_dir)
    guide_path = "os_guide.md"

    new_content = """

## 🗂️ COMMAND TAXONOMY FOR SIGMAOS (OMNI-SHELL)
SigmaOS incorporates a robust command taxonomy designed to scale beyond 10,000 unique commands through base verbs, explicit objects, and flag-based expansion, entirely circumventing the reliance on high-level Python parsers in favor of C11 native parsing.

### 1. System Management
- `sigma-update` → Update system shards and native modules.
- `sigma-shard list` → List all dynamically loaded active shards.
- `sigma-shard purge <name>` → Remove shard natively from memory and disk.
- `sigma-kernel tune --latency` → Optimize the C11/ASM kernel for low-latency execution.
- `sigma-backup create --incremental` → Snapshot system state via VFS Amnesic Persistence.

### 2. Package & Cross-Distro Software Absorption
- `sigma-pkg install <package>`
- `sigma-pkg remove <package>`
- `sigma-pkg search <query>`
- `sigma-pkg personality switch arch` → Emulate Arch behavior (pacman commands).
- `sigma-pkg rebuild --source` → Gentoo-style low-level C11 compilation.
- `sigma-distro personality ubuntu` / `fedora` / `gentoo` / `nix` → Universal Absorption.

### 3. Desktop & Window Management (Direct-Canvas UI)
- `sigma-ui window open <app>` / `sigma-ui window close <id>`
- `sigma-ui window resize --width 800 --height 600`
- `sigma-ui window move --x 100 --y 200`
- `sigma-ui workspace switch dev`
- `sigma-ui workspace tile --vertical` / `split --ratio 70:30`

### 4. Customization & Personalization (UI Morphing)
- `sigma-ui theme set dark` / `sigma-ui theme list`
- `sigma-ui persona gamer` / `sigma-ui morph --minimal`
- `sigma-ui wallpaper set <file>` / `sigma-ui font set <family>`
- `sigma-ui cursor style <type>`
- `sigma-ui dock position bottom` / `sigma-ui dock auto-hide enable`

### 5. File Management & Media
- `sigma-file open <path>` / `sigma-file copy <src> <dest>` / `sigma-file delete <path>`
- `sigma-file search <query>` / `sigma-file compress <path>` / `sigma-file extract <archive>`
- `sigma-media play <file>` / `sigma-media pause` / `sigma-media stop`
- `sigma-media volume set 50` / `sigma-media record <output>`

### 6. Security, Privacy & Compliance
- `sigma-sec audit` → Run low-level SELinux/AppArmor parity checks natively.
- `sigma-sec sandbox <app>` → Launch an application strictly in an isolated namespace.
- `sigma-sec encrypt home` / `sigma-sec decrypt <path>`
- `sigma-sec persona qubes` → Switch system to absolute Qubes-style compartmentalized isolation.
- `sigma-sec lock screen` / `sigma-sec logout`
- `sigma-sec user add <name>` / `sigma-sec user remove <name>`

### 7. Performance & Hardware Efficiency
- `sigma-perf profile` → Display real-time system performance insights without overhead.
- `sigma-perf tune --gpu-priority` / `sigma-perf cache prefetch <app>`
- `sigma-perf shard optimize`
- `sigma-monitor cpu` / `sigma-monitor memory` / `sigma-monitor disk`
- `sigma-monitor network` / `sigma-monitor processes` / `sigma-monitor logs tail`

### 8. Networking & Connectivity
- `sigma-net connect <ssid>` / `sigma-net firewall enable`
- `sigma-net monitor --live` / `sigma-net persona server`
- `sigma-net wifi connect <ssid>` / `sigma-net wifi disconnect`
- `sigma-net bluetooth pair <device>` / `sigma-net vpn connect <profile>`
- `sigma-net firewall rule add <rule>`

### 9. Automation, Intelligence & Scalability
- `sigma-auto schedule <task>` / `sigma-auto recipe apply <file>`
- `sigma-auto heal` → Self-repair system anomalies utilizing local, low-level Sentinel logs.
- `sigma-ai optimize workload` / `sigma-ai predict cache` / `sigma-ai tune --ml`

### 📈 Scaling Strategy: 10,000+ Commands
By utilizing **Namespaces** (`sigma-ui`, `sigma-sec`, `sigma-net`), **Personas** (developer, gamer, server), and **Flags** (`--force`, `--dry-run`, `--source`, `--verbose`, `--json`), the Omnibus terminal architecture inherently expands from a few hundred robust native commands to a virtually infinite topology capable of handling every possible computational requirement.
"""

    with open(guide_path, "a", encoding="utf-8") as f:
        f.write(new_content)

    print("Appended Command Taxonomy documentation to os_guide.md.")

    # Commit and push
    try:
        subprocess.run(["git", "add", "os_guide.md"], check=True)
        subprocess.run(["git", "commit", "-m", "Incorporate SigmaOS 10000+ Command Taxonomy Strategy"], check=True)
        subprocess.run(["git", "push"], check=True)
        print("Successfully synced Command Taxonomy with GitHub.")
    except Exception as e:
        print(f"Git operations failed: {e}")

if __name__ == "__main__":
    append_commands_to_guide()
