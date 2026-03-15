# Generated method: SigmaGUI._show_task_view
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
    def _show_task_view(self):
        """USP: Sovereign Task View (Windows 11 / macOS Mission Control)."""
        if hasattr(self, '_task_view_pop') and self._task_view_pop.winfo_exists():
            self._task_view_pop.destroy()
            return
        self._task_view_pop = tk.Toplevel(self)
        self._task_view_pop.attributes('-fullscreen', True)
        self._task_view_pop.attributes('-topmost', True)
        self._task_view_pop.configure(bg='#0A0E14')
        self._task_view_pop.attributes('-alpha', 0.95)
        container = tk.Frame(self._task_view_pop, bg='#0A0E14')
        container.pack(expand=True, fill='both', padx=100, pady=100)
        tk.Label(container, text='ACTIVE WORKSPACES', font=('Outfit', 32, 'bold'), fg=PAL['cyan'], bg='#0A0E14').pack(pady=(0, 50))
        grid = tk.Frame(container, bg='#0A0E14')
        grid.pack(expand=True, fill='both')
        cols = 3
        for i, key in enumerate(self._active_tabs):
            card = tk.Frame(grid, bg=PAL['bg2'], bd=1, relief='flat', highlightthickness=1, highlightbackground=PAL['bg4'])
            card.grid(row=i // cols, column=i % cols, padx=20, pady=20, sticky='nsew')
            icon_map = {'dashboard': '🏠', 'browser': '🌐', 'explorer': '📁', 'brain': '🧠', 'zenith': '⚡'}
            icon = icon_map.get(key, '💠')
            tk.Label(card, text=icon, font=('Segoe UI Symbol', 48), fg=PAL['cyan'], bg=PAL['bg2']).pack(pady=(20, 10))
            tk.Label(card, text=key.upper(), font=('Inter Bold', 12), fg='white', bg=PAL['bg2']).pack(pady=5)
            btn = tk.Button(card, text='SWITCH TO SPACE', font=('Inter Bold', 8), bg=PAL['accent'], fg='white', relief='flat', padx=20, pady=10, command=lambda k=key: [self._show_page(k), self._task_view_pop.destroy()])
            btn.pack(pady=20)
            card.bind('<Enter>', lambda e, c=card: c.config(highlightbackground=PAL['accent']))
            card.bind('<Leave>', lambda e, c=card: c.config(highlightbackground=PAL['bg4']))
        self._task_view_pop.bind('<Escape>', lambda e: self._task_view_pop.destroy())