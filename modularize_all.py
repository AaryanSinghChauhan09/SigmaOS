"""
SigmaOS Deep Modularizer v2.0
==============================
Breaks each class method into its own standalone file (one function = one file).
Targets the largest files with multiple functions.
"""

import os
import ast
import sys
import textwrap

ROOT = r"."

# ─────────────────────────────────────────────────────────────
# Helper: safely write a file without overwriting if exists
# ─────────────────────────────────────────────────────────────
def safe_write(path: str, content: str, overwrite: bool = True):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if os.path.exists(path) and not overwrite:
        print(f"  [SKIP] {path} already exists.")
        return
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"  [WROTE] {path}")


def make_init(folder: str, imports: list[str]):
    """Create/update __init__.py in folder with given import lines."""
    init_path = os.path.join(folder, "__init__.py")
    header = '"""Auto-generated __init__.py — SigmaOS deep modularizer."""\n\n'
    body = "\n".join(imports) + "\n"
    safe_write(init_path, header + body, overwrite=True)


# ─────────────────────────────────────────────────────────────
# 1. Modularize fluid_ui.py  →  userland/system_api/fluid_ui/
# ─────────────────────────────────────────────────────────────
def modularize_fluid_ui():
    PKG = os.path.join(ROOT, "userland", "system_api", "fluid_ui")
    os.makedirs(PKG, exist_ok=True)

    modules = {
        "render_taskbar": '''\
"""fluid_ui.render_taskbar — Fluid Taskbar Matrix renderer."""


def render_taskbar_extension(kernel=None) -> str:
    """USP: Native multi-monitor taskbar with sub-millisecond predictive rendering."""
    cpu = "4%"
    if kernel and hasattr(kernel, "perf"):
        metrics = kernel.perf.get_telemetry()
        cpu = metrics.get("cpu_load", "4%")
    return f"Fluid Taskbar Matrix: [CPU: {cpu} | GPU: Ready | Missions: ALIVE]"
''',
        "universal_search": '''\
"""fluid_ui.universal_search — OmniSearch nexus launcher."""


def launch_universal_search() -> str:
    """USP: Spotlight/Raycast analog, integrated natively into the kernel bus."""
    return "Search Index: 1M+ Local Nodes synchronized. OmniSearch Nexus Ready."
''',
        "window_transparency": '''\
"""fluid_ui.window_transparency — Glassmorphism transparency control."""


def apply_window_transparency(state: dict, kernel=None, alpha: float = 0.9) -> str:
    """Aesthetic Customization: Hardware-accelerated Glassmorphism."""
    state["transparency_alpha"] = max(0.1, min(1.0, alpha))
    if kernel and hasattr(kernel, "bus"):
        kernel.bus.emit("ui.transparency_shifted", {"alpha": state["transparency_alpha"]})
    return f"Window Compositor: Applied {state['transparency_alpha'] * 100}% transparency via DWM-Hooks."
''',
        "snap_window": '''\
"""fluid_ui.snap_window — Physics-based window snapping."""

VALID_ZONES = ["Left_Half", "Right_Half", "Top_Left", "ZenCenter"]


def snap_window(state: dict, kernel=None, app_id: str = "", zone: str = "") -> str:
    """USP: Physics-based window snapping (Magnetic Layouts)."""
    if zone not in VALID_ZONES:
        return "Error: Invalid Snap Zone."
    msg = f"Magnetic Snap: \'{app_id}\' locked to \'{zone}\' (Tension: {state[\'physics\'][\'spring_tension\']})"
    if kernel and hasattr(kernel, "bus"):
        kernel.bus.emit("ui.window_snapped", {"app": app_id, "zone": zone})
    return msg
''',
        "configure_widgets": '''\
"""fluid_ui.configure_widgets — Live telemetry widget manager."""
from typing import Dict, Any, List, Optional


def configure_widgets(state: dict, add: Optional[List[str]] = None, remove: Optional[List[str]] = None) -> Dict[str, Any]:
    """Personalization: Dynamically anchor or remove live telemetry widgets."""
    if add:
        state["active_widgets"].extend(add)
    if remove:
        state["active_widgets"] = [w for w in state["active_widgets"] if w not in remove]
    state["active_widgets"] = list(dict.fromkeys(state["active_widgets"]))
    return {
        "status": "Fluid Layout Adjusted",
        "active_widgets": state["active_widgets"],
        "message": f"UI Nexus: Now anchoring {len(state['active_widgets'])} live widgets to the desktop plane.",
    }
''',
        "cognitive_metamorphosis": '''\
"""fluid_ui.cognitive_metamorphosis — Instant Cognitive UI morphing."""


def instant_cognitive_metamorphosis(state: dict, stress_level: float, task_type: str) -> str:
    """USP: Phase 3 Singularity - Instant Cognitive UI based on biometric/task inputs."""
    from userland.system_api.fluid_ui.configure_widgets import configure_widgets
    from userland.system_api.fluid_ui.window_transparency import apply_window_transparency

    if stress_level > 7.5:
        state["layout_mode"] = "Absolute_Minimalism (High-Stress Override)"
        configure_widgets(state, remove=list(state["active_widgets"]))
        apply_window_transparency(state, alpha=1.0)
        return (
            f"COGNITIVE-UI: Stress spike detected ({stress_level}). "
            "Liquidating distractions. Shell morphing into pure text-focus layout."
        )
    if "creative" in task_type.lower():
        state["layout_mode"] = "Holographic_Canvas"
        apply_window_transparency(state, alpha=0.4)
        return "COGNITIVE-UI: Creative intent sensed. Shell borders dissolved. Glassmorphism maximized to 60%."
    return "COGNITIVE-UI: Biological steady-state. Standard morphological boundaries maintained."
''',
        "adaptive_theme": '''\
"""fluid_ui.adaptive_theme — Adaptive context-aware theme engine."""


def set_adaptive_theme(state: dict, context: str) -> str:
    """Personalization: Autonomously shift UI based on environmental context."""
    from userland.system_api.fluid_ui.window_transparency import apply_window_transparency
    from userland.system_api.fluid_ui.configure_widgets import configure_widgets

    ctx = context.lower()
    if ctx == "night":
        state["layout_mode"] = "Abyssal_Dark"
        apply_window_transparency(state, alpha=0.95)
    elif ctx == "gaming":
        state["layout_mode"] = "Performance_Solid"
        apply_window_transparency(state, alpha=1.0)
    elif ctx == "focus":
        state["layout_mode"] = "Zen_Minimalist"
        apply_window_transparency(state, alpha=0.7)
        configure_widgets(state, remove=["social_feed", "stocks"])
    else:
        state["layout_mode"] = "Dynamic_Glass"
    return f"Adaptive Theme Engine: Metamorphosis to \'{state['layout_mode']}\' complete."
''',
        "health_check": '''\
"""fluid_ui.health_check — FluidUI health probe."""


def health_check(state: dict) -> str:
    """Returns health status of the Fluid UI subsystem."""
    return f"OK — Fluid UI | Mode: {state['layout_mode']} | Cognitive Morphing Active."
''',
        "accessibility_suite": '''\
"""fluid_ui.accessibility_suite — Accessibility feature manifest."""
from typing import Dict


def get_accessibility_suite() -> Dict[str, str]:
    """Industry Leader: Integrated Neural Screen Reader and Gesture Logic."""
    return {
        "Voice_Control": "Active (Local-NPU Zero Latency)",
        "Haptic_Feedback": "Calibrated (Curve-matched)",
        "High_Contrast": "Available (Color-Blindness Adaptive)",
        "Eye_Tracking": "Ready (Cursor-Lock enabled)",
    }
''',
    }

    imports = []
    for mod_name, content in modules.items():
        safe_write(os.path.join(PKG, f"{mod_name}.py"), content)
        # figure out function/class name from content for __init__
        imports.append(f"from .{mod_name} import *  # noqa: F401,F403")

    # Compat shim — keep old fluid_ui.py but re-export from package
    shim = '''\
"""
fluid_ui — SigmaOS Fluid UI (v3 Apex)
======================================
Backward-compat shim.  Real implementation lives in fluid_ui/ package.
"""
import time
from typing import Dict, Any, List, Optional

from userland.system_api.fluid_ui.render_taskbar import render_taskbar_extension
from userland.system_api.fluid_ui.universal_search import launch_universal_search
from userland.system_api.fluid_ui.window_transparency import apply_window_transparency
from userland.system_api.fluid_ui.snap_window import snap_window
from userland.system_api.fluid_ui.configure_widgets import configure_widgets
from userland.system_api.fluid_ui.cognitive_metamorphosis import instant_cognitive_metamorphosis
from userland.system_api.fluid_ui.adaptive_theme import set_adaptive_theme
from userland.system_api.fluid_ui.health_check import health_check
from userland.system_api.fluid_ui.accessibility_suite import get_accessibility_suite


class SigmaFluidUI:
    """
    Sovereign Fluid UI (v3 Apex)
    ============================
    USP: Replaces clunky, static Desktop Environments with a physics-based,
    AI-adaptive UI layer. Class is a thin facade over the modular function package.
    """

    def __init__(self, kernel=None, user_name="Sovereign-User"):
        self.kernel = kernel
        self.user = user_name
        self._state = {
            "layout_mode": "Dynamic_Glass",
            "active_widgets": ["cpu_glance", "mission_control"],
            "physics": {"friction": 0.85, "spring_tension": 300},
            "transparency_alpha": 0.9,
        }

    # ── Delegating facade methods ────────────────────────────
    def render_taskbar_extension(self) -> str:
        return render_taskbar_extension(self.kernel)

    def launch_universal_search(self) -> str:
        return launch_universal_search()

    def apply_window_transparency(self, alpha: float = 0.9) -> str:
        return apply_window_transparency(self._state, self.kernel, alpha)

    def snap_window(self, app_id: str, zone: str) -> str:
        return snap_window(self._state, self.kernel, app_id, zone)

    def configure_widgets(self, add=None, remove=None):
        return configure_widgets(self._state, add, remove)

    def instant_cognitive_metamorphosis(self, stress_level: float, task_type: str) -> str:
        return instant_cognitive_metamorphosis(self._state, stress_level, task_type)

    def set_adaptive_theme(self, context: str) -> str:
        return set_adaptive_theme(self._state, context)

    def health_check(self) -> str:
        return health_check(self._state)

    @staticmethod
    def get_accessibility_suite() -> Dict[str, str]:
        return get_accessibility_suite()

    # ── State property pass-throughs ──────────────────────────
    @property
    def layout_mode(self) -> str:
        return self._state["layout_mode"]

    @property
    def transparency_alpha(self) -> float:
        return self._state["transparency_alpha"]

    @property
    def active_widgets(self) -> List[str]:
        return self._state["active_widgets"]
'''
    # Write shim to the original path (overwrite)
    old_path = os.path.join(ROOT, "userland", "system_api", "fluid_ui.py")
    safe_write(old_path, shim, overwrite=True)

    make_init(PKG, imports)
    print("[OK] fluid_ui modularized.")


# ─────────────────────────────────────────────────────────────
# 2. Modularize omni_automator.py → userland/system_api/omni_automator/
# ─────────────────────────────────────────────────────────────
def modularize_omni_automator():
    PKG = os.path.join(ROOT, "userland", "system_api", "omni_automator")
    os.makedirs(PKG, exist_ok=True)

    files = {
        "mission_node.py": '''\
"""omni_automator.mission_node — MissionNode dataclass definition."""
from dataclasses import dataclass, field
from typing import Dict, Any, Optional


@dataclass
class MissionNode:
    id: str
    name: str
    node_type: str
    params: Dict[str, Any] = field(default_factory=dict)
    next_node_id: Optional[str] = None
    execution_time_ms: float = 0.0
''',
        "constants.py": '''\
"""omni_automator.constants — Mission library, presets, and config."""
from typing import Dict, Any

MISSION_LIBRARY: Dict[str, list] = {
    "Hardening": ["Kill_Legacy_Shims", "Update_Sovereign_Policies", "Seal_Shadow_Vault"],
    "Optimization": ["Flush_VRAM", "Steer_IRQs", "Trigger_Prewarmer"],
    "Sync": ["Mesh_Merkle_Verify", "Push_to_Origin_Master"],
}

PRESETS: Dict[str, Dict[str, Any]] = {
    "Gaming_Apex": {
        "name": "🎮 Gaming Apex Mode",
        "tuning": "Gaming",
        "actions": ["Hyper_Drive_Engage", "Starve_Background", "Apply_Aura:CyberPunk"],
        "description": "Unlocks maximum silicon potential for zero-latency gameplay.",
    },
    "Nightly_Purge": {
        "name": "🧹 Nightly System Purge",
        "actions": ["Flush_VRAM", "Mesh_Sync_Critical", "Scrub_Temp_Files", "Apply_Aura:DeepSpace"],
        "description": "Optimizes storage and security while the user rests.",
    },
    "Deep_Focus": {
        "name": "🧠 Deep Focus Protocol",
        "tuning": "Efficiency",
        "actions": ["Mute_Notifications", "Block_Distractions", "Apply_Aura:Monolith", "Starve_Background"],
        "description": "Engages zero-interruption hyper-focus state.",
    },
    "Creative_Flow": {
        "name": "🎨 Creative Flow State",
        "tuning": "Performance",
        "actions": ["Boost_GPU_Priority", "Enable_Spatial_Audio", "Apply_Aura:Fluency"],
        "description": "Allocates maximum media/render power and fluid aesthetics.",
    },
}
''',
        "get_preview_card.py": '''\
"""omni_automator.get_preview_card — Transparent execution preview."""
from typing import Dict, Any
from userland.system_api.omni_automator.constants import PRESETS


def get_preview_card(preset_key: str) -> Dict[str, Any]:
    """USP: Transparent Execution Log Previews before committing to ring-0 hardware routines."""
    p = PRESETS.get(preset_key)
    if not p:
        return {"Error": "Preset Not Found"}
    return {
        "Card_Title": f"🔍 Preview: {p[\'name\']}",
        "Expected_Resource_Shift": f"CPU/GPU will pivot to \'{p.get(\'tuning\', \'Balanced\')}\' mode.",
        "Execution_DAG": p.get("actions", []),
        "Impact_Rating": "High (Kernel Modifications)" if "tuning" in p else "Low (Userland Only)",
        "Trust_Level": "VERIFIED_0xAPEX",
    }
''',
        "decompose_intent.py": '''\
"""omni_automator.decompose_intent — Intent → MissionNode DAG decomposer."""
from typing import List
from userland.system_api.omni_automator.mission_node import MissionNode


def decompose_intent(intent: str) -> List[MissionNode]:
    """Decomposes a natural-language intent string into a MissionNode DAG."""
    nodes = []
    low_intent = intent.lower()
    nodes.append(MissionNode("n0", "Ingest_Context", "action", {"intent": intent}))

    if "security" in low_intent or "harden" in low_intent:
        nodes.extend([
            MissionNode("n1", "Seal_Vaults", "action"),
            MissionNode("n2", "Audit_Syscalls", "decision"),
        ])
        nodes[0].next_node_id = "n1"
        nodes[1].next_node_id = "n2"
    else:
        nodes.append(MissionNode("n1", "Autonomous_Execution", "action"))
        nodes[0].next_node_id = "n1"

    return nodes
''',
        "execute_action_logic.py": '''\
"""omni_automator.execute_action_logic — Individual action executor."""
import time
from typing import Dict, Any, List


def execute_action_logic(action: str, ledger: List[Dict[str, Any]], kernel=None) -> str:
    """Executes a single named action and records result to the transparent ledger."""
    msg = f"Executed: {action}"

    if "Apply_Aura:" in action:
        aura_name = action.split(":")[1]
        if kernel and hasattr(kernel, "aura"):
            kernel.aura.apply_aura(aura_name)
            msg = f"AURA: Shifted to {aura_name}"
    elif action == "Hyper_Drive_Engage":
        if kernel and hasattr(kernel, "perf"):
            kernel.perf.apply_tuning("Performance")
            msg = "PERF: Hyper-Drive Engaged."
    elif action == "Flush_VRAM":
        if kernel and hasattr(kernel, "perf"):
            kernel.perf._flush_vram_buffers()
            msg = "MEM: VRAM Flushed."
    elif action == "Mute_Notifications":
        msg = "FOCUS: Hardware interrupt silencing active."
    elif action == "Block_Distractions":
        msg = "FOCUS: Network Guardian enforcing packet drop on non-critical sites."
    elif action == "Starve_Background":
        msg = "PERF: Background threads starved of CPU cycles."
    elif action == "Boost_GPU_Priority":
        msg = "PERF: CUDA/Vulkan scheduling pinned to REALTIME."
    elif action == "Enable_Spatial_Audio":
        msg = "AUDIO: Sovereign Spatial acoustic dampening enabled."
    elif action == "Scrub_Temp_Files":
        msg = "FS: SigmaFS swept temp sectors securely."
    elif action == "Mesh_Sync_Critical":
        msg = "SYNC: Off-site Merkle synchronization completed."

    if kernel and hasattr(kernel, "bus"):
        kernel.bus.emit("auto.action_log", {"msg": msg})

    ledger.append({
        "timestamp": time.ctime(),
        "action": action,
        "result_status": msg,
        "trust_verifier": "Sigma_Swarm_Audit_0x0",
    })
    return msg
''',
        "launch_mission.py": '''\
"""omni_automator.launch_mission — Mission launcher."""
import uuid
from typing import List, Dict
from userland.system_api.omni_automator.mission_node import MissionNode
from userland.system_api.omni_automator.decompose_intent import decompose_intent


def launch_mission(
    intent: str,
    active_missions: Dict[str, List[MissionNode]],
    stats: dict,
) -> str:
    """Launches a new autonomous mission from a natural-language intent."""
    uid_str = uuid.uuid4().hex
    mid = f"mission-{uid_str[:8]}"
    active_missions[mid] = decompose_intent(intent)
    stats["workflows_executed"] += 1
    return f"OmniAutomator Apex: Mission \'{mid}\' launched for intent: \'{intent}\'."
''',
        "launch_preset.py": '''\
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
    return f"🚀 APEX EXECUTION: {p[\'name\']}{evolved_str} initialized in {elapsed:.2f}ms.\\nStatus: {res_summary}"
''',
        "genome.py": '''\
"""omni_automator.genome — Workflow genome extraction & synthesis."""
from typing import Dict
from userland.system_api.omni_automator.constants import PRESETS
from userland.system_api.omni_automator.execute_action_logic import execute_action_logic


def extract_workflow_genome(preset_key: str, genome_db: Dict[str, str]) -> str:
    """USP: Synthesize workflows into reusable DNA mapped structurally via DAG."""
    p = PRESETS.get(preset_key)
    if not p:
        return "ERROR: NO_GENOME"
    actions = p.get("actions", [])
    genome_sig = f"SGM-{hash(\'|\'.join(actions))}-v1"
    genome_db[genome_sig] = "|".join(actions)
    return genome_sig


def synthesize_from_genome(
    genome_sig: str,
    genome_db: Dict[str, str],
    transparent_ledger: list,
    stats: dict,
    kernel=None,
) -> str:
    """USP: Recombine and execute a workflow directly from its DNA string."""
    if genome_sig not in genome_db:
        return f"Genome {genome_sig} not found in sequence library."
    actions = genome_db[genome_sig].split("|")
    for action in actions:
        execute_action_logic(action, transparent_ledger, kernel)
    stats["workflows_executed"] += 1
    return f"GENOME RE-SEQUENCED: Executed {len(actions)} nodes seamlessly."
''',
        "sentinel.py": '''\
"""omni_automator.sentinel — Proactive autonomous sentinel loop."""
import time
import threading
from typing import Dict, Any


class OmniSentinel:
    """Proactive OS Intelligence — decides when to shift modes based on real-time telemetry."""

    def __init__(self, stats: dict, kernel=None, launch_preset_fn=None):
        self.stats = stats
        self.kernel = kernel
        self.launch_preset_fn = launch_preset_fn
        self._running = False
        self._thread: threading.Thread | None = None

    def start(self):
        """Start the proactive sentinel daemon thread."""
        if not self._running:
            self._running = True
            self._thread = threading.Thread(target=self._cycle, daemon=True)
            self._thread.start()
            print("[OMNI] Proactive Sentinel [ONLINE].")

    def stop(self):
        """Stop the sentinel loop."""
        self._running = False

    def _cycle(self):
        """Autonomous Decision Loop."""
        while self._running:
            try:
                time.sleep(15)
                if self.kernel and self.kernel.perf:
                    metrics = self.kernel.perf.get_telemetry()
                    cpu = float(metrics.get("cpu_load", "0%").replace("%", ""))
                    if cpu > 80.0:
                        if self.launch_preset_fn:
                            self.launch_preset_fn("Nightly_Purge")
                        self.stats["proactive_interventions"] += 1
                        if hasattr(self.kernel, "bus"):
                            self.kernel.bus.emit(
                                "auto.sentinel_trigger", {"res": "CPU_HIGH", "action": "PURGE"}
                            )
                self.stats["actions_automated"] += 2
            except Exception as e:
                print(f"[SENTINEL_ERR] {e}")
''',
        "health_check.py": '''\
"""omni_automator.health_check — Health probe."""


def health_check(stats: dict) -> str:
    """Returns the current health status of the OmniAutomator subsystem."""
    return f"OK — OmniAutomator v5.0 | Missions Executed: {stats[\'workflows_executed\']}"
''',
        "healing_cycle.py": '''\
"""omni_automator.healing_cycle — Self-healing orchestration trigger."""


def execute_healing_cycle(kernel=None) -> str:
    """Unified self-healing orchestration — invokes repair engine if available."""
    if kernel and hasattr(kernel, "repair_engine"):
        kernel.repair_engine.repair("UAL_Shim", "Bit-drift auto-detection")
    return "Forensic-Autopilot: Restoration cycle COMPLETE."
''',
        "register_folder_action.py": '''\
"""omni_automator.register_folder_action — Folder-action binding."""


def register_folder_action(folder: str, action: str) -> str:
    """Binds an automation action to a watched folder."""
    return f"Folder Action \'{action}\' firmly bound to \'{folder}\'."
''',
        "get_benchmarks.py": '''\
"""omni_automator.get_benchmarks — Benchmark ledger accessor."""
from typing import Dict


def get_benchmarks(benchmark_ledger: Dict[str, float]) -> Dict[str, float]:
    """USP: Benchmark and compare the efficiency of different automations directly in the OS."""
    return benchmark_ledger
''',
        "get_transparent_ledger.py": '''\
"""omni_automator.get_transparent_ledger — Execution log accessor."""
from typing import List, Dict, Any


def get_transparent_ledger(transparent_ledger: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """USP: Human-readable execution log that traces every single action taken by the AI swarm."""
    return transparent_ledger
''',
    }

    imports = []
    for fname, content in files.items():
        safe_write(os.path.join(PKG, fname), content)
        imports.append(f"from .{fname[:-3]} import *  # noqa")

    # Compat shim
    shim = '''\
"""
omni_automator — SigmaOS OmniAutomator (v5.0 Apex Singularity)
================================================================
Backward-compat shim.  Real implementation lives in omni_automator/ package.
"""
from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional
import time
import uuid
import threading

from userland.system_api.agentic_claw import SigmaAgenticClaw, ActionNode
from userland.system_api.omni_automator.mission_node import MissionNode
from userland.system_api.omni_automator.constants import MISSION_LIBRARY, PRESETS
from userland.system_api.omni_automator.get_preview_card import get_preview_card
from userland.system_api.omni_automator.decompose_intent import decompose_intent
from userland.system_api.omni_automator.execute_action_logic import execute_action_logic
from userland.system_api.omni_automator.launch_mission import launch_mission as _launch_mission
from userland.system_api.omni_automator.launch_preset import launch_preset as _launch_preset
from userland.system_api.omni_automator.genome import extract_workflow_genome, synthesize_from_genome
from userland.system_api.omni_automator.sentinel import OmniSentinel
from userland.system_api.omni_automator.health_check import health_check as _health_check
from userland.system_api.omni_automator.healing_cycle import execute_healing_cycle as _healing_cycle
from userland.system_api.omni_automator.register_folder_action import register_folder_action as _reg_folder
from userland.system_api.omni_automator.get_benchmarks import get_benchmarks as _get_benchmarks
from userland.system_api.omni_automator.get_transparent_ledger import get_transparent_ledger as _get_ledger


class ISigmaModule: pass
class SigmaModuleBase:
    def __init__(self, kernel=None): self.kernel = kernel


class SigmaOmniAutomator(SigmaModuleBase):
    """Unified Agentic Backplane. Thin facade over the modular omni_automator package."""

    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.claw = SigmaAgenticClaw(kernel)
        self.active_missions: Dict[str, List[MissionNode]] = {}
        self.variables: Dict[str, Any] = {}
        self.stats = {
            "workflows_executed": 0, "actions_automated": 0,
            "proactive_interventions": 0, "time_saved_min": 0.0,
            "missions_run": 0, "blocks_compiled": 0, "repairs_auto": 0,
        }
        self.benchmark_ledger: Dict[str, float] = {}
        self.routine_evolution_memory: Dict[str, int] = {}
        self.transparent_ledger: List[Dict[str, Any]] = []
        self.workflow_genome_db: Dict[str, str] = {}
        self.MISSION_LIBRARY = MISSION_LIBRARY
        self.PRESETS = PRESETS
        self._sentinel = OmniSentinel(self.stats, kernel, self.launch_preset)

    def get_preview_card(self, preset_key: str) -> Dict[str, Any]:
        return get_preview_card(preset_key)

    def launch_mission(self, intent: str) -> str:
        return _launch_mission(intent, self.active_missions, self.stats)

    def _decompose_intent(self, intent: str) -> List[MissionNode]:
        return decompose_intent(intent)

    def launch_preset(self, preset_key: str) -> str:
        return _launch_preset(
            preset_key, self.stats, self.benchmark_ledger,
            self.routine_evolution_memory, self.transparent_ledger, self.kernel,
        )

    def get_benchmarks(self) -> Dict[str, float]:
        return _get_benchmarks(self.benchmark_ledger)

    def get_transparent_ledger(self) -> List[Dict[str, Any]]:
        return _get_ledger(self.transparent_ledger)

    def extract_workflow_genome(self, preset_key: str) -> str:
        return extract_workflow_genome(preset_key, self.workflow_genome_db)

    def synthesize_from_genome(self, genome_sig: str) -> str:
        return synthesize_from_genome(
            genome_sig, self.workflow_genome_db, self.transparent_ledger, self.stats, self.kernel
        )

    def _execute_action_logic(self, action: str) -> str:
        return execute_action_logic(action, self.transparent_ledger, self.kernel)

    def register_folder_action(self, folder: str, action: str):
        return _reg_folder(folder, action)

    def health_check(self) -> str:
        return _health_check(self.stats)

    def execute_healing_cycle(self):
        return _healing_cycle(self.kernel)

    def start_sentinel(self):
        self._sentinel.start()

    def stop_sentinel(self):
        self._sentinel.stop()
'''
    old_path = os.path.join(ROOT, "userland", "system_api", "omni_automator.py")
    safe_write(old_path, shim, overwrite=True)
    make_init(PKG, imports)
    print("[OK] omni_automator modularized.")


# ─────────────────────────────────────────────────────────────
# 3. Modularize sigma_forge.py → sigma_forge/ package
# ─────────────────────────────────────────────────────────────
def modularize_sigma_forge():
    PKG = os.path.join(ROOT, "sigma_forge")
    os.makedirs(PKG, exist_ok=True)

    files = {
        "forge_app.py": '''\
"""sigma_forge.forge_app — App scaffold generator."""
import os


def forge_app(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS app scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f\'\'\'"""
{name} Application for SigmaOS
"""
from sigma_core.system.interfaces import SigmaModuleBase

class {class_name}(SigmaModuleBase):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.app_id = "{name}_v1"

    def run(self, *args, **kwargs):
        print(f"[{{self.app_id}}] Execution starting...")
        return "SUCCESS"

    def health_check(self):
        return f"OK - {{self.app_id}} ACTIVE"
\'\'\'
    return _write(name, output_dir, content, "app")


def _write(name: str, output_dir: str, content: str, kind: str) -> str:
    os.makedirs(output_dir, exist_ok=True)
    filename = f"{name.lower()}.py"
    target = os.path.join(output_dir, filename)
    if os.path.exists(target):
        return f"Error: \'{target}\' already exists. Forge aborted."
    with open(target, "w") as f:
        f.write(content)
    return f"Forge SUCCESS: Created {kind} \'{name}\' at {target}"
''',
        "forge_agent.py": '''\
"""sigma_forge.forge_agent — Agent scaffold generator."""
import os
from sigma_forge.forge_app import _write


def forge_agent(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS agent scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f\'\'\'"""
{name} Agent for SigmaOS
"""
from sigma_core.agent_orchestrator import SigmaAgentIsolate

class {class_name}(SigmaAgentIsolate):
    def __init__(self, agent_id, role="{name}", persona="Advanced", goal="Optimize", kernel=None):
        super().__init__(agent_id, role, persona, goal, kernel)

    def execute_mission(self, context):
        print(f"[AGENT:{{self.role}}] Handling mission: {{context}}")
        return f"Mission sequence for {{context}} finalized."
\'\'\'
    return _write(name, output_dir, content, "agent")
''',
        "forge_service.py": '''\
"""sigma_forge.forge_service — Background service scaffold generator."""
import os
from sigma_forge.forge_app import _write


def forge_service(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS background service scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f\'\'\'"""
{name} Background Service for SigmaOS
"""
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class {class_name}(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self._running = False

    def start_service(self):
        self._running = True
        return f"{name} Service ONLINE"

    def stop_service(self):
        self._running = False
        return f"{name} Service OFFLINE"

    def health_check(self):
        return "OK" if self._running else "INACTIVE"
\'\'\'
    return _write(name, output_dir, content, "service")
''',
        "forge_dispatcher.py": '''\
"""sigma_forge.forge_dispatcher — Routes forge commands to correct generators."""
from sigma_forge.forge_app import forge_app
from sigma_forge.forge_agent import forge_agent
from sigma_forge.forge_service import forge_service


def forge(template_type: str, name: str, output_dir: str = "userland/apps") -> str:
    """Dispatch to the appropriate forge generator based on template_type."""
    dispatch = {
        "app": forge_app,
        "agent": forge_agent,
        "service": forge_service,
    }
    fn = dispatch.get(template_type)
    if fn is None:
        return f"Error: Template type \'{template_type}\' unknown. Choose from: {list(dispatch.keys())}"
    return fn(name, output_dir)
''',
        "list_templates.py": '''\
"""sigma_forge.list_templates — Lists available forge templates."""


def list_templates() -> list:
    """Returns all available scaffold template types."""
    return ["app", "agent", "service"]
''',
    }

    imports = []
    for fname, content in files.items():
        safe_write(os.path.join(PKG, fname), content)
        imports.append(f"from .{fname[:-3]} import *  # noqa")

    # Shim for sigma_forge.py
    shim = '''\
"""
sigma_forge.py — SigmaOS Forge SDK (v1.0 Apex)
================================================
Backward-compat shim. Real implementation lives in sigma_forge/ package.
"""
import os
import sys
import argparse

from sigma_forge.forge_dispatcher import forge
from sigma_forge.list_templates import list_templates


class SigmaForge:
    """Thin facade over the modular sigma_forge package."""

    def forge(self, template_type, name, output_dir="userland/apps"):
        return forge(template_type, name, output_dir)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="SigmaForge: Sovereign SDK")
    parser.add_argument("cmd", choices=["new", "list"], help="Forge command")
    parser.add_argument("--type", choices=list_templates(), default="app", help="Type to forge")
    parser.add_argument("--name", help="Name of the forged object")

    args = parser.parse_args()
    f = SigmaForge()
    if args.cmd == "new" and args.name:
        print(f.forge(args.type, args.name))
    elif args.cmd == "list":
        print("Available Templates:", ", ".join(list_templates()))
    else:
        parser.print_help()
'''
    safe_write(os.path.join(ROOT, "sigma_forge.py"), shim, overwrite=True)
    make_init(PKG, imports)
    print("[OK] sigma_forge modularized.")


