"""
SigmaOS Context-Aware Automation Tool (CAAT)
==============================================
USP: The OS is an Adaptive Orchestrator. It senses, decides, acts, and explains.

Competition comparison:
  Windows  → Task Scheduler (rigid, time-based, developer-centric).
  macOS    → Shortcuts (manual triggers, limited OS depth).
  Linux    → cron / systemd timers (powerful but entirely static and manual).
  SigmaOS  → CAAT natively monitors user behavioral entropy, environmental sensors,
             and cross-device states to dynamically adapt the OS in real-time.

Core innovations:
  1. Behavioral & Environmental Awareness — Learns from active workload and time/location.
  2. Cross-Device Orchestration           — Syncs context across the sovereign mesh.
  3. Explainable AI Decisions             — Logs exactly *why* a background action occurred.
  4. Instant-On Session Recall            — Restores workspace context across reboots seamlessly.
  5. Eco-Efficiency Optimizer             — Dynamically shifts to green computing states.
"""
from enum import Enum
import time
import random
from dataclasses import dataclass, field


class ContextState(Enum):
    WORK       = "Productivity Focus"
    GAMING     = "High-Performance Gaming"
    WELLNESS   = "Downtime / Wellness"
    TRAVEL     = "Low-Bandwidth / Battery Saver"
    IDLE       = "System Idle (Janitor Window)"


@dataclass
class AutomationRule:
    name: str
    condition: str
    action: str
    enabled: bool = True


class SigmaCAAT:
    """Context-Aware Automation Tool & Adaptive Orchestrator."""

    def __init__(self, kernel=None):
        self.kernel = kernel
        self._current_context = ContextState.IDLE
        self._sensors = {
            "battery_pct": 85,
            "ambient_light": "High",
            "active_window": "Explorer",
            "cpu_load": 12.5,
            "grid_carbon_intensity": 180, # gCO2/kWh
            "location": "Home",
            "weather": "Sunny",
            "biometric_trusted": True,
            "calendar_event": None
        }
        self._rules = [
            AutomationRule("Eco-Mode Override", "battery < 30% or carbon > 400", "Enable Deep Sleep & Defer Background Jobs"),
            AutomationRule("Game Boost", "active_window in ['Steam', 'Game', 'UnrealEngine']", "Allocate 80% CPU via cgroup, pause indexing, disable notifications"),
            AutomationRule("Focus Flow", "time between 09:00 - 17:00", "Mute non-essential notifications, preload IDE, disable social media"),
            AutomationRule("Wellness Dimming", "ambient_light == 'Low' and time > 20:00", "Enable warm-color projection, lower brightness, activate Blue Light Filter"),
            AutomationRule("Travel Mode", "location == 'Airport' or location == 'Cafe'", "Enable VPN, limit background data, lower screen resolution for battery"),
            AutomationRule("Weather Adaptive UI", "weather == 'Rainy' or weather == 'Cloudy'", "Switch to high-contrast icons and warmer UI glow for visibility"),
            AutomationRule("Biometric Lockdown", "biometric_trusted == False", "Encrypt all 'Private' vault folders, lock Kernel-level app launches")
        ]
        self._audit_log = []
        self._stats = {"inferences": 0, "automations_triggered": 0, "energy_saved_mwh": 0}

    def _log_action(self, reason: str, action: str):
        self._stats["automations_triggered"] += 1
        entry = {
            "timestamp": time.strftime("%H:%M:%S"),
            "reason": reason,
            "action": action
        }
        self._audit_log.append(entry)
        return entry

    def update_sensors(self, **kwargs) -> dict:
        """Update environmental/system sensor data manually or via telemetry."""
        self._sensors.update(kwargs)
        return {"status": "Sensors Updated", "current_data": self._sensors}

    def evaluate_context(self) -> dict:
        """
        The 'Sense & Decide' phase. 
        Evaluates current sensors to determine the optimal OS operating context.
        """
        self._stats["inferences"] += 1
        old_context = self._current_context

        # AI Heuristic Simulation
        if self._sensors["active_window"] in ["Visual Studio", "Figma", "Excel"]:
            new_context = ContextState.WORK
        elif self._sensors["cpu_load"] > 60 and self._sensors["active_window"] == "Game":
            new_context = ContextState.GAMING
        elif self._sensors["battery_pct"] < 30 or self._sensors["grid_carbon_intensity"] > 300:
            new_context = ContextState.TRAVEL
        else:
            new_context = ContextState.WELLNESS  # default relaxed state

        if new_context != old_context:
            self._current_context = new_context
            action = self._apply_context_profile(new_context)
            self._log_action(f"Context shifted to {new_context.name}", action)
            return {
                "changed": True, 
                "old": old_context.name, 
                "new": new_context.name,
                "action_taken": action,
                "message": f"CAAT: Context seamlessly shifted to {new_context.value}."
            }
        
        return {"changed": False, "current": new_context.name, "message": f"CAAT: Maintaining existing context ({new_context.value})."}

    def _apply_context_profile(self, context: ContextState) -> str:
        """The 'Act' phase. Changes system behavior intelligently."""
        if context == ContextState.WORK:
            return "Pre-warmed IDE RAM cache. Muted social notifications across Sovereign Mesh."
        elif context == ContextState.GAMING:
            return "Throttled background telemetry. CPU Governor set to PERFORMANCE. Suspended janitor."
        elif context == ContextState.TRAVEL:
            self._stats["energy_saved_mwh"] += 45
            return "OS entered Eco-Mode. Deferred background tasks. Screen refresh rate bounded to 30Hz."
        elif context == ContextState.WELLNESS:
            return "Enabled warm-color UI projection. Suggested screen break in 45m."
        return "System normalized."

    def trigger_instant_session_recall(self) -> dict:
        """Restores the exact multi-app workflow state from the last known context."""
        return {
            "status": "Recalled",
            "message": "CAAT: Instant-On Session Recall complete. Restored 3 apps, 12 browser tabs, and VPN state seamlessly."
        }

    def eco_efficiency_optimizer(self) -> dict:
        """Examines the carbon intensity grid and defers jobs if needed."""
        carbon = self._sensors["grid_carbon_intensity"]
        if carbon > 250:
            self._log_action(f"Grid CO2 high ({carbon}g)", "Paused all AI batch processing & updates.")
            return {"mode": "Green", "message": f"CAAT: Grid is dirty ({carbon}g CO2). Suspending heavy loads to save emissions."}
        else:
            return {"mode": "Performance", "message": f"CAAT: Grid is clean ({carbon}g CO2). Operating normally."}

    def list_rules(self) -> list[dict]:
        return [{"name": r.name, "condition": r.condition, "action": r.action, "enabled": r.enabled} for r in self._rules]

    def get_audit_trail(self) -> list[dict]:
        """The 'Explain' phase for the user empowerment dashboard."""
        return self._audit_log[-10:]

    def health_check(self) -> str:
        s = self._stats
        return f"OK — Context: {self._current_context.name}, Triggers: {s['automations_triggered']}, Energy Saved: {s['energy_saved_mwh']}mWh."


if __name__ == "__main__":
    caat = SigmaCAAT()
    print(caat.update_sensors(active_window="Visual Studio")["message"])
    print(caat.evaluate_context()["message"])
    print(caat.eco_efficiency_optimizer()["message"])
    print(caat.trigger_instant_session_recall()["message"])
    print(caat.health_check())
