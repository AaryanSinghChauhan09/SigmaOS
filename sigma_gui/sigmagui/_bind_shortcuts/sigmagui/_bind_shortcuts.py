"""
Auto-split from sigma_gui\sigmagui\_bind_shortcuts.py — SigmaGUI._bind_shortcuts
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
    def _bind_shortcuts(self):
        """Global Keyboard Orchestration (Microsoft Aligned Sovereign UX)."""
        self.bind_all('<Control-Shift-Escape>', lambda e: self._show_page('system_audit'))
        self.bind_all('<Alt-Tab>', lambda e: self._cycle_tabs(1))
        self.bind_all('<Alt-i>', lambda e: self._show_page('config_hub'))
        self.bind_all('<Alt-e>', lambda e: self._show_page('explorer'))
        self.bind_all('<Alt-s>', lambda e: self._show_page('search'))
        self.bind_all('<Alt-r>', lambda e: self._show_spotlight())
        self.bind_all('<Alt-l>', lambda e: self._lock_screen())
        self.bind_all('<Alt-a>', lambda e: self._show_page('automation_hub'))
        self.bind_all('<Alt-g>', lambda e: self._show_page('gaming_hub'))
        self.bind_all('<Alt-p>', lambda e: self._show_page('analytics_page'))
        self.bind_all('<Alt-v>', lambda e: self._notify('CLIPBOARD', 'Sovereign Clipboard History: No PII detected.', 'INFO'))
        self.bind_all('<Alt-w>', lambda e: self._show_page('intelligence_hub'))
        self.bind_all('<Alt-c>', lambda e: self._show_page('sovereign_chat'))
        self.bind('<Control-k>', lambda e: self._show_spotlight())
        self.bind('<Control-space>', lambda e: self._show_spotlight())
        self.bind('<Control-s>', lambda e: self._trigger_sync())
        self.bind('<Control-comma>', lambda e: self._show_page('config_hub'))
        self.bind('<F5>', lambda e: self._reboot())
        self.bind('<Alt-Key-1>', lambda e: self._apply_snap_layout('FLOATING'))
        self.bind('<Alt-Key-2>', lambda e: self._apply_snap_layout('TILING'))
        self.bind('<Alt-Key-3>', lambda e: self._apply_snap_layout('QUARTERS'))
        self.bind('<Alt-Key-4>', lambda e: self._apply_snap_layout('SIDEBAR'))
        self.bind('<Alt-Key-5>', lambda e: self._apply_snap_layout('PILLAR'))
        self.bind('<Control-Tab>', lambda e: self._cycle_tabs(1))
        self.bind('<Control-Shift-Tab>', lambda e: self._cycle_tabs(-1))
        self.bind_all('<F1>', lambda e: self._show_page('manual'))
        self.bind_all('<Alt-x>', lambda e: self._show_quick_link_menu())
        self.bind_all('<Alt-b>', lambda e: self._show_page('brain'))
        self.bind_all('<Control-Shift-L>', lambda e: self._toggle_bare_minimum())
        self.bind_all('<Shift-Escape>', lambda e: self._emergency_shutdown())
        for key in ['d', 'e', 'i', 's', 'r', 'l', 'a', 'g', 'p', 'v', 'w', 'c']:
            self.bind_all(f'<Super_L>-{key}', lambda e, k=key: self._handle_ms_shortcut(k))
            self.bind_all(f'<Super_R>-{key}', lambda e, k=key: self._handle_ms_shortcut(k))
