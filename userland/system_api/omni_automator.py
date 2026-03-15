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
