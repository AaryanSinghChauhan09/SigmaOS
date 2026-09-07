# SigmaOS AI Agent Dashboard Operation Management Guide

This guide defines design patterns, widget lifecycle rules, reactive data binding standards, and ARIA accessibility guidelines for AI agents managing dashboard subsystems across SigmaOS.

---

## 1. Overview of Dashboard Subsystems in SigmaOS

SigmaOS employs visual-first, reactive dashboards across telemetry, privacy, compliance, network security, and quick-action web execution layers:

1. **System Monitor Unified Dashboard (`UnifiedDashboard`):** Real-time CPU, memory, and disk telemetry widget grid.
2. **Visual Network & Hardware Dashboard (`VisualDashboardManager`):** Firewall rule policies, WireGuard/OpenVPN tunnel connections, process capabilities, and thermal hardware telemetry.
3. **Privacy Control Dashboard (`PrivacyDashboard`):** Telemetry blocking rules, lockdown presets (`StrictLockdown`, `Custom`), and real-time privacy score calculation (0–100%).
4. **Adaptive Compliance Dashboard (`ComplianceOverviewDashboard`):** Corporate governance, taxation, environmental, and financial statutory compliance scoring with alert and deadline tracking.
5. **OliveTin Web Execution Dashboard (`OliveTin`):** Responsive HTML/CSS action dashboard simulation engine for safe command execution.

---

## 2. Dashboard Design & Performance Principles

AI agents implementing or modifying dashboard components MUST strictly follow these rules:

1. **Non-Blocking Telemetry Updates:** Updating telemetry widgets MUST execute asynchronously or via non-blocking lock-free state reads (`AtomicUsize` / `StateStore`) to prevent UI frame stutter.
2. **ARIA Accessibility Standards:** Web/desktop rendered dashboard controls MUST include accessible ARIA labels (`aria-label`, `aria-describedby`), focus indicators, and keyboard navigation support (`Enter`/`Space`).
3. **Deterministic Score Calculation:** Scoring formulas (e.g. privacy score, compliance score) MUST be pure, deterministic functions without side effects.
4. **Responsive Layout Grid Adapters:** Widgets MUST automatically reflow when rendered in CLI mode (`monitor show`), Zenith GTK/QML desktop UI, or OliveTin HTML web mode.

---

## 3. Core Dashboard Interfaces

### A. System Monitor Widget Interface
```rust
pub trait DashboardWidget {
    fn widget_id(&self) -> &'static str;
    fn update_telemetry(&mut self, val: f64);
    fn render_summary(&self) -> String;
}
```

### B. Privacy Dashboard Rule Management
```rust
impl PrivacyDashboard {
    pub fn apply_preset(&mut self, preset: PrivacyPreset) {
        match preset {
            PrivacyPreset::StrictLockdown => {
                for rule in self.rules.iter_mut() {
                    rule.enabled = true;
                }
            }
            // ...
        }
    }

    pub fn calculate_privacy_score(&self) -> u32 {
        let active_count = self.rules.iter().filter(|r| r.enabled).count();
        (active_count * 100) / self.rules.len()
    }
}
```

---

## 4. UI/UX & Keyboard Navigation Rules

* Icon-only dashboard buttons MUST include explicit `aria-label` tags.
* Dashboard status alerts (e.g., Critical Compliance Alert) MUST use high-contrast color styling (`--accent-gold`, `--accent-red`) and announce updates via `aria-live="polite"`.
