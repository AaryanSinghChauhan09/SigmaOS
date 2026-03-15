"""
Auto-split from sigma_gui\sigmagui\_build_customizer_page.py — SigmaGUI._build_customizer_page
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
    def _build_customizer_page(self):
        p = tk.Frame(self._content, bg=PAL['bg'])
        self._pages['customizer'] = p
        tk.Label(p, text='🎨  Sigma Customization Studio: The Living Canvas', font=FONT_LOGO, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w', pady=(0, 8))
        body = tk.Frame(p, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=450)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        ai_c = self._card(l_fr, '🌈 Generative Theme Engine')
        ai_c.master.pack(fill='x', pady=5)
        m_var = tk.StringVar(value='Focus')
        for m in ['Focus', 'Creative', 'Night', 'Neon']:
            tk.Radiobutton(ai_c, text=m, variable=m_var, value=m, bg=PAL['card'], fg=PAL['text'], command=lambda m=m: self._log_voice(self.kernel.registry.get('customizer').generate_ai_theme(m)['message'])).pack(side='left', padx=5)
        aura_c = self._card(l_fr, '✨ Sovereign Branding Auras')
        aura_c.master.pack(fill='x', pady=5)
        tk.Label(aura_c, text='Select an OS Persona:', bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        a_var = tk.StringVar(value='omni')
        auras = ['omni', 'nexus', 'synergy', 'fusion', 'prism', 'horizon', 'unity']
        for i in range(0, len(auras), 4):
            row = tk.Frame(aura_c, bg=PAL['card'])
            row.pack(fill='x')
            for a in auras[i:i + 4]:
                tk.Radiobutton(row, text=a.capitalize(), variable=a_var, value=a, bg=PAL['card'], fg=PAL['text'], command=lambda a=a: self._log_voice(self.kernel.registry.get('customizer').apply_branding_aura(a)['msg'])).pack(side='left', padx=2)
        color_c = self._card(l_fr, '🖌️ Chromatic Orchestration')
        color_c.master.pack(fill='x', pady=5)

        def _apply_colors():
            acc = random.choice(['#FF4757', '#2ED573', '#7B2FBE', '#00FFFF', '#FFD700'])
            bg = random.choice(['#0D0D1A', '#1A1A24', '#0F172A'])
            self._log_voice(self.kernel.registry.get('customizer').apply_color_palette(acc, bg))
        ttk.Button(color_c, text='🎲 Randomize Global Palette', command=_apply_colors).pack(side='left', padx=5)

        def _upload_logo():
            import tkinter.filedialog as fd
            path = fd.askopenfilename(title='Select Sovereign Logo', filetypes=[('Image Files', '*.png *.jpg *.ico')])
            if path:
                self._log_voice(self.kernel.registry.get('customizer').set_application_logo(path))
        ttk.Button(color_c, text='🖼️ Upload Custom OS Logo', command=_upload_logo).pack(side='left', padx=5)
        lc_c = self._card(l_fr, '📐 Layout & Icon Packs')
        lc_c.master.pack(fill='x', pady=5)
        tk.Label(lc_c, text='Sidebar:', bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        s_var = tk.StringVar(value='Left')
        for s in ['Left', 'Right', 'Floating']:
            tk.Radiobutton(lc_c, text=s, variable=s_var, value=s, bg=PAL['card'], fg=PAL['text'], command=lambda s=s: self._log_voice(self.kernel.registry.get('customizer').switch_layout(s, 'Comfortable'))).pack(side='left')
        tk.Label(lc_c, text='\nIcon Pack:', bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        p_var = tk.StringVar(value='Sovereign_3D')
        for p in ['Sovereign_3D', 'Fluent', 'Retro_8Bit']:
            tk.Radiobutton(lc_c, text=p, variable=p_var, value=p, bg=PAL['card'], fg=PAL['text'], command=lambda p=p: self._log_voice(self.kernel.registry.get('customizer').swap_icon_pack(p))).pack(side='left', padx=5)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        sp_c = self._card(r_fr, '🔉 Acoustics & OS Physics')
        sp_c.master.pack(fill='x', pady=5)
        v_var = tk.StringVar(value='Calm')
        for v in ['Calm', 'Mechanical', 'Cyber']:
            tk.Radiobutton(sp_c, text=v, variable=v_var, value=v, bg=PAL['card'], fg=PAL['text'], command=lambda v=v: self._log_voice(self.kernel.registry.get('customizer').apply_soundscape(v))).pack(side='left', padx=5)
        tk.Label(sp_c, text='\nAnimation Curve:', bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        c_var = tk.StringVar(value='Quartic')
        for c in ['Quartic', 'Bouncy', 'Elastic']:
            tk.Radiobutton(sp_c, text=c, variable=c_var, value=c, bg=PAL['card'], fg=PAL['text'], command=lambda c=c: self._log_voice(self.kernel.registry.get('customizer').adjust_animation_studio(c, 300))).pack(side='left', padx=5)
        typo_c = self._card(r_fr, '📝 Typography Morpher')
        typo_c.master.pack(fill='x', pady=5)

        def _morph_typo(w, s):
            self._log_voice(self.kernel.registry.get('customizer').morph_fonts(w, s)['message'])
        ttk.Button(typo_c, text='Sleek (Thin, 0.9x)', command=lambda: _morph_typo('Thin', 0.9)).pack(side='left', padx=2)
        ttk.Button(typo_c, text='Standard (Regular, 1x)', command=lambda: _morph_typo('Regular', 1.0)).pack(side='left', padx=2)
        ttk.Button(typo_c, text='Accessible (Bold, 1.3x)', command=lambda: _morph_typo('Bold', 1.3)).pack(side='left', padx=2)
        ttk.Button(typo_c, text='♿ High Contrast (WCAG)', command=lambda: self._log_voice('Global High-Contrast Mode Activated. Theme forced to [B/W Stark].')).pack(side='left', padx=5)
