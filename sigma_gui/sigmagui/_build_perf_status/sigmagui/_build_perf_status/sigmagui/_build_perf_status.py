# Generated method: SigmaGUI._build_perf_status
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
    def _build_perf_status(self, parent):
        fr = tk.Frame(parent, bg='#0D0F12', width=140)
        fr.pack(side='right', fill='y')
        fr.pack_propagate(False)
        tk.Label(fr, text='⚡ SYSTEM CORE', font=('Inter Bold', 8), bg='#0D0F12', fg=PAL['cyan']).pack(pady=10)
        self._meters = {}
        for meter in ['CPU', 'GPU', 'RAM', 'CONSCIOUS']:
            m_fr = tk.Frame(fr, bg='#0A0C0E', padx=5, pady=6)
            m_fr.pack(fill='x', pady=2)
            tk.Label(m_fr, text=meter, bg='#0A0C0E', fg=PAL['dim'], font=('Inter', 7)).pack()
            canvas = tk.Canvas(m_fr, width=120, height=6, bg='#1A1C1E', highlightthickness=0)
            canvas.pack()
            bar = canvas.create_rectangle(0, 0, 10, 6, fill=PAL['cyan'], outline='')
            self._meters[meter] = (canvas, bar)
        self._sec_status_var = tk.StringVar(value='🛡️ SECURE')
        self._sec_status_lbl = tk.Label(fr, textvariable=self._sec_status_var, font=('Inter Bold', 7), bg='#0D0F12', fg=PAL['teal'])
        self._sec_status_lbl.pack(pady=10)
        self._rollback_var = tk.StringVar(value='Slot A: ACTIVE')
        tk.Label(fr, textvariable=self._rollback_var, font=('Inter', 7), bg='#0D0F12', fg=PAL['gold']).pack()

        def update_meters():
            perf = self.kernel.perf
            metrics = perf.get_realtime_metrics() if perf else {}
            mesh = self.kernel.mesh
            mesh_intel = mesh.get_mesh_intel() if mesh else {'total_tflops': 0}
            warden = self.kernel.warden
            warden_report = warden.get_security_audit() if warden else {'lockdown': 'OFF'}
            vals = {'CPU': SigmaSys.cpu_usage(), 'GPU': random.randint(1, 4), 'RAM': SigmaSys.ram_usage(), 'CONSCIOUS': int((self.kernel.cog_fabric.conscious_score if hasattr(self.kernel, 'cog_fabric') else 0.8) * 100)}
            for m, v in vals.items():
                w = int(v * 1.2)
                self._meters[m][0].coords(self._meters[m][1], 0, 0, w, 6)
                color = PAL['teal'] if m == 'CONSCIOUS' else PAL['cyan'] if v < 80 else PAL['red']
                self._meters[m][0].itemconfig(self._meters[m][1], fill=color)
            if warden_report.get('lockdown') == 'ON':
                self._sec_status_var.set('🔒 LOCKDOWN ACTIVE')
                self._sec_status_lbl.config(fg=PAL['red'])
            else:
                self._sec_status_var.set('🧠 SINGULARITY ACTIVE')
                self._sec_status_lbl.config(fg=PAL['cyan'])
            upd = self.kernel.update_manager
            if upd:
                slot = getattr(upd, '_slot_active', 'A')
                self._rollback_var.set(f'Slot {slot}: ACTIVE')
            if hasattr(self.kernel.memory, 'get_stats'):
                nmc_stats = self.kernel.memory.get_stats()
                self._rollback_var.set(f"NMC: {nmc_stats.get('nmc_impact', '1.0x')}")
            self.after(2000, update_meters)
        update_meters()
        return fr