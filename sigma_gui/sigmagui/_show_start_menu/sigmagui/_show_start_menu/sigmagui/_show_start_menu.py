# Generated method: SigmaGUI._show_start_menu
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
    def _show_start_menu(self):
        """Ultra-Premium Popover Start Menu with dynamic reveal."""
        if hasattr(self, '_start_popup') and self._start_popup.winfo_exists():
            self._start_popup.destroy()
            return
        self._start_popup = tk.Toplevel(self)
        self._start_popup.overrideredirect(True)
        self._start_popup.configure(bg=PAL['bg2'])
        w, h = (640, 520)
        x = self.winfo_x() + self.winfo_width() // 2 - w // 2
        y_final = self.winfo_y() + self.winfo_height() - h - 70
        y_start = y_final + 50
        self._start_popup.geometry(f'{w}x{h}+{x}+{y_start}')
        self._start_popup.attributes('-alpha', 0.0)

        def fade(alpha, pos_y):
            if alpha < 1.0:
                self._start_popup.attributes('-alpha', alpha)
                self._start_popup.geometry(f'{w}x{h}+{x}+{int(pos_y)}')
                self.after(10, lambda: fade(alpha + 0.1, pos_y - 2))
            else:
                self._start_popup.attributes('-alpha', 1.0)
                self._start_popup.geometry(f'{w}x{h}+{x}+{y_final}')
        fade(0.1, y_start)
        main = tk.Frame(self._start_popup, bg=PAL['bg2'], highlightthickness=1, highlightbackground=PAL['border'])
        main.pack(fill='both', expand=True)
        search_fr = tk.Frame(main, bg=PAL['bg3'], padx=20, pady=15)
        search_fr.pack(fill='x')
        s_box = tk.Frame(search_fr, bg=PAL['bg4'], padx=15, pady=8)
        s_box.pack(fill='x')
        tk.Label(s_box, text='🔍', bg=PAL['bg4'], fg=PAL['dim']).pack(side='left')
        s_entry = tk.Entry(s_box, bg=PAL['bg4'], fg=PAL['text'], insertbackground=PAL['cyan'], font=FONT_MED, relief='flat', borderwidth=0)
        s_entry.pack(side='left', fill='x', expand=True, padx=10)
        s_entry.insert(0, 'Search for apps, settings, or AI help...')
        s_entry.focus_set()
        grid_fr = tk.Frame(main, bg=PAL['bg2'], padx=30, pady=20)
        grid_fr.pack(fill='both', expand=True)
        pinned_apps = [('🌐', 'Browser', 'browser'), ('📁', 'Explorer', 'explorer'), ('📦', 'App Store', 'store'), ('🧪', 'Lab', 'lab'), ('🧠', 'Aether AI', 'brain'), ('📊', 'Data Studio', 'ds_studio'), ('🦾', 'Forge', 'forge'), ('🪐', 'Aura Mesh', 'mesh'), ('💻', 'Terminal', 'terminal'), ('🔐', 'Vault', 'secrets_hub'), ('📱', 'Mirror', 'phone_mirror'), ('🧬', 'Nexus', 'nexus')]
        for i, (icon, name, page) in enumerate(pinned_apps):
            c, r = (i % 4, i // 4)
            btn = tk.Frame(grid_fr, bg=PAL['bg2'], width=130, height=90)
            btn.grid(row=r, column=c, padx=5, pady=5)
            btn.pack_propagate(False)
            tk.Label(btn, text=icon, font=('Segoe UI Symbol', 24), bg=PAL['bg2']).pack()
            tk.Label(btn, text=name, font=('Inter', 8), fg=PAL['text'], bg=PAL['bg2']).pack()

            def _hover(e, fr=btn):
                fr.config(bg=PAL['bg3'])

            def _leave(e, fr=btn):
                fr.config(bg=PAL['bg2'])

            def _click(e, p=page):
                [self._show_page(p), self._start_popup.destroy()]
            btn.bind('<Enter>', _hover)
            btn.bind('<Leave>', _leave)
            btn.bind('<Button-1>', _click)
        footer = tk.Frame(main, bg=PAL['bg3'], height=60, padx=20)
        footer.pack(fill='x', side='bottom')
        user_fr = tk.Frame(footer, bg=PAL['bg3'])
        user_fr.pack(side='left', pady=10)
        tk.Label(user_fr, text='👤', font=('Inter', 14), bg=PAL['bg3'], fg=PAL['accent']).pack(side='left')
        tk.Label(user_fr, text='Sovereign-User Sovereign', font=('Inter Bold', 9), bg=PAL['bg3'], fg=PAL['text']).pack(side='left', padx=10)
        tk.Button(footer, text='⏻', font=('Inter Bold', 14), bg=PAL['bg3'], fg=PAL['red'], relief='flat', bd=0, command=self.destroy).pack(side='right', pady=10)
        self._start_popup.bind('<FocusOut>', lambda e: self._start_popup.destroy())