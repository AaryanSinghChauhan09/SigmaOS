# Generated method: SigmaGUI._build_mathema_page
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
    def _build_mathema_page(self):
        sm = self.kernel.math
        if not sm:
            from sigma_mathema import SigmaMathema
            sm = SigmaMathema()
        p = tk.Frame(self._content, bg=PAL['bg'])
        self._pages['mathema'] = p
        tk.Label(p, text='Σ Mathema: Sovereign Engineering Intelligence', font=FONT_LOGO, fg=PAL['teal'], bg=PAL['bg']).pack(anchor='w', pady=(0, 8))
        tk.Label(p, text='NCERT K-12 Syllabus | IIT-JEE Advanced | Engineering Calculus | Physics Constants', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        body = tk.Frame(p, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=450)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        calc_card = self._card(l_fr, '🧮 Sovereign Calculator Engine')
        calc_card.master.pack(fill='x', pady=10)
        ent = ttk.Entry(calc_card, font=('Consolas', 14))
        ent.pack(fill='x', pady=10)
        ent.insert(0, 'sin(pi/4) * sqrt(2)')
        btn_f = tk.Frame(calc_card, bg=PAL['card'])
        btn_f.pack(fill='x')

        def run_eval():
            expr = ent.get()
            if not expr.strip():
                return
            res = sm.evaluate_expression(expr)
            self._log(m_log, f'\nIN  : {expr}', 'INFO')
            if isinstance(res, (int, float)):
                self._log(m_log, f'OUT : {res:.6g}', 'OK')
            else:
                self._log(m_log, f'OUT : {res}', 'OK' if not str(res).startswith('Error') else 'ERR')
            m_log.see('end')
        ttk.Button(btn_f, text='Evaluate (IIT-JEE)', command=run_eval).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(btn_f, text='AC', command=lambda: [ent.delete(0, 'end'), m_log.delete('1.0', 'end')]).pack(side='left', padx=2)
        subj_card = self._card(l_fr, '🎓 NCERT & IIT-JEE Subject Labs')
        subj_card.master.pack(fill='x', pady=10)

        def set_p(expr):
            ent.delete(0, 'end')
            ent.insert(0, expr)
        tk.Label(subj_card, text='Junior Maths (Class 1-8):', bg=PAL['card'], fg=PAL['green']).pack(anchor='w', pady=(5, 0))
        j_row = tk.Frame(subj_card, bg=PAL['card'])
        j_row.pack(fill='x', pady=2)
        ttk.Button(j_row, text='Table (7x8)', command=lambda: set_p('7 * 8')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(j_row, text='Area (Circ)', command=lambda: set_p('pi * r**2')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(j_row, text='Percent (%)', command=lambda: set_p('(part/total)*100')).pack(side='left', fill='x', expand=True, padx=2)
        tk.Label(subj_card, text='Advanced Maths (9-12 / JEE):', bg=PAL['card'], fg=PAL['cyan']).pack(anchor='w', pady=(5, 0))
        m_row = tk.Frame(subj_card, bg=PAL['card'])
        m_row.pack(fill='x', pady=2)
        ttk.Button(m_row, text='d/dx', command=lambda: set_p("sm.jee_derivative_sim('x**3', 2)")).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(m_row, text='∫ Integral', command=lambda: set_p("sm.jee_integral_sim('sin(x)', 0, pi)")).pack(side='left', fill='x', expand=True, padx=2)
        tk.Label(subj_card, text='Physics (Mechanics/Quantum):', bg=PAL['card'], fg=PAL['gold']).pack(anchor='w', pady=(5, 0))
        p_row = tk.Frame(subj_card, bg=PAL['card'])
        p_row.pack(fill='x', pady=2)
        ttk.Button(p_row, text='Const: G', command=lambda: set_p('6.674e-11')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(p_row, text='Const: h', command=lambda: set_p('6.626e-34')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(p_row, text='K.E.', command=lambda: set_p('0.5 * m * v**2')).pack(side='left', fill='x', expand=True, padx=2)
        tk.Label(subj_card, text='Chemistry (Periodic/Thermo):', bg=PAL['card'], fg=PAL['teal']).pack(anchor='w', pady=(5, 0))
        c_row = tk.Frame(subj_card, bg=PAL['card'])
        c_row.pack(fill='x', pady=2)

        def show_chem(sym):
            data = sm.chemistry_data(sym)
            self._log(m_log, f'\n[CHEM] Element: {sym}', 'HEAD')
            for k, v in data.items():
                self._log(m_log, f'  {k}: {v}', 'OK')
        ttk.Button(c_row, text='Element (H)', command=lambda: show_chem('H')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(c_row, text='Molar (H2O)', command=lambda: self._log(m_log, f"H2O Molar Mass: {sm.molar_mass_calc({'H': 2, 'O': 1})}", 'OK')).pack(side='left', fill='x', expand=True, padx=2)
        ttk.Button(c_row, text='Ideal Gas', command=lambda: set_p('sm.ideal_gas_law(P=1, V=22.4, n=1)')).pack(side='left', fill='x', expand=True, padx=2)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        out_card = self._card(r_fr, '📊 Mathematical Ledger & Study Journal')
        out_card.master.pack(fill='both', expand=True)
        m_log = self._console(out_card, height=30)
        m_log.pack(fill='both', expand=True)
        self._log(m_log, 'Mathema v2.0 Apex: Science & Engineering Kernel Loaded.', 'HEAD')
        self._log(m_log, 'NCERT K-12 Syllabus: Maths, Physics, Chemistry [OFFLINE]', 'INFO')