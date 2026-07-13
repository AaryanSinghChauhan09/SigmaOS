# SOVEREIGN FOCUS MODE

> **Component**: `kernel/shards/focus_mode/` | **Category**: UX/Productivity | **Status**: Planned

The **SigmaOS Sovereign Focus Mode** is a distraction-free computing environment that combines OS-level resource management, notification suppression, and AI-powered workflow assistance into a unified productivity layer.

---

## Overview

Unlike simple "Do Not Disturb" implementations, Sovereign Focus Mode operates at the **kernel level** — it actively re-prioritizes system resources toward the focused application, suppresses all non-critical background activity, and uses AI telemetry to detect and maintain optimal cognitive load states.

```
┌──────────────────────────────────────────────────────────────┐
│                  FOCUS MODE ARCHITECTURE                     │
│                                                              │
│  User triggers "focus" → FocusShard activates               │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                  FOCUS POLICY ENGINE                  │   │
│  │                                                       │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │   │
│  │  │  Notification│  │  CPU/Memory  │  │  Network   │  │   │
│  │  │  Suppressor  │  │  Rebalancer  │  │  Throttle  │  │   │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│              ↕ sigma-bus IPC                                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                  AI FOCUS ASSISTANT                   │   │
│  │   • Detects context switches                          │   │
│  │   • Suggests break times (Pomodoro-aware)             │   │
│  │   • Learns your focus patterns                        │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

---

## Features

### 1. OS-Level Distraction Elimination

| Suppressed | Allowed |
|---|---|
| Social media notifications | Critical system alerts |
| Email/chat pings | Calendar reminders (optional) |
| System update popups | Security events |
| Background app CPU | Focused app + terminals |
| Auto-sync services | Manual saves |
| Screen dimming | Emergency overrides |

### 2. Resource Rebalancing

During focus mode, the scheduler and memory manager prioritize the focused process group:

```toml
# /etc/sigma/focus_mode.toml
[focus]
focused_app_cpu_weight   = 80    # 80% of CPU budget
background_cpu_weight    = 10    # background gets 10%
system_cpu_weight        = 10    # kernel/daemons get 10%

[memory]
focused_app_oom_score    = -500  # Never OOM-kill focused app
background_oom_score     = 900   # OOM-kill background first

[network]
allow_focused_app        = true
throttle_background_mbps = 0.5   # 512KB/s for background
```

### 3. AI-Powered Assistance

```rust
// The AI Focus Assistant runs as a lightweight shard
// It monitors typing cadence, app switch frequency,
// and scroll patterns to infer focus depth

pub struct FocusMetrics {
    pub keystrokes_per_min:  f32,  // typing activity
    pub app_switches_per_hr: f32,  // context switches
    pub idle_time_sec:       u32,  // time since last keystroke
    pub focus_depth_score:   f32,  // 0.0 (unfocused) to 1.0 (deep flow)
}

impl FocusAssistant {
    /// Suggest a break when focus depth drops below threshold
    pub fn check_break_needed(&self, metrics: &FocusMetrics) -> Option<BreakSuggestion> {
        if metrics.focus_depth_score < 0.3 && self.time_in_focus_min > 90 {
            Some(BreakSuggestion::FiveMinuteBreak)
        } else if self.time_in_focus_min > 50 {
            Some(BreakSuggestion::PomodoroBreak)
        } else {
            None
        }
    }
}
```

---

## CLI Interface

```bash
# Enter focus mode (auto-detect current focused window)
sigma focus

# Focus on specific app
sigma focus --app "code"

# Focus with timer (Pomodoro: 25min work)
sigma focus --duration 25m

# Focus with allowed list
sigma focus --allow "calendar,terminal"

# Check focus status
sigma focus status
# Focus Mode: ACTIVE (18 minutes)
# Focused App: Visual Studio Code
# Notifications suppressed: 12
# Context switches prevented: 3
# Focus depth score: 0.87 (Deep Flow)

# Exit focus mode
sigma focus off
```

---

## Zenith Desktop Integration

The Zenith desktop shows a subtle focus indicator in the header:

```
┌─────────────────────────────────────────────────────────────┐
│  🎯 FOCUS  [██████████░░░░]  18/25min  | ← End Focus       │
└─────────────────────────────────────────────────────────────┘
```

---

## Privacy Guarantees

- All AI metrics processed **locally** — never sent to network
- Focus patterns stored in encrypted local database
- No behavioral telemetry without explicit opt-in
- Focus data purged after 30 days (configurable)

---

## Configuration

```toml
# ~/.config/sigma/focus.toml

[pomodoro]
work_duration_min  = 25
short_break_min    = 5
long_break_min     = 15
cycles_before_long = 4

[notifications]
allow_list = ["calendar.critical", "security.alert"]
block_list = ["social.*", "email.*", "chat.*"]

[ai_assistant]
enabled          = true
break_suggestions = true
focus_depth_hud  = true
learning_mode    = true  # learns your patterns
privacy_mode     = true  # local-only, no network
```

---

## Roadmap

- [ ] Basic notification suppression (Q2)
- [ ] CPU/memory rebalancing during focus (Q2)
- [ ] Zenith HUD integration (Q3)
- [ ] AI focus depth detection (Q3)
- [ ] Pomodoro timer with smart break detection (Q4)
- [ ] Multi-monitor focus mode (one focused, others dimmed) (Q4)
- [ ] Voice command: "Hey Sigma, focus mode on" (Year 2)
