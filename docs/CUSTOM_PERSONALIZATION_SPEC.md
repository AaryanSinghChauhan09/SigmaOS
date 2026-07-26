# 🎨 Custom Personalization & Automation Specification (`S-Custom`)

This specification details the architecture, evaluation loops, adaptive theme transitions, and conditional action pipelines of the **Custom Personalization & Customization Engine** (`S-Custom`) for SigmaOS.

Drawing inspiration from **Samsung Modes & Routines** and **Android Adaptive Profiles**, the personalization engine is a zero-dependency, context-aware automation framework built entirely with Object-Oriented Programming (OOP) principles and user-defined functions in Rust, Zig, and Nim.

---

## 🗺️ Customization System Architecture

```
                    ┌────────────────────────────────────────┐
                    │       System Diagnostics / Context     │
                    └───────────────────┬────────────────────┘
                                        │ (Time, Location, Battery, Active App)
                    ┌───────────────────▼────────────────────┐
                    │      Customization Evaluation Loop     │
                    └───────────────────┬────────────────────┘
                                        │ (Conditional Trigger Matching)
         ┌──────────────────────────────┼──────────────────────────────┐
         ▼                              ▼                              ▼
 ┌──────────────┐               ┌──────────────┐               ┌──────────────┐
 │ Set Theme    │               │  Launch App  │               │ Run Scripts  │
 └──────────────┘               └──────────────┘               └──────────────┘
```

---

## 1. Zero-Dependency OOP Rust Specification (Evaluation & Trigger Loops)

The core customization engine dynamically evaluates registered user routines based on diagnostic metrics, executing contextual actions without memory allocations.

```rust
pub const MAX_ROUTINES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    Time,
    Battery,
    Activity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    SetDarkMode,
    SetLightMode,
    ThrottlingEnable,
    ThrottlingDisable,
}

pub struct CustomCondition {
    pub trigger: TriggerType,
    pub threshold: u32,
}

impl CustomCondition {
    pub fn new(trigger: TriggerType, threshold: u32) -> Self {
        Self { trigger, threshold }
    }

    pub fn evaluate(&self, current_val: u32) -> bool {
        match self.trigger {
            TriggerType::Time => current_val == self.threshold,
            TriggerType::Battery => current_val < self.threshold,
            TriggerType::Activity => current_val > self.threshold,
        }
    }
}

pub struct CustomRoutine {
    pub id: u32,
    pub condition: CustomCondition,
    pub action: Action,
    pub enabled: bool,
}

impl CustomRoutine {
    pub fn new(id: u32, condition: CustomCondition, action: Action) -> Self {
        Self {
            id,
            condition,
            action,
            enabled: true,
        }
    }
}

pub struct PersonalizationEngine {
    routines: [Option<CustomRoutine>; MAX_ROUTINES],
}

impl PersonalizationEngine {
    pub fn new() -> Self {
        const NONE_ROUTINE: Option<CustomRoutine> = None;
        Self {
            routines: [NONE_ROUTINE; MAX_ROUTINES],
        }
    }

    pub fn register_routine(&mut self, routine: CustomRoutine) -> Result<(), &'static str> {
        for slot in self.routines.iter_mut() {
            if slot.is_none() {
                *slot = Some(routine);
                return Ok(());
            }
        }
        Err("Personalization routine boundary limits exceeded")
    }

    pub fn process_context(&self, trigger: TriggerType, current_val: u32, output_queue: &mut [Option<Action>]) -> usize {
        let mut count = 0;
        for slot in self.routines.iter() {
            if let Some(ref routine) = slot {
                if routine.enabled && routine.condition.trigger == trigger {
                    if routine.condition.evaluate(current_val) {
                        if count < output_queue.len() {
                            output_queue[count] = Some(routine.action);
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod personalization_tests {
    use super::*;

    #[test]
    fn test_routine_automation() {
        let mut engine = PersonalizationEngine::new();
        let battery_condition = CustomCondition::new(TriggerType::Battery, 20); // Battery < 20%
        let battery_routine = CustomRoutine::new(1, battery_condition, Action::ThrottlingEnable);

        assert!(engine.register_routine(battery_routine).is_ok());

        let mut actions = [None; 4];
        let triggered = engine.process_context(TriggerType::Battery, 15, &mut actions);

        assert_eq!(triggered, 1);
        assert_eq!(actions[0], Some(Action::ThrottlingEnable));
    }
}
```

---

## 2. Zero-Dependency OOP Zig Specification (Fast State Evaluators)

Provides immediate, memory-safe, real-time evaluation of low-level diagnostic boundaries (e.g., thermal thresholds) without runtime overhead.

```zig
const std = @import("std");

pub const SystemTrigger = enum {
    ThermalLevel,
    MemoryLevel,
};

pub const ProfileAction = enum {
    ScaleFrequencyDown,
    ScaleFrequencyUp,
    LogThermalWarning,
};

pub const OptimizationRule = struct {
    trigger: SystemTrigger,
    threshold_value: u32,
    action: ProfileAction,

    pub fn check(self: *const OptimizationRule, val: u32) ?ProfileAction {
        switch (self.trigger) {
            .ThermalLevel => {
                if (val >= self.threshold_value) {
                    return self.action;
                }
            },
            .MemoryLevel => {
                if (val >= self.threshold_value) {
                    return self.action;
                }
            },
        }
        return null;
    }
};
```

---

## 3. Zero-Dependency OOP Nim Specification (Adaptive Desktop Theming)

Triggers unprivileged userland commands to modify the active visual layouts, color sets, and notification profiles dynamically.

```nim
type
  ThemeConfig* = object
    backgroundHex*: string
    foregroundHex*: string
    fontFamily*: string

  ThemeCustomizer* = ref object of RootObj
    activeThemeName*: string
    activeConfig*: ThemeConfig

method applyTheme*(self: ThemeCustomizer, name: string, bg: string, fg: string, font: string) {.base.} =
  self.activeThemeName = name
  self.activeConfig.backgroundHex = bg
  self.activeConfig.foregroundHex = fg
  self.activeConfig.fontFamily = font

proc newThemeCustomizer*(name: string): ThemeCustomizer =
  new(result)
  result.activeThemeName = name
  result.activeConfig = ThemeConfig(backgroundHex: "#0F172A", foregroundHex: "#F8FAFC", fontFamily: "sans-serif")
```

---

## 🔄 Customization Checklist & Quality Assurance

To secure perfect, seamless customization compared to Samsung:
1.  **Adaptive Battery Profiling:** Automatically enable low-power thermal throttles when battery levels drop beneath user-defined boundaries.
2.  **No-Latency Transitions:** Guarantee theme switching updates desktop compositing layers instantly without dropping window frame updates.
3.  **Encapsulated Automation:** All userland custom scripts executed by routines must be gated strictly by sandbox capability profiles.
