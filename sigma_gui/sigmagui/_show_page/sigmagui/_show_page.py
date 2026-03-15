# Generated method: SigmaGUI._show_page
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
    def _show_page(self, key: str):
        """USP: Sovereign Multi-Tab Page Orchestrator (Apex v4)."""
        aura_map = {'dashboard': PAL['cyan'], 'network_vanguard': PAL['red'], 'sovereign_suite': PAL['teal'], 'visual_customizer': PAL['purple'], 'brain': PAL['accent2'], 'gmail_ai': PAL['orange'], 'intelligence_studio': PAL['accent'], 'gurukul_academy': PAL['gold'], 'shopping_wizard': PAL['green'], 'mail_orchestrator': PAL['blue'], 'sovereign_comms': PAL['teal'], 'wellness': PAL['gold']}
        guardian = self.kernel.registry.get('guardian')
        if guardian and guardian.is_child_mode():
            restricted = ['terminal', 'kernel_debug', 'system_audit', 'network_warden', 'virtualbox', 'nexus_ai', 'war_room', 'compliance_center', 'sovereign_lab', 'dev_forge', 'ai_lifecycle', 'zenith', 'network_vanguard', 'visual_customizer', 'sovereign_suite', 'intelligence_studio', 'mail_orchestrator', 'sovereign_comms', 'enterprise', 'reports', 'automation_hub', 'brain', 'antigravity_hub', 'software_matrix', 'projects', 'config_hub', 'cipher_studio', 'advanced_calculator', 'unit_converter', 'data_analyzer', 'univ_hub']
            if key in restricted:
                self._notify('Guardian', f"Page '{key}' is restricted for your safety.", 'ERR')
                self.after(100, lambda: self._show_page('gurukul_academy'))
                return
        active_aura = aura_map.get(key, PAL['accent'])
        self._morphic_island(f'SPACE: {key.upper()}', active_aura, 1000)
        if key not in self._active_tabs:
            self._active_tabs.append(key)
            self._refresh_tab_ribbon()
        if key not in self._pages:
            if key in self._page_defs:
                try:
                    self._page_defs[key]()
                except Exception as e:
                    self._build_placeholder_page(key)
            else:
                self._build_placeholder_page(key)
        self._active_tab.set(key)
        self._refresh_tab_ribbon()
        target = self._pages[key]
        for k, p in self._pages.items():
            if p.winfo_exists() and k != key:
                p.pack_forget()
        target.pack(fill='both', expand=True)
        if hasattr(self, '_stage_manager') and self._stage_manager.winfo_exists():
            self._update_stage_manager(key)
        self._history.append(key)