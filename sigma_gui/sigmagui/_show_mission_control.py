"""
Auto-split from sigma_gui.py — SigmaGUI._show_mission_control
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
    def _show_mission_control(self):
        """Ultra-High Fidelity Mission Control (Visual Overview)."""
        if hasattr(self, '_mc_popup') and self._mc_popup.winfo_exists():
            self._mc_popup.destroy()
            return
        self._mc_popup = tk.Toplevel(self)
        self._mc_popup.attributes('-fullscreen', True)
        self._mc_popup.attributes('-alpha', 0.96)
        self._mc_popup.configure(bg='#050510')
        tk.Label(self._mc_popup, text='Mission Control', font=('Inter Bold', 32), fg=PAL['text'], bg='#050510').pack(pady=40)
        grid = tk.Frame(self._mc_popup, bg='#050510')
        grid.pack(expand=True)
        page_list = list(self._page_defs.keys())
        for i, key in enumerate(page_list):
            r, c = (i // 4, i % 4)
            if i >= 12:
                break
            c_fr = tk.Frame(grid, bg='#050510', padx=15, pady=15)
            c_fr.grid(row=r, column=c)
            p_card = self._card(c_fr, title=key.upper(), glass=True, padx=40, pady=30)
            p_card.master.config(highlightbackground=PAL['accent'] if self._active_tab.get() == key else PAL['bg4'])
            icon = '🌐' if 'browser' in key else '📁' if 'explorer' in key else '🧠' if 'brain' in key else '🏔️'
            tk.Label(p_card, text=icon, font=('Segoe UI Symbol', 48), bg=PAL['bg2']).pack(pady=10)
            tk.Label(p_card, text=f'Workspace: {key}', font=FONT_MED, fg=PAL['dim'], bg=PAL['bg2']).pack()

            def _switch(k=key):
                self._show_page(k)
                self._mc_popup.destroy()
            p_card.bind('<Button-1>', lambda e, k=key: _switch(k))
            for child in p_card.winfo_children():
                child.bind('<Button-1>', lambda e, k=key: _switch(k))
        tk.Label(self._mc_popup, text='Press ESC or Click Space to Exit', font=FONT_SMALL, fg=PAL['dim'], bg='#050510').pack(side='bottom', pady=40)
        self._mc_popup.bind('<Escape>', lambda e: self._mc_popup.destroy())
        self._mc_popup.bind('<Button-1>', lambda e: self._mc_popup.destroy() if e.widget == self._mc_popup else None)
