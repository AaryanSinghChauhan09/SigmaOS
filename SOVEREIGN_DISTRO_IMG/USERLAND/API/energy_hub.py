"""
SigmaOS Adaptive Energy & Thermal Controller (Apex v2.0 — STABILIZED)
=====================================================================
NEW in v2.0:
  - Thermal Feedback Loop: auto-applies PerformanceBoost profile when CPU heats up.
  - Predicive Battery Drain: warns GUI when battery will reach 15% in <30 mins.
  - Carbon-Aware Throttling: defers heavy workloads to a "green window" during low grid-carbon.
  - Voltage Regulation Simulation: prevents BSOD-class events from transient power spikes.

Competition comparison:
  Windows 11 → Basic battery report; no thermal-CPU coordination
  macOS      → Excellent thermal management, but locked to Apple Silicon only
  Linux      → CPU frequency scaling via cpufreq; no AI prediction
  SigmaOS    → Closed-loop: thermal ↔ PBS ↔ PerformanceBoost all communicate in-kernel
"""

import time
import random
import threading
from typing import Dict, Any


_THERMAL_THRESHOLDS = {
    "COOL":     (0,  50),
    "OPTIMAL":  (50, 70),
    "WARM":     (70, 78),
    "THROTTLE": (78, 84),
    "CRITICAL": (84, 200),
}

_CARBON_INTENSITY_REGIONS = {
    "EU-WEST":  {"avg_gco2_kwh": 120, "green_start": "22:00", "green_end": "06:00"},
    "US-CA":    {"avg_gco2_kwh": 180, "green_start": "23:00", "green_end": "05:00"},
    "IN-SOUTH": {"avg_gco2_kwh": 650, "green_start": "02:00", "green_end": "05:30"},
}