# ─────────────────────────────────────────────────────────────
# 4. Modularize mode_manager routines → mode_manager/routines/
# ─────────────────────────────────────────────────────────────
def modularize_mode_manager_routines():
    PKG = os.path.join(ROOT, "userland", "system_api", "mode_manager")
    RPKG = os.path.join(PKG, "routines")
    os.makedirs(RPKG, exist_ok=True)

    routines = {
        "notifications.py": """\
\"\"\"mode_manager.routines.notifications — Notification control routines.\"\"\"


def disable_notifications(phase: str = "") -> str:
    \"\"\"Simulates disabling system notifications.\"\"\"
    return "Notifications disabled."


def enable_notifications(phase: str = "") -> str:
    \"\"\"Simulates enabling system notifications.\"\"\"
    return "Notifications enabled."
""",
        "ui_theme.py": """\
\"\"\"mode_manager.routines.ui_theme — UI theme routines.\"\"\"


def set_gaming_ui_theme(phase: str = "") -> str:
    \"\"\"Simulates applying a gaming UI theme.\"\"\"
    return "Gaming UI theme applied."


def reset_ui_theme(phase: str = "") -> str:
    \"\"\"Simulates resetting the UI theme to default.\"\"\"
    return "UI theme reset to default."
""",
        "creative.py": """\
\"\"\"mode_manager.routines.creative — Creative mode routines.\"\"\"


def launch_creative_suite(phase: str = "") -> str:
    \"\"\"Simulates launching creative software suite.\"\"\"
    return "Creative suite launched."


def optimize_disk_cache(phase: str = "") -> str:
    \"\"\"Simulates optimizing disk cache for media editing.\"\"\"
    return "Disk cache optimized."


def flush_disk_cache(phase: str = "") -> str:
    \"\"\"Simulates flushing disk cache.\"\"\"
    return "Disk cache flushed."
""",
        "automation.py": """\
\"\"\"mode_manager.routines.automation — Automation agent routines.\"\"\"


def start_automation_agent(phase: str = "") -> str:
    \"\"\"Simulates starting an automation agent.\"\"\"
    return "Automation agent started."


def stop_automation_agent(phase: str = "") -> str:
    \"\"\"Simulates stopping an automation agent.\"\"\"
    return "Automation agent stopped."


def isolate_network_traffic(phase: str = "") -> str:
    \"\"\"Simulates isolating network traffic for automation processes.\"\"\"
    return "Network traffic isolated."


def restore_network_traffic(phase: str = "") -> str:
    \"\"\"Simulates restoring normal network traffic.\"\"\"
    return "Network traffic restored."
""",
        "mesh.py": """\
\"\"\"mode_manager.routines.mesh — Mesh & swarm routines.\"\"\"


def forge_global_mesh(kernel=None, phase: str = "") -> str:
    \"\"\"Engages the global automation mesh.\"\"\"
    if kernel and hasattr(kernel, "registry"):
        ar = kernel.registry.get("agentic_runtime")
        if ar and hasattr(ar, "forge_automation_mesh"):
            ar.forge_automation_mesh("sys.mode_shifted", ["notify_mesh", "optimize_ram"])
            return "Global Automation Mesh engaged (0ms Zapier Alternative)."
    return "Agentic Runtime offline."


def spawn_hyper_swarm(kernel=None, phase: str = "") -> str:
    \"\"\"Spawns a hyper-agent swarm.\"\"\"
    if kernel and hasattr(kernel, "registry"):
        ar = kernel.registry.get("agentic_runtime")
        if ar and hasattr(ar, "spawn_agent_swarm"):
            return ar.spawn_agent_swarm("Autonomous Mode Coordination", top_k_agents=5)
    return "Agentic Runtime offline."


def build_cognitive_dag(kernel=None, phase: str = "") -> str:
    \"\"\"Builds a sovereign cognitive DAG.\"\"\"
    if kernel and hasattr(kernel, "registry"):
        ar = kernel.registry.get("agentic_runtime")
        if ar and hasattr(ar, "build_sovereign_graph"):
            ar.build_sovereign_graph(
                "OS-Orchestrator", ["Listen", "Decide", "Act"],
                {"Listen": ["Decide"], "Decide": ["Act"]}
            )
            return "Sovereign Cognitive DAG built (LangGraph Alternative)."
    return "Agentic Runtime offline."


def cooldown_swarm(phase: str = "") -> str:
    \"\"\"Cools down the agentic swarm.\"\"\"
    return "Agentic Swarm compute cooled. Matrix returning to standby."
""",
        "ads.py": """\
\"\"\"mode_manager.routines.ads — Ad blocker & shopping routines.\"\"\"


def activate_ad_blocker(phase: str = "") -> str:
    \"\"\"Simulates activating a system-wide ad blocker.\"\"\"
    return "Ad blocker activated."


def open_shopping_browser(phase: str = "") -> str:
    \"\"\"Simulates opening a specialized shopping browser.\"\"\"
    return "Shopping browser opened."


def deactivate_ad_blocker(phase: str = "") -> str:
    \"\"\"Simulates deactivating a system-wide ad blocker.\"\"\"
    return "Ad blocker deactivated."
""",
        "display.py": """\
\"\"\"mode_manager.routines.display — Display control routines.\"\"\"


def monitor_cpu_temp(phase: str = "") -> str:
    \"\"\"Simulates starting CPU temperature monitoring.\"\"\"
    return "CPU temperature monitoring started."


def dim_display(phase: str = "") -> str:
    \"\"\"Simulates dimming the display.\"\"\"
    return "Display dimmed."


def restore_display(phase: str = "") -> str:
    \"\"\"Simulates restoring display brightness.\"\"\"
    return "Display brightness restored."


def calibrate_display(phase: str = "") -> str:
    \"\"\"Simulates display calibration.\"\"\"
    return "Display calibrated."


def reset_display_calibration(phase: str = "") -> str:
    \"\"\"Simulates resetting display calibration.\"\"\"
    return "Display calibration reset."
""",
        "animations.py": """\
\"\"\"mode_manager.routines.animations — Animation control routines.\"\"\"


def disable_animations(phase: str = "") -> str:
    \"\"\"Simulates disabling UI animations.\"\"\"
    return "UI animations disabled."


def enable_animations(phase: str = "") -> str:
    \"\"\"Simulates enabling UI animations.\"\"\"
    return "UI animations enabled."
""",
        "console.py": """\
\"\"\"mode_manager.routines.console — Console switching routines.\"\"\"


def switch_to_text_console(phase: str = "") -> str:
    \"\"\"Simulates switching to a text-only console.\"\"\"
    return "Switched to text console."


def kill_gui_processes(phase: str = "") -> str:
    \"\"\"Simulates killing GUI-related processes.\"\"\"
    return "GUI processes terminated."


def start_gui_processes(phase: str = "") -> str:
    \"\"\"Simulates starting GUI-related processes.\"\"\"
    return "GUI processes started."


def switch_to_graphical_console(phase: str = "") -> str:
    \"\"\"Simulates switching to a graphical console.\"\"\"
    return "Switched to graphical console."
""",
        "ai_frameworks.py": """\
\"\"\"mode_manager.routines.ai_frameworks — AI/ML framework routines.\"\"\"


def load_ai_frameworks(phase: str = "") -> str:
    \"\"\"Simulates loading AI/ML frameworks.\"\"\"
    return "AI frameworks loaded."


def allocate_vram(phase: str = "") -> str:
    \"\"\"Simulates allocating dedicated VRAM.\"\"\"
    return "VRAM allocated."


def unload_ai_frameworks(phase: str = "") -> str:
    \"\"\"Simulates unloading AI/ML frameworks.\"\"\"
    return "AI frameworks unloaded."


def deallocate_vram(phase: str = "") -> str:
    \"\"\"Simulates deallocating VRAM.\"\"\"
    return "VRAM deallocated."


def activate_intelligence_suite(kernel=None, phase: str = "") -> str:
    \"\"\"USP: Hydrates professional intelligence engines for Data/AI roles.\"\"\"
    engines = []
    if kernel:
        for attr in ("viz_engine", "ml_engine", "genai_lab", "insights_engine", "sql_forge", "hypertune"):
            if getattr(kernel, attr, None):
                engines.append(attr)
    if engines:
        return f"Intelligence Suite Active: {', '.join(engines)} hydrated."
    return "Intelligence Suite: Engines offline or not found in registry."
""",
        "data_lakes.py": """\
\"\"\"mode_manager.routines.data_lakes — Data lake & Jupyter routines.\"\"\"


def mount_data_lakes(phase: str = "") -> str:
    \"\"\"Simulates mounting data lake storage.\"\"\"
    return "Data lakes mounted."


def start_jupyter_lab(phase: str = "") -> str:
    \"\"\"Simulates starting Jupyter Lab.\"\"\"
    return "Jupyter Lab started."


def unmount_data_lakes(phase: str = "") -> str:
    \"\"\"Simulates unmounting data lake storage.\"\"\"
    return "Data lakes unmounted."
""",
        "vpn.py": """\
\"\"\"mode_manager.routines.vpn — VPN control routines.\"\"\"


def activate_vpn(phase: str = "") -> str:
    \"\"\"Simulates activating VPN.\"\"\"
    return "VPN activated."


def deactivate_vpn(phase: str = "") -> str:
    \"\"\"Simulates deactivating VPN.\"\"\"
    return "VPN deactivated."
""",
        "legal.py": """\
\"\"\"mode_manager.routines.legal — Legal suite routines.\"\"\"


def launch_legal_suite(phase: str = "") -> str:
    \"\"\"Simulates launching legal research software.\"\"\"
    return "Legal suite launched."
""",
        "dev_environment.py": """\
\"\"\"mode_manager.routines.dev_environment — Developer environment routines.\"\"\"


def start_dev_environment(phase: str = "") -> str:
    \"\"\"Simulates starting development environment (IDE, Docker).\"\"\"
    return "Development environment started."


def enable_code_completion(phase: str = "") -> str:
    \"\"\"Simulates enabling advanced code completion.\"\"\"
    return "Code completion enabled."


def stop_dev_environment(phase: str = "") -> str:
    \"\"\"Simulates stopping development environment.\"\"\"
    return "Development environment stopped."
""",
        "presentation.py": """\
\"\"\"mode_manager.routines.presentation — Presentation mode routines.\"\"\"


def mute_system_sounds(phase: str = "") -> str:
    \"\"\"Simulates muting system sounds.\"\"\"
    return "System sounds muted."


def start_presentation_software(phase: str = "") -> str:
    \"\"\"Simulates starting presentation software.\"\"\"
    return "Presentation software started."


def unmute_system_sounds(phase: str = "") -> str:
    \"\"\"Simulates unmuting system sounds.\"\"\"
    return "System sounds unmuted."
""",
        "wifi.py": """\
\"\"\"mode_manager.routines.wifi — Wi-Fi control routines.\"\"\"


def disconnect_wifi(phase: str = "") -> str:
    \"\"\"Simulates disconnecting from Wi-Fi.\"\"\"
    return "Wi-Fi disconnected."


def enable_offline_sync(phase: str = "") -> str:
    \"\"\"Simulates enabling offline file synchronization.\"\"\"
    return "Offline sync enabled."


def connect_wifi(phase: str = "") -> str:
    \"\"\"Simulates connecting to Wi-Fi.\"\"\"
    return "Wi-Fi connected."
""",
        "diagnostics.py": """\
\"\"\"mode_manager.routines.diagnostics — System diagnostic routines.\"\"\"


def run_system_diagnostics(phase: str = "") -> str:
    \"\"\"Simulates running system diagnostics.\"\"\"
    return "System diagnostics running."


def isolate_network(phase: str = "") -> str:
    \"\"\"Simulates isolating the network for security.\"\"\"
    return "Network isolated."


def reboot_system(phase: str = "") -> str:
    \"\"\"Simulates initiating a system reboot.\"\"\"
    return "System reboot initiated."
""",
        "hyper_drive.py": """\
\"\"\"mode_manager.routines.hyper_drive — Hyper-Drive engagement routines.\"\"\"


def engage_hyper_drive(kernel=None, phase: str = "") -> str:
    \"\"\"USP: Engages the Hyper-Drive Quantum Optimizer.\"\"\"
    if kernel and hasattr(kernel, "registry"):
        hd = kernel.registry.get("hyper_drive")
        if hd and hasattr(hd, "execute_ai_debloat") and hasattr(hd, "trigger_precognitive_cache"):
            hd.execute_ai_debloat()
            hd.trigger_precognitive_cache("Optimizing for Apex performance.")
            return "Hyper-Drive engaged: AI De-bloat and Pre-cognitive cache active."
    return "Hyper-Drive module not found."


def activate_zen_latency(kernel=None, phase: str = "") -> str:
    \"\"\"USP: Activates Zen Latency mode for instant UI feedback.\"\"\"
    if kernel and hasattr(kernel, "registry"):
        hd = kernel.registry.get("hyper_drive")
        if hd and hasattr(hd, "engage_zen_latency_mode"):
            return hd.engage_zen_latency_mode()
    return "Hyper-Drive module not available for Zen Latency."


def disengage_hyper_drive(phase: str = "") -> str:
    \"\"\"Disengages Hyper-Drive optimizations.\"\"\"
    return "Hyper-Drive disengaged. Reverting to standard scheduling."
""",
        "security.py": """\
\"\"\"mode_manager.routines.security — Security & compliance routines.\"\"\"


def run_compliance_audit(kernel=None, phase: str = "") -> str:
    \"\"\"Runs a full compliance audit via the compliance module.\"\"\"
    if kernel and getattr(kernel, "compliance", None):
        return str(kernel.compliance.run_full_compliance_audit())
    return "Compliance Auditor offline."


def seal_all_vaults(kernel=None, phase: str = "") -> str:
    \"\"\"Seals all sovereign vaults.\"\"\"
    if kernel and getattr(kernel, "crypt_guard", None):
        return "All sovereign vaults sealed with SHA-512."
    return "CryptGuard offline."


def activate_ghost_mask(kernel=None, phase: str = "") -> str:
    \"\"\"Activates GhostChat anonymous mask.\"\"\"
    if kernel and getattr(kernel, "ghost_chat", None):
        return "GhostChat mask active. Anonymous peer routing enabled."
    return "GhostChat offline."


def scrub_recent_media(kernel=None, phase: str = "") -> str:
    \"\"\"Initiates forensic scrub on recent media assets.\"\"\"
    if kernel and getattr(kernel, "media_forge", None):
        return "MediaForge forensic scrub initiated on recent assets."
    return "MediaForge offline."


def unseal_standard_vaults(kernel=None, phase: str = "") -> str:
    \"\"\"Restores standard vault access.\"\"\"
    if kernel and hasattr(kernel, "crypt_guard") and kernel.crypt_guard:
        return "Standard vaults unsealed. Access restored to normal privilege level."
    return "Vaults unsealed (CryptGuard offline — fallback mode)."
""",
        "focus.py": """\
\"\"\"mode_manager.routines.focus — Focus timer routines.\"\"\"


def start_focus_timer(phase: str = "") -> str:
    \"\"\"Starts a Pomodoro-style focus timer (25 min work / 5 min break).\"\"\"
    return "Focus Timer ACTIVE: 25-minute Pomodoro session started. Distractions blocked."


def stop_focus_timer(phase: str = "") -> str:
    \"\"\"Stops the active focus timer.\"\"\"
    return "Focus Timer STOPPED. All sessions logged. Distraction control lifted."
""",
        "design.py": """\
\"\"\"mode_manager.routines.design — Design software routines.\"\"\"


def launch_design_software(phase: str = "") -> str:
    \"\"\"Simulates launching design software.\"\"\"
    return "Design software launched."
""",
        "bi_dashboard.py": """\
\"\"\"mode_manager.routines.bi_dashboard — Business Intelligence dashboard routine.\"\"\"


def launch_bi_dashboard(phase: str = "") -> str:
    \"\"\"Simulates launching the SigmaOS Strategic BI Dashboard.\"\"\"
    return "Strategic BI Dashboard active. Real-time ROI and Market Trends visible."
""",
        "log_mode_change.py": """\
\"\"\"mode_manager.routines.log_mode_change — Mode-change logger.\"\"\"
import time


def log_mode_change(current_mode: str = "", phase: str = "") -> str:
    \"\"\"Logs the mode change event.\"\"\"
    return f"System log: Mode change {phase} for {current_mode} at {time.time()}."
""",
    }

    rimports = []
    for fname, content in routines.items():
        safe_write(os.path.join(RPKG, fname), content)
        rimports.append(f"from .{fname[:-3]} import *  # noqa")

    make_init(RPKG, rimports)

    # Write mode_manager package __init__
    pkg_init = '''\
"""mode_manager — SigmaOS Mode Manager package."""
from .mode_manager_core import SigmaModeManager  # noqa: F401
'''
    safe_write(os.path.join(PKG, "__init__.py"), pkg_init, overwrite=True)
    print("[OK] mode_manager routines modularized.")


# ─────────────────────────────────────────────────────────────
# 5. Run all modularizers
# ─────────────────────────────────────────────────────────────
if __name__ == "__main__":
    print("=" * 60)
    print("SigmaOS Deep Modularizer v2.0")
    print("=" * 60)

    modularize_fluid_ui()
    modularize_omni_automator()
    modularize_sigma_forge()
    modularize_mode_manager_routines()

    print("\n" + "=" * 60)
    print("All modularization complete!")
    print("=" * 60)
