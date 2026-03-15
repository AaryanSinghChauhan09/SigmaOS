"""
Auto-split from sigma_gui\sigmagui\_launch_app.py — SigmaGUI._launch_app
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
    def _launch_app(self, app_id):
        """Universal Sovereign App Launcher — Zero-Trust Isolated Process Runner."""
        import subprocess as _sp
        app_map = {'sigma.dev.codeforge': 'userland/apps/codeforge.py', 'sigma.dev.indent_flow': 'userland/apps/indent_flow.py', 'sigma.dev.bash': 'userland/apps/bash.py', 'sigma.media.aurapaint': 'userland/apps/aurapaint.py', 'sigma.media.pulseplay': 'userland/apps/pulseplayer.py', 'sigma.sys.sentinel': 'userland/apps/sentinel.py', 'sigma.sys.shield': 'userland/apps/shield.py', 'sigma.sys.titan_capture': 'userland/apps/titan_capture.py', 'sigma.prod.writer': 'userland/apps/writer.py', 'sigma.prod.pdf_forge': 'userland/apps/pdf_forge.py', 'sigma.prod.text_cleaner': 'userland/apps/text_cleaner.py', 'sigma.prod.pure_text': 'userland/apps/text_cleaner.py', 'sigma.prod.excel_ai': 'userland/apps/excel_hub.py', 'sigma.prod.project_flow': 'userland/apps/project_flow.py', 'sigma.prod.board_hub': 'userland/apps/board_hub.py', 'sigma.sys.welcome': 'userland/apps/welcome_guide.py', 'sigma.comm.omnibrowser': 'userland/apps/omnibrowser.py', 'sigma.comm.meshtalk': 'userland/apps/meshtalk.py', 'sigma.ai.antigravity': 'userland/apps/sigma_antigravity.py', 'sigma.ai.nexus_ai': 'userland/apps/nexus_ai.py', 'sigma.ai.prompt_o_matic': 'userland/apps/prompt_o_matic.py', 'sigma.ai.ag_finder': 'userland/apps/ag_finder.py', 'sigma.ai.email_disco': 'userland/apps/email_disco.py', 'sigma.game.g01': 'userland/apps/chess.py', 'sigma.game.g02': 'userland/apps/ludo.py', 'sigma.game.g21': 'userland/apps/jigsaw_puzzle.py', 'sigma.game.g22': 'userland/apps/spot_it.py', 'sigma.game.g23': 'userland/apps/shell_game.py', 'sigma.game.chess': 'userland/apps/chess.py', 'sigma.game.ludo': 'userland/apps/ludo.py'}
        self._notify('Sigma Launcher', f'Launching {app_id}…', 'OK')
        try:
            if app_id in app_map:
                script = os.path.join(_ROOT, app_map[app_id])
                if os.path.exists(script):
                    flags = _sp.CREATE_NEW_CONSOLE if os.name == 'nt' else 0
                    _sp.Popen([sys.executable, script], cwd=_ROOT, creationflags=flags)
                    self._log_voice(f"Sovereign process '{app_id}' isolated and running.")
                else:
                    self._notify('Launcher Error', f'Binary not found: {app_map[app_id]}', 'ERR')
                    self._log_voice(f'ERROR: App file missing — {app_map[app_id]}')
            else:
                page_map = {'sigma.ui.dashboard': 'dashboard', 'sigma.ui.store': 'store', 'sigma.ui.terminal': 'terminal', 'sigma.ui.automation': 'automation'}
                if app_id in page_map:
                    self._show_page(page_map[app_id])
                else:
                    self._notify('Launcher', f"Module '{app_id}' handled natively.", 'INFO')
        except Exception as e:
            self._notify('KERNEL FAULT', f'Launch failure: {str(e)}', 'ERR')
            self._log_voice(f'CRASH: {app_id} — {str(e)}')