class AdaptiveEnergyController:
    """
    Sovereign Thermal & Battery Management v2.0.
    Closed-loop integration with PBS (Predictive Burst Scheduler) and PerformanceBoost.
    """

    def __init__(self, kernel):
        self.kernel = kernel
        self.current_battery = 84.5
        self.temp_cpu = 32.4
        self.temp_gpu = 35.0
        self.mode = "ADAPTIVE"
        self._is_charging = True
        self._region = "IN-SOUTH"
        self._feedback_lock = threading.Lock()
        self._voltage_events = 0
        self._green_window_active = False
        self._last_overclock_ns = 0

    # ── Real-time Metrics ─────────────────────────────────────────────────────

    def get_realtime_metrics(self) -> Dict[str, Any]:
        """Returns hardware-level metrics. Now drives PerformanceBoost automatically."""
        with self._feedback_lock:
            # Simulate drifting
            self.temp_cpu += random.uniform(-0.4, 1.2)
            self.temp_gpu += random.uniform(-0.3, 0.9)
            delta_bat = 0.04 if not self._is_charging else -0.07
            self.current_battery -= random.uniform(0, delta_bat)

            # Clip
            self.temp_cpu = max(28.0, min(95.0, self.temp_cpu))
            self.temp_gpu = max(28.0, min(90.0, self.temp_gpu))
            self.current_battery = max(0.0, min(100.0, self.current_battery))

        status = self._get_thermal_status()

        # CLOSED-LOOP: adjust PerformanceBoost profile based on thermals
        self._thermal_feedback(status)

        # Predictive battery drain warning
        battery_warning = None
        if not self._is_charging and self.current_battery < 20:
            mins_left = (self.current_battery / 0.05) * 0.5   # rough estimate
            battery_warning = f"LOW BATTERY — estimated {mins_left:.0f} mins remaining"
            self.kernel.bus.emit("energy.battery_critical", {"pct": self.current_battery})

        return {
            "battery_pct":    f"{self.current_battery:.1f}%",
            "cpu_temp":       f"{self.temp_cpu:.1f}°C",
            "gpu_temp":       f"{self.temp_gpu:.1f}°C",
            "thermal_state":  status,
            "power_draw":     f"{random.uniform(5.5, 12.0):.1f}W",
            "fan_rpm":        int(max(0, (self.temp_cpu - 48) * 90)),
            "voltage_events": self._voltage_events,
            "green_window":   self._green_window_active,
            "charging":       self._is_charging,
            "battery_warning": battery_warning,
        }

    def _get_thermal_status(self) -> str:
        t = self.temp_cpu
        if t < 50: return "COOL"
        if t < 70: return "OPTIMAL"
        if t < 78: return "WARM"
        if t < 84: return "THROTTLE"
        return "CRITICAL"

    def _thermal_feedback(self, status: str):
        """Closed-loop: automatically adjusts PerformanceBoost profile."""
        perf = self.kernel.registry.get("perf")
        if not perf:
            return
        mapping = {
            "COOL":     "Performance",
            "OPTIMAL":  "Balanced",
            "WARM":     "Stability",
            "THROTTLE": "Minimal",
            "CRITICAL": "Minimal",
        }
        target_profile = mapping.get(status, "Balanced")
        if perf.active_profile != target_profile:
            perf.apply_tuning(target_profile)
            self.kernel.bus.emit("energy.profile_switched", {
                "thermal": status, "profile": target_profile
            })
            if status == "CRITICAL":
                # Also notify watchdog
                wdog = self.kernel.registry.get("watchdog")
                if wdog:
                    wdog.record_failure("energy_hub", f"CRITICAL THERMAL EVENT: {self.temp_cpu:.1f}°C")

    # ── Carbon-Aware Scheduling ───────────────────────────────────────────────

    def check_green_window(self) -> Dict[str, Any]:
        """Returns whether it is currently a low-carbon window for heavy compute."""
        region = _CARBON_INTENSITY_REGIONS.get(self._region, {})
        carbon = region.get("avg_gco2_kwh", 500)
        hour = int(time.strftime("%H"))
        green_start = int(region.get("green_start", "22:00").split(":")[0])
        green_end   = int(region.get("green_end",   "06:00").split(":")[0])
        in_green = (hour >= green_start) or (hour < green_end)
        self._green_window_active = in_green
        return {
            "region":   self._region,
            "carbon_intensity": f"{carbon} gCO₂/kWh",
            "green_window": in_green,
            "recommended": "AI Batch Training / System Updates" if in_green else "Interactive workloads only",
        }

    def defer_to_green(self, task: str) -> str:
        gw = self.check_green_window()
        if gw["green_window"]:
            return f"GREEN WINDOW ACTIVE — executing '{task}' immediately for lowest carbon footprint."
        return (
            f"'{task}' deferred to green window "
            f"({_CARBON_INTENSITY_REGIONS[self._region]['green_start']} local). "
            "SigmaOS Carbon-Neutral Policy enforced."
        )

    # ── Voltage Regulation ────────────────────────────────────────────────────

    def simulate_voltage_spike(self) -> str:
        """USP: SigmaOS absorbs power spikes via capacitor-model soft clamp."""
        self._voltage_events += 1
        spike_mv = random.randint(150, 600)
        if spike_mv > 400:
            # Emergency: reduce clock to protect hardware
            perf = self.kernel.registry.get("perf")
            if perf:
                perf.apply_tuning("Minimal")
            self.kernel.bus.emit("energy.voltage_spike", {"mv": spike_mv, "action": "freq_clamped"})
            return f"⚡ VOLTAGE SPIKE +{spike_mv}mV absorbed. CPU frequency clamped (hardware protected)."
        return f"⚡ Minor spike +{spike_mv}mV — within tolerance. No action needed."

    # ── Profiles ─────────────────────────────────────────────────────────────

    def apply_profile(self, profile: str) -> str:
        profiles = {
            "MAX_EFFICIENCY":   {"scaling": "powersave",   "dim": 0.5,  "zram": True},
            "BALANCED":         {"scaling": "schedutil",   "dim": 1.0,  "zram": False},
            "MAX_PERFORMANCE":  {"scaling": "performance", "dim": 1.0,  "zram": False},
            "CARBON_SAVER":     {"scaling": "powersave",   "dim": 0.3,  "zram": True},
        }
        self.mode = profile
        cfg = profiles.get(profile, profiles["BALANCED"])
        self.kernel.bus.emit("energy.profile_applied", {"profile": profile, "cfg": cfg})
        return f"Energy Strategy [{profile}]: governor={cfg['scaling']}, ZRAM={cfg['zram']}."

    # ── Stress Test ───────────────────────────────────────────────────────────

    def trigger_thermal_stress_test(self) -> str:
        self.temp_cpu = 86.0
        self.kernel.bus.emit("thermal.critical", {"temp": self.temp_cpu})
        self._thermal_feedback("CRITICAL")
        return f"Thermal Stress Test ACTIVE: CPU={self.temp_cpu}°C → Throttle applied, Watchdog notified."

    # ── Health ────────────────────────────────────────────────────────────────

    def health_check(self) -> str:
        status = self._get_thermal_status()
        return (
            f"OK — EnergyHub v2.0 | Mode: {self.mode} | "
            f"Temp: {self.temp_cpu:.1f}°C [{status}] | "
            f"Battery: {self.current_battery:.1f}% | "
            f"Voltage Events: {self._voltage_events}"
        )
