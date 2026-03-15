"""
Auto-split from sigma_gui\sigmagui\_build_mindmap_page.py — SigmaGUI._build_mindmap_page
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
    def _build_mindmap_page(self):
        p = tk.Frame(self._content, bg=PAL['bg'])
        self._pages['mindmap'] = p
        tk.Label(p, text='🗺️ Nice Mind (Logic Flowchart Studio)', font=FONT_TITLE, fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        split = tk.PanedWindow(p, orient='horizontal', bg=PAL['border'], sashwidth=4)
        split.pack(fill='both', expand=True)
        edit_fr = tk.Frame(split, bg=PAL['bg2'])
        split.add(edit_fr, minsize=300)
        tk.Label(edit_fr, text='Syntax: Mermaid graph TD or Indented text', bg=PAL['bg2'], fg=PAL['dim'], font=FONT_SMALL).pack(anchor='w', padx=5, pady=5)
        code = scrolledtext.ScrolledText(edit_fr, bg='#111111', fg=PAL['cyan'], font=FONT_MONO, insertbackground='white')
        code.pack(fill='both', expand=True, padx=5, pady=5)
        code.insert('1.0', 'graph TD\n  Start[Start Node]\n  Choice{Decision}\n  Action1[Do Logic A]\n  Action2[Do Logic B]')
        canvas_fr = tk.Frame(split, bg=PAL['bg'])
        split.add(canvas_fr, minsize=400)
        cvs = tk.Canvas(canvas_fr, bg='#111122', highlightthickness=0)
        cvs.pack(fill='both', expand=True, padx=5, pady=5)

        def _render_flow(event=None):
            cvs.delete('all')
            txt = code.get('1.0', 'end').strip()
            lines = [l.strip() for l in txt.split('\n') if l.strip()]
            y_start = 50
            try:
                w = int(cvs.winfo_width())
            except:
                w = 800
            x_start = max(w // 2 - 100, 150)
            for i, line in enumerate(lines):
                if line.startswith('graph'):
                    continue
                parts = line.split('[')
                label = parts[1].split(']')[0] if len(parts) > 1 else line
                is_decision = '{' in line
                if is_decision:
                    label = line.split('{')[1].split('}')[0]
                    cvs.create_polygon(x_start + 100, y_start, x_start + 200, y_start + 25, x_start + 100, y_start + 50, x_start, y_start + 25, fill=PAL['card'], outline=PAL['gold'])
                    cvs.create_text(x_start + 100, y_start + 25, text=label, fill='white', font=FONT_MED)
                else:
                    cvs.create_rectangle(x_start, y_start, x_start + 200, y_start + 50, fill=PAL['card'], outline=PAL['cyan'])
                    cvs.create_text(x_start + 100, y_start + 25, text=label, fill='white', font=FONT_MED)
                if i < len(lines) - 1:
                    cvs.create_line(x_start + 100, y_start + 50, x_start + 100, y_start + 90, fill=PAL['dim'], arrow='last', width=2)
                y_start = y_start + 90
        code.bind('<KeyRelease>', _render_flow)
        cvs.bind('<Configure>', _render_flow)
        p.after(200, _render_flow)
