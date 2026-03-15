"""
Auto-split from sigma_gui.py — SigmaGUI._build_legal_pro_page
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
    def _build_legal_pro_page(self):
        p = tk.Frame(self._content, bg=PAL['bg'])
        self._pages['legal_pro'] = p
        tk.Label(p, text='⚖️  Sigma Lawyer Pro: Legal Operating System', font=FONT_LOGO, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w', pady=(0, 8))
        body = tk.Frame(p, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=460)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        draft_c = self._card(l_fr, '📄 Contract Forge (Automated Drafting)')
        draft_c.master.pack(fill='x', pady=5)
        ttk.Button(draft_c, text='Draft NDA', command=lambda: self._log_voice('Legal: Drafting Sovereign NDA for BNS_2023...')).pack(side='left', padx=5)
        ttk.Button(draft_c, text='Analyze Contract', command=lambda: self._log_voice(self.kernel.law.analyze_statute('Contract_Act_1872', 'Sec_10'))).pack(side='left', padx=5)
        search_c = self._card(l_fr, '🏛️ Sovereign Precedent Search')
        search_c.master.pack(fill='x', pady=5)
        ttk.Button(search_c, text='Search Case Law', command=lambda: self._log_voice(self.kernel.law.get_procedural_roadmap('Criminal Trial (BNSS)'))).pack(side='left', padx=5)
        law_sec_c = self._card(l_fr, '🛡️ Confidentiality & Live Compliance')
        law_sec_c.master.pack(fill='x', pady=5)
        ttk.Button(law_sec_c, text='Encrypted Client Portal', command=lambda: self._log_voice(self.kernel.law.encrypted_client_portal('CLIENT_77'))).pack(side='left', padx=5)
        ttk.Button(law_sec_c, text='Live MCA21 Audit', command=lambda: self._log_voice(self.kernel.law.automated_compliance_alert('SIGMA_CORP'))).pack(side='left', padx=5)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        console_c = self._card(r_fr, '📟 Lawyer Pro Matter Console')
        console_c.master.pack(fill='both', expand=True)
        self._legal_pro_log = self._console(console_c, height=25)
        self._legal_pro_log.pack(fill='both', expand=True)
