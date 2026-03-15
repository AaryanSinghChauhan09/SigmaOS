# Generated method: SigmaGUI._build_topbar
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
    def _build_topbar(self):
        self._topbar = tk.Frame(self, bg=PAL['bg2'], height=56)
        self._topbar.pack(fill='x')
        self._topbar.pack_propagate(False)
        bar = self._topbar
        start_fr = tk.Frame(bar, bg=PAL['bg2'])
        start_fr.pack(side='left', padx=(12, 0))
        ttk.Button(start_fr, text='σ', width=3, command=self._show_start_menu, name='start_btn').pack(side='left')
        ttk.Button(start_fr, text='🌐 Web OS', width=12, command=self._launch_web_os, name='webos_btn').pack(side='left', padx=(5, 0))
        self._logo_lbl = tk.Label(start_fr, textvariable=self._dashboard_title, font=FONT_LOGO, fg=PAL['cyan'], bg=PAL['bg2'])
        self._logo_lbl.pack(side='left', padx=8)
        intent_fr = tk.Frame(bar, bg=PAL['bg2'])
        intent_fr.pack(side='left', padx=12, fill='x', expand=True)
        self._intent_var = tk.StringVar(value="🔮 Type Intent (e.g., 'Setup Lawyer Workspace' or 'Audit Security')")
        self._intent_entry = ttk.Entry(intent_fr, textvariable=self._intent_var, name='intent_input')
        self._intent_entry.pack(side='left', fill='x', expand=True, padx=(0, 6))
        self._intent_entry.bind('<FocusIn>', lambda e: self._intent_var.set('') if 'Type Intent' in self._intent_var.get() else None)
        self._intent_entry.bind('<Return>', self._intent_exec)
        self._intent_entry.bind('<KeyRelease>', self._show_omni_suggest)
        ttk.Button(intent_fr, text='🚀 Execute', command=self._intent_exec, name='intent_exec_btn').pack(side='left')
        self._suggest_pop = None
        self._island = tk.Frame(self._topbar, bg=PAL['bg'], width=280, height=36)
        self._island.place(relx=0.5, rely=0.5, anchor='center')
        self._island.pack_propagate(False)
        self._island_lbl = tk.Label(self._island, text='🛡️ SOVEREIGN DEFENSE ACTIVE', font=('Segoe UI', 9, 'bold'), fg=PAL['cyan'], bg=PAL['bg'])
        self._island_lbl.pack(expand=True)