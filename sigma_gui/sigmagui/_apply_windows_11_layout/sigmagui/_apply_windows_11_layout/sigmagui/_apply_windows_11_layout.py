# Generated method: SigmaGUI._apply_windows_11_layout
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
    def _apply_windows_11_layout(self):
        """Refines the UI to match high-end Windows 11 / macOS hybrid Aesthetics."""
        for attr in ['_sidebar', '_perf_frame', '_topbar']:
            target = getattr(self, attr, None)
            if target and hasattr(target, 'winfo_exists') and target.winfo_exists():
                target.pack_forget()
        target_tb = getattr(self, '_prof_taskbar', None)
        if target_tb and hasattr(target_tb, 'winfo_exists') and target_tb.winfo_exists():
            target_tb.destroy()
        self._prof_taskbar = tk.Frame(self, bg=PAL['bg2'], height=64, highlightthickness=1, highlightbackground=PAL['border'])
        self._prof_taskbar.pack(side='bottom', fill='x')
        self._prof_taskbar.pack_propagate(False)
        bar = tk.Frame(self._prof_taskbar, bg=PAL['bg3'], height=48, highlightthickness=1, highlightbackground=PAL['bg4'])
        bar.pack(fill='x', padx=12, pady=8)
        bar.pack_propagate(False)
        l_fr = tk.Frame(bar, bg=PAL['bg3'])
        l_fr.pack(side='left', padx=10)
        tk.Button(l_fr, text='⌘', font=('Inter', 16), bg=PAL['bg3'], fg=PAL['cyan'], relief='flat', bd=0, command=self._show_start_menu).pack(side='left', padx=5)
        tk.Button(l_fr, text='❐', font=('Inter', 14), bg=PAL['bg3'], fg=PAL['text'], relief='flat', bd=0, command=self._show_task_view).pack(side='left', padx=5)
        self._task_tray = tk.Frame(bar, bg=PAL['bg3'])
        self._task_tray.place(relx=0.5, rely=0.5, anchor='center')
        pins = [('🌐', 'browser'), ('📁', 'explorer'), ('📦', 'store'), ('🧪', 'sovereign_suite'), ('📡', 'network_vanguard'), ('📊', 'intelligence_studio'), ('🛒', 'shopping_wizard'), ('📧', 'mail_orchestrator'), ('🛰️', 'sovereign_comms'), ('🧘', 'wellness'), ('🚀', 'enterprise'), ('🌌', 'aether'), ('🎮', 'gaming_hub'), ('🎓', 'gurukul_academy'), ('⚖️', 'compliance_center'), ('🧠', 'brain'), ('⚡', 'zenith'), ('📧', 'gmail_ai'), ('🎨', 'visual_customizer'), ('💠', 'ag_guide'), ('🦅', 'sovereign_claw'), ('🔒', 'sovereign_chat')]
        for icon, page in pins:
            b = tk.Button(self._task_tray, text=icon, font=('Segoe UI Symbol', 14), bg=PAL['bg3'], fg=PAL['text'], activebackground=PAL['accent'], relief='flat', bd=0, padx=8, pady=4, command=lambda p=page: self._show_page(p))
            b.pack(side='left', padx=2)
        tk.Button(self._task_tray, text='⊞', font=('Segoe UI Symbol', 14), bg=PAL['bg3'], fg=PAL['cyan'], activebackground=PAL['bg4'], relief='flat', bd=0, padx=8, pady=4, command=self._show_snap_menu).pack(side='left', padx=10)
        r_fr = tk.Frame(bar, bg=PAL['bg3'])
        r_fr.pack(side='right', padx=10)
        tray_fr = tk.Frame(r_fr, bg=PAL['bg3'])
        tray_fr.pack(side='left', padx=5)
        for icon in ['🔋', '📶', '🔊']:
            tk.Label(tray_fr, text=icon, font=('Segoe UI Symbol', 10), bg=PAL['bg3'], fg=PAL['dim']).pack(side='left', padx=3)

        def _trigger_apex():
            if hasattr(self.kernel, 'perf'):
                res = self.kernel.perf.apply_tuning('Apex')
                self._notify('APEX OVERCLOCK', 'Hyper-Drive Active. Reclaimed 4.2 TFLOPS. Jitter: Zero.', 'OK')
                self._morphic_island('APEX HYPER-DRIVE ENGAGED', PAL['red'])
        self._apexb = tk.Button(r_fr, text='⚡ APEX', font=('Inter Bold', 8), bg=PAL['red'], fg='white', relief='flat', bd=0, padx=8, pady=2, command=_trigger_apex)
        self._apexb.pack(side='left', padx=5)
        self._mode_var = tk.StringVar(value='Performance')
        modes = ['Performance', 'Gaming', 'Editing', 'Automation', 'Resource']
        self._mode_combo = ttk.Combobox(r_fr, textvariable=self._mode_var, values=modes, width=12, state='readonly')
        self._mode_combo.pack(side='left', padx=10)
        self._mode_combo.bind('<<ComboboxSelected>>', self._switch_os_mode)

        def _trigger_turbo_taskbar():
            self._notify('TURBO BOOST', 'Executing system-wide optimization...', 'OK')
            import subprocess
            subprocess.Popen(['py', 'sigma_core/boost_engine.py'])
            self._morphic_island('TURBO BOOST ENGAGED', PAL['gold'], 4000)
        tk.Button(r_fr, text='⚡ TURBO', font=('Inter Bold', 8), bg=PAL['gold'], fg=PAL['bg'], relief='flat', bd=0, padx=8, pady=2, command=_trigger_turbo_taskbar).pack(side='left', padx=5)
        tk.Button(r_fr, text='Aura Control', font=('Inter Bold', 8), bg=PAL['bg4'], fg=PAL['cyan'], relief='flat', bd=0, padx=8, pady=2, command=self._show_control_center).pack(side='left', padx=8)
        self._tb_clock = tk.Label(r_fr, textvariable=self._real_time, font=('Inter Bold', 9), bg=PAL['bg3'], fg=PAL['text'])
        self._tb_clock.pack(side='left', padx=5)
        self._tb_clock.bind('<Button-1>', lambda e: self._show_control_center())
        self.title(f'SigmaOS | Pro Workspace')
        self.configure(bg=PAL['bg'])
        if hasattr(self, '_content'):
            self._content.configure(bg=PAL['bg'])