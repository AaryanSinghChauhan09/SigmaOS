"""
Auto-split from sigma_gui\sigmagui\_show_spotlight.py — SigmaGUI._show_spotlight
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
    def _show_spotlight(self):
        """Standard-Grade Command Palette (Raycast/Alfred Hybrid)."""
        if hasattr(self, '_spot') and self._spot.winfo_exists():
            self._spot.destroy()
            return
        self._spot = tk.Toplevel(self)
        self._spot.overrideredirect(True)
        self._spot.configure(bg=PAL['bg2'])
        w, h = (700, 450)
        x = self.winfo_x() + self.winfo_width() // 2 - w // 2
        y = self.winfo_y() + 150
        self._spot.geometry(f'{w}x{h}+{x}+{y}')
        self._spot.attributes('-alpha', 0.98)
        self._spot.attributes('-topmost', True)
        inner = tk.Frame(self._spot, bg=PAL['bg'], highlightthickness=1, highlightbackground=PAL['accent'])
        inner.pack(fill='both', expand=True)
        search_fr = tk.Frame(inner, bg=PAL['bg'], pady=15)
        search_fr.pack(fill='x')
        tk.Label(search_fr, text=' 🔮 ', font=('Inter', 24), bg=PAL['bg']).pack(side='left', padx=(15, 0))
        s_var = tk.StringVar()
        s_ent = tk.Entry(search_fr, textvariable=s_var, font=('Inter', 22), bg=PAL['bg'], fg='white', insertbackground=PAL['cyan'], relief='flat', borderwidth=0)
        s_ent.pack(side='left', fill='x', expand=True, padx=15)
        s_ent.focus_set()
        tk.Frame(inner, bg=PAL['border'], height=1).pack(fill='x')
        results_fr = tk.Frame(inner, bg=PAL['bg'])
        results_fr.pack(fill='both', expand=True, padx=10, pady=10)

        def _add_result(category, icon, label, page_key):
            row = tk.Frame(results_fr, bg=PAL['bg'], padx=10, pady=8)
            row.pack(fill='x')
            tk.Label(row, text=icon, font=('Segoe UI Symbol', 14), bg=PAL['bg'], fg=PAL['cyan']).pack(side='left', padx=(0, 15))
            txt_fr = tk.Frame(row, bg=PAL['bg'])
            txt_fr.pack(side='left')
            tk.Label(txt_fr, text=label, font=FONT_MED, fg=PAL['text'], bg=PAL['bg']).pack(anchor='w')
            tk.Label(txt_fr, text=category, font=('Inter', 7), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')

            def _hvr(e, r=row):
                r.config(bg=PAL['bg3'])
                [c.config(bg=PAL['bg3']) for c in r.winfo_children()]

            def _lve(e, r=row):
                r.config(bg=PAL['bg'])
                [c.config(bg=PAL['bg']) for c in r.winfo_children()]

            def _clk(e, p=page_key):
                [self._show_page(p), self._spot.destroy()]
            row.bind('<Enter>', _hvr)
            row.bind('<Leave>', _lve)
            row.bind('<Button-1>', _clk)
            for child in row.winfo_children():
                child.bind('<Button-1>', _clk)
        all_suggestions = [('SOVEREIGN HUB', '🏠', 'Sovereign Dashboard', 'dashboard'), ('SOVEREIGN HUB', '⚡', 'Antigravity Zenith', 'zenith'), ('VFS EXPLORER', '📁', 'File System Manager', 'explorer'), ('DEVELOPER', '💻', 'Sigma DevForge', 'dev_forge'), ('NETWORK', '🛡️', 'Network Warden', 'network_warden'), ('PRODUCTIVITY', '🏗️', 'Omni Workspaces', 'omni_work'), ('AI STUDIO', '🧠', 'Aether Brain Lab', 'brain'), ('KERNEL', '📟', 'Sovereign Terminal', 'terminal'), ('SECURITY', '🛡️', 'Sovereign Sanctuary', 'sanctuary'), ('COMPLIANCE', '⚖️', 'Humanity Core Auditor', 'compliance')]

        def _filter(e=None):
            for w in results_fr.winfo_children():
                w.destroy()
            q = s_var.get().lower()
            count: int = 0
            for cat, icon, lbl, p in all_suggestions:
                if not q or q in lbl.lower() or q in cat.lower() or (q in p):
                    _add_result(cat, icon, lbl, p)
                    count_val = int(count)
                    count = count_val + 1
                    if count >= 6:
                        break
        s_var.trace_add('write', lambda n, i, m: _filter())
        _filter()

        def _exec(e):
            q = s_var.get().lower()
            match = None
            for cat, icon, lbl, p in all_suggestions:
                if q and (q in lbl.lower() or q in p):
                    match = p
                    break
            if match:
                target = str(match)
                self._history.append(target)
                self._show_page(target)
            elif q in self._page_defs:
                self._show_page(q)
            else:
                self._intent_var.set(q)
                self._intent_exec()
            if hasattr(self, '_spot') and self._spot:
                try:
                    self._spot.destroy()
                except:
                    pass
        s_ent.bind('<Return>', _exec)
        s_ent.bind('<Escape>', lambda e: self._spot.destroy())
        footer = tk.Frame(inner, bg=PAL['bg2'], height=30)
        footer.pack(fill='x', side='bottom')
        tk.Label(footer, text='SEARCH OR TYPE INTENT • ↵ TO EXECUTE • ESC TO CANCEL', font=('Inter', 7), bg=PAL['bg2'], fg=PAL['dim']).pack(pady=5)
