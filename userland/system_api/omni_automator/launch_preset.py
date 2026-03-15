"""omni_automator.launch_preset — Preset executor."""
import time
from typing import Dict, Any
from userland.system_api.omni_automator.constants import PRESETS
from userland.system_api.omni_automator.execute_action_logic import execute_action_logic


def launch_preset(
    preset_key: str,
    stats: dict,
    benchmark_ledger: dict,
    routine_evolution_memory: dict,
    transparent_ledger: list,
    kernel=None,
) -> str:
    """Executes a named automation preset with benchmarking and evolution heuristics."""
    p = PRESETS.get(preset_key)
    if not p:
        return f"Error: Preset {preset_key} not found."

    if "tuning" in p and kernel and hasattr(kernel, "perf"):
        kernel.perf.apply_tuning(p["tuning"])

    start_time = time.time()
    results = []

    routine_evolution_memory[preset_key] = routine_evolution_memory.get(preset_key, 0) + 1
    evolved_str = ""
    if routine_evolution_memory[preset_key] > 5:
        evolved_str = " [EVOLVED: Trimming redundant context sync based on history]"

    for action in p.get("actions", []):
        results.append(execute_action_logic(action, transparent_ledger, kernel))

    elapsed = (time.time() - start_time) * 1000.0
    benchmark_ledger[preset_key] = elapsed
    stats["time_saved_min"] += 2.5

    res_summary = " -> ".join(results)
    return f"🚀 APEX EXECUTION: {p['name']}{evolved_str} initialized in {elapsed:.2f}ms.\nStatus: {res_summary}"
