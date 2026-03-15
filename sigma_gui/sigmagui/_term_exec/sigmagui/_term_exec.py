"""
Auto-split from sigma_gui\sigmagui\_term_exec.py — SigmaGUI._term_exec
"""

import sys
import os
import threading
import json
import time
import importlib
import random
from sigma_core import SigmaKernel, SigmaConfig
from sigma_projects import TaskStatus, Priority
from userland.system_api.sigma_std import SigmaSys, SigmaNetwork
from userland.system_api.sigma_games_engine import SigmaGamesEngine
from gui_pkg.styles import PAL, FONT_MONO, FONT_SMALL, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_LOGO
from gui_pkg.mixins import UIMixin
from gui_pkg.premium_ui import PremiumUIMixin
from gui_pkg.war_room import WarRoomPage
from gui_pkg.mission_control import MissionControlPage
from gui_pkg.cosmos_dash import CosmosDashPage
from gui_pkg.fabric import FabricPage
from gui_pkg.brain import BrainPage
from gui_pkg.zenith import ZenithPage
from gui_pkg.sovereign_lab import SovereignLabPage
from gui_pkg.kernel_debug import KernelDebugPage
from gui_pkg.automation_hub import AutomationHubPage
from gui_pkg.routines_dash import RoutinesDashPage
from gui_pkg.ag_physics import AGPhysicsPage
from gui_pkg.advanced_calculator import AdvancedCalculatorPage
from gui_pkg.unit_converter import UnitConverterPage
from gui_pkg.data_analyzer import DataAnalyzerPage
from gui_pkg.arcade import ArcadePage
from gui_pkg.aether_orch import AetherOrchPage
from gui_pkg.chemistry_lab import ChemistryLabPage
from gui_pkg.cipher_studio import CipherStudioPage
from gui_pkg.ncert_simulator import NcertSimulatorPage
from gui_pkg.ncert_calc import NcertCalcPage
from gui_pkg.diksha_vlab import DikshaVLabPage
from gui_pkg.katbook_reader import KatbookReaderPage
from gui_pkg.time_tracker import TimeTrackerPage
from gui_pkg.browser_page import BrowserPage
from gui_pkg.software_matrix import SoftwareMatrixPage
from gui_pkg.config_hub import ConfigHubPage
from gui_pkg.audit_view import AuditViewPage
from gui_pkg.analytics_page import AnalyticsPage
from gui_pkg.terminal_page import TerminalPage
from gui_pkg.univ_hub_page import UnivHubPage
from gui_pkg.shopping_wizard import ShoppingWizardPage
from gui_pkg.mail_orchestrator import MailOrchestratorPage
from gui_pkg.sovereign_comms import SovereignCommsPage
from gui_pkg.sovereign_wellness import SovereignWellnessPage
from gui_pkg.enterprise_hub import EnterpriseHubPage
from gui_pkg.ag_guide import AGGuidePage
from gui_pkg.gmail_ai import GmailAIPage
from gui_pkg.customizer import CustomizerPage
from gui_pkg.prompt_o_matic import PromptOMaticPage
from gui_pkg.store_page import StorePage
from gui_pkg.identity_page import IdentityPage
from gui_pkg.access_page import AccessPage
from gui_pkg.warden_page import WardenPage
from gui_pkg.linux_parity_page import LinuxParityPage
from gui_pkg.silo_page import SiloPage
from gui_pkg.intelligence_hub_page import IntelligenceHubPage
from gui_pkg.apex_page import ApexPage
from gui_pkg.nexus_page import NexusPage
from gui_pkg.writesense_page import WritesensePage
from gui_pkg.flow_page import FlowPage
from gui_pkg.vanguard_page import VanguardPage
from gui_pkg.ai_lifecycle_page import AILifecyclePage
from gui_pkg.governor_page import GovernorPage
from gui_pkg.search_page import SearchPage
from gui_pkg.explorer_page import ExplorerPage
from gui_pkg.project_center import ProjectCenterPage
from gui_pkg.law_page import LawPage
from gui_pkg.buyhatke_page import BuyhatkePage
from gui_pkg.dashboard_page import DashboardPage
from gui_pkg.aether_page import AetherPage
from gui_pkg.claw_page import ClawPage
from gui_pkg.openroutines_page import OpenRoutinesPage
from gui_pkg.chat_page import SigmaChatPage



class SigmaGUI:
    def _term_exec(self, event=None):
        raw = self._term_input.get().strip()
        if not raw:
            return
        self._term_history.append(raw)
        self._term_hist_idx = -1
        self._term_input.set('')
        prompt = '# ' if self._is_elevated.get() else 'σ > '
        self._log(self._term_out, f'{prompt}{raw}', 'WARN' if self._is_elevated.get() else 'INFO')
        parts = raw.split()
        cmd = parts[0].lower()

        def run():
            try:
                if cmd == 'help':
                    self._log(self._term_out, 'Apex Commands: fabric | automator | forge | mesh | ual | zenith\n  security | manual | health | events | call | clear', 'INFO')
                elif cmd == 'manual':
                    self._show_page('manual')
                    self._log(self._term_out, '  ✔ Opening User Manual...', 'OK')
                elif cmd == 'fabric':
                    res = self.kernel.fabric.execute_neural_prefetch('Work')
                    self._log(self._term_out, f'  ✔ {res}', 'OK')
                elif cmd == 'automator':
                    mid = self.kernel.automator.plan_mission('Test')
                    self._log(self._term_out, f'  ✔ Mission Staged: {mid}', 'OK')
                elif cmd == 'forge':
                    res = self.kernel.forge.process_document('local.pdf', 'Audit')
                    self._log(self._term_out, f'  ✔ Forge: {res}', 'OK')
                elif cmd == 'mesh':
                    res = self.kernel.mesh.broadcast_update_intent('v3')
                    self._log(self._term_out, f'  ✔ Mesh: {res}', 'OK')
                elif cmd == 'ual':
                    res = self.kernel.ual.bridge_app('test.exe')
                    self._log(self._term_out, f"  ✔ UAL: {res['Message']}", 'OK')
                elif cmd == 'security':
                    sec = self.kernel.security
                    if sec:
                        for r in [sec.secure_boot_verify(), sec.ebpf_proactive_monitoring()]:
                            self._log(self._term_out, f'  ✔ {r}', 'OK')
                elif cmd == 'health':
                    for m, s in self.kernel.registry.health_check().items():
                        self._log(self._term_out, f'  ✔ {m}: {s}', 'OK')
                elif cmd == 'events':
                    for e in self.kernel.bus.get_history(10):
                        self._log(self._term_out, f"  {e['event']}: {e['payload']}", 'INFO')
                elif cmd == 'call':
                    if len(parts) >= 3:
                        r = self.kernel.registry.call(parts[1], parts[2])
                        if isinstance(r, dict):
                            for k, v in r.items():
                                self._log(self._term_out, f'  {k}: {v}', 'INFO')
                        else:
                            self._log(self._term_out, f'  ✔ {r}', 'OK')
                elif cmd == 'clear':
                    self._term_out.configure(state='normal')
                    self._term_out.delete('1.0', 'end')
                    self._term_out.configure(state='disabled')
                elif cmd == 'zenith':
                    prompt = ' '.join(parts[1:])
                    if not prompt:
                        self._log(self._term_out, '  Usage: zenith <prompt>', 'WARN')
                    else:
                        self._log(self._term_out, f'  🚀 Dispatching Mission to Zenith: {prompt[:30]}...', 'INFO')
                        import urllib.request, urllib.parse, json
                        try:
                            data = urllib.parse.urlencode({'prompt': prompt, 'nodes': '["ChatGPT","Claude"]'}).encode()
                            req = urllib.request.Request('http://localhost:8001/api/dispatch', data=data)
                            with urllib.request.urlopen(req) as response:
                                res = json.loads(response.read().decode())
                                self._log(self._term_out, f"  ✔ Dispatched! Task ID: {res.get('task_id')}", 'OK')
                        except Exception as e:
                            self._log(self._term_out, f'  ✖ Connectivity Error: Is Zenith Kernel running? ({e})', 'ERR')
                else:
                    self._log(self._term_out, f"  Unknown: '{cmd}'. Type 'help'.", 'WARN')
            except Exception as exc:
                self._log(self._term_out, f'  Error: {exc}', 'ERR')
        threading.Thread(target=run, daemon=True).start()
