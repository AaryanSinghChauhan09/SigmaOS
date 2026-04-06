# 🧮 Automating SigmaOS

SigmaOS embeds **persona-aware automation** directly into the kernel — no external tools like cron, Ansible, or systemd required.

---

## Automation Architecture

```text
User / Persona Profile
    └─► Automation Shard (automation_shard.c)
            ├─► Task Scheduler (scheduler_ai.c)
            ├─► Macro Engine  (automation_shard.c)
            ├─► Remote Bot    (sovereign_tools/remote_bot.c)
            └─► XClicker      (sovereign_tools/xclicker.c)
```

---

## Built-in Automation Capabilities

| Tool | File | Description |
| ---- | ---- | ----------- |
| **Task Scheduler** | `scheduler_ai.c` | AI-predicted task slots using lightweight heuristics |
| **Macro Engine** | `automation_shard.c` | Record, replay, and script event automation |
| **XClicker** | `xclicker.c` | High-frequency auto-clicker with jitter profiles |
| **AutoKey Port** | `MATRIX_TOOLS` config | Keyboard macro automation via IPC injection |
| **Remote Bot** | `remote_bot.c` | Executes scheduled tasks on remote session endpoints |
| **Backup Manager** | `backup_manager.c` | Incremental VFS snapshot scheduling |

---

## Task Scheduler (`scheduler_ai.c`)

Uses a lightweight ML heuristic to predict peak load periods and schedule tasks in low-utilization windows:

```c
// Shard registers its task
sigma_schedule_task("daily_scan", SIGMA_TRIGGER_INTERVAL, 86400);

// Scheduler AI evaluates priority
sigma_ai_schedule_optimize();
```

---

## Automation via Omni Shell

```bash
# Create a repeating task
sigma task create --name "Daily VFS Backup" --interval 86400 --cmd "sigma vfs snapshot"

# List all scheduled tasks
sigma task list

# Run immediately
sigma task run "Daily VFS Backup"

# Remove task
sigma task remove "Daily VFS Backup"
```

---

## Persona-Aware Automation Profiles

| Persona | Automated Tasks |
| --------- | ---------------- |
| **Developer** | Auto-build on file change, auto-commit reminders, dependency scanner |
| **Student** | Daily quiz generation at 08:00, spaced-repetition flashcard review |
| **Forensic Analyst** | Hourly memory snapshot, real-time PCAP capture, audit log rotation |
| **Gamer** | Auto-launch gaming shard on login, performance tuning at session start |

---

## Roadmap

- [ ] GUI drag-and-drop automation workflow builder
- [ ] AI-generated automation rule suggestions based on usage patterns
- [ ] Export/import automation profiles as `.sigma` bundles
- [ ] Webhook triggers for external event integration
