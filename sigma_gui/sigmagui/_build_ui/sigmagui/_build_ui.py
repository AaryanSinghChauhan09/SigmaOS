"""
Auto-split from sigma_gui\sigmagui\_build_ui.py — SigmaGUI._build_ui
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
    def _build_ui(self):
        self._build_topbar()
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        self._main = tk.Frame(body, bg=PAL['bg'])
        self._main.pack(fill='both', expand=True)
        self._sidebar = self._build_sidebar(self._main)
        self._perf_frame = self._build_perf_status(self._main)
        self._content_container = tk.Frame(self._main, bg=PAL['bg'])
        self._content_container.pack(side='left', fill='both', expand=True, padx=(0, 8), pady=8)
        self._tab_ribbon = tk.Frame(self._content_container, bg=PAL['bg'], height=35)
        self._tab_ribbon.pack(fill='x', pady=(0, 5))
        self._tab_ribbon.pack_propagate(False)
        self._content = tk.Frame(self._content_container, bg=PAL['bg'])
        self._content.pack(fill='both', expand=True)
        self._island_var = tk.StringVar(value='SIGMA KERNEL: NOMINAL')
        self._island_fr = tk.Frame(self._content, bg=PAL['bg2'], height=28, padx=20, highlightthickness=1, highlightbackground=PAL['border'])
        self._island_fr.place(relx=0.5, y=14, anchor='n')
        self._island_lbl = tk.Label(self._island_fr, textvariable=self._island_var, font=('Inter Bold', 7), fg=PAL['cyan'], bg=PAL['bg2'])
        self._island_lbl.pack()

        def _island_expand(msg, color=PAL['cyan'], dur=3000):
            self._island_var.set(msg.upper())
            self._island_lbl.config(fg=color)
            self._island_fr.config(highlightbackground=color, height=34)
            self.after(dur, lambda: [self._island_var.set('SIGMA KERNEL: NOMINAL'), self._island_lbl.config(fg=PAL['cyan']), self._island_fr.config(highlightbackground=PAL['border'], height=28)])
        self._morphic_island = _island_expand
        self._stage_manager = tk.Frame(self._main, bg=PAL['bg'], width=80)
        self._stage_manager.pack(side='left', fill='y', padx=5)
        self._stage_manager.pack_forget()
        self._pages: dict[str, tk.Frame] = {}
        self._page_defs = {'dashboard': lambda: self._set_modular_page('dashboard', DashboardPage), 'browser': lambda: self._set_modular_page('browser', BrowserPage), 'explorer': lambda: self._set_modular_page('explorer', ExplorerPage), 'projects': lambda: self._set_modular_page('projects', ProjectCenterPage), 'law_pro': lambda: self._set_modular_page('law_pro', LawPage), 'buyhatke': lambda: self._set_modular_page('buyhatke', BuyhatkePage), 'search': lambda: self._set_modular_page('search', SearchPage), 'prompt_o_matic': lambda: self._set_modular_page('prompt_o_matic', PromptOMaticPage), 'routines_dash': lambda: self._set_modular_page('routines_dash', RoutinesDashPage), 'software_matrix': lambda: self._set_modular_page('software_matrix', SoftwareMatrixPage), 'nexus_ai': lambda: self._set_modular_page('nexus_ai', NexusPage), 'antigravity_hub': lambda: self._set_modular_page('antigravity_hub', AGGuidePage), 'brain': lambda: self._set_modular_page('brain', BrainPage), 'identity': lambda: self._set_modular_page('identity', IdentityPage), 'access': lambda: self._set_modular_page('access', AccessPage), 'network_warden': lambda: self._set_modular_page('network_warden', WardenPage), 'silo': lambda: self._set_modular_page('silo', SiloPage), 'intelligence_hub': lambda: self._set_modular_page('intelligence_hub', IntelligenceHubPage), 'terminal': lambda: self._set_modular_page('terminal', TerminalPage), 'automation_hub': lambda: self._set_modular_page('automation_hub', AutomationHubPage), 'ai_lifecycle': lambda: self._set_modular_page('ai_lifecycle', AILifecyclePage), 'zenith': lambda: self._set_modular_page('zenith', ZenithPage), 'config_hub': lambda: self._set_modular_page('config_hub', ConfigHubPage), 'gaming_hub': lambda: self._set_modular_page('gaming_hub', ArcadePage), 'system_audit': lambda: self._set_modular_page('system_audit', AuditViewPage), 'openroutines_hub': lambda: self._set_modular_page('openroutines_hub', OpenRoutinesPage), 'governor': lambda: self._set_modular_page('governor', GovernorPage), 'ag_physics': lambda: self._set_modular_page('ag_physics', AGPhysicsPage), 'visual_customizer': lambda: self._set_modular_page('visual_customizer', CustomizerPage), 'gmail_ai': lambda: self._set_modular_page('gmail_ai', GmailAIPage), 'aether': lambda: self._set_modular_page('aether', AetherPage), 'sovereign_suite': lambda: self._set_modular_page('sovereign_suite', SovereignLabPage), 'network_vanguard': lambda: self._set_modular_page('network_vanguard', VanguardPage), 'intelligence_studio': lambda: self._set_modular_page('intelligence_studio', IntelligenceHubPage), 'gurukul_academy': lambda: self._set_modular_page('gurukul_academy', UnivHubPage), 'compliance_center': self._build_compliance_center_page, 'mission_control': lambda: self._set_modular_page('mission_control', MissionControlPage), 'advanced_calculator': lambda: self._set_modular_page('advanced_calculator', AdvancedCalculatorPage), 'unit_converter': lambda: self._set_modular_page('unit_converter', UnitConverterPage), 'data_analyzer': lambda: self._set_modular_page('data_analyzer', DataAnalyzerPage), 'chemistry_lab': lambda: self._set_modular_page('chemistry_lab', ChemistryLabPage), 'cipher_studio': lambda: self._set_modular_page('cipher_studio', CipherStudioPage), 'ncert_simulator': lambda: self._set_modular_page('ncert_simulator', NcertSimulatorPage), 'ncert_calc': lambda: self._set_modular_page('ncert_calc', NcertCalcPage), 'diksha_vlab': lambda: self._set_modular_page('diksha_vlab', DikshaVLabPage), 'katbook_reader': lambda: self._set_modular_page('katbook_reader', KatbookReaderPage), 'time_tracker': lambda: self._set_modular_page('time_tracker', TimeTrackerPage), 'univ_hub': lambda: self._set_modular_page('univ_hub', UnivHubPage), 'reports': lambda: self._set_modular_page('reports', AnalyticsPage), 'shopping_wizard': lambda: self._set_modular_page('shopping_wizard', ShoppingWizardPage), 'mail_orchestrator': lambda: self._set_modular_page('mail_orchestrator', MailOrchestratorPage), 'sovereign_comms': lambda: self._set_modular_page('sovereign_comms', SovereignCommsPage), 'wellness': lambda: self._set_modular_page('wellness', SovereignWellnessPage), 'enterprise': lambda: self._set_modular_page('enterprise', EnterpriseHubPage), 'linux_parity': lambda: self._set_modular_page('linux_parity', LinuxParityPage), 'store': lambda: self._set_modular_page('store', StorePage), 'ag_guide': lambda: self._set_modular_page('ag_guide', AGGuidePage), 'aether': lambda: self._set_modular_page('aether', AetherOrchPage), 'apex': lambda: self._set_modular_page('apex', ApexPage), 'nexus': lambda: self._set_modular_page('nexus', NexusPage), 'writesense': lambda: self._set_modular_page('writesense', WritesensePage), 'flow': lambda: self._set_modular_page('flow', FlowPage), 'sovereign_claw': lambda: self._set_modular_page('sovereign_claw', ClawPage), 'sovereign_chat': lambda: self._set_modular_page('sovereign_chat', SigmaChatPage)}
        self.after(2000, self._vbox_check)
        self._build_dashboard()
        self._show_page('dashboard')
        self._build_alzheimer_page()
        self._build_mindmap_page()
        self._show_page('dashboard')
        self._apply_windows_11_layout()
        self._start_perf_engine()
