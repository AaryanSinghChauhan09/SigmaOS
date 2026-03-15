# Generated method: SigmaGUI._handle_assistant_approval
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
    def _handle_assistant_approval(self, approved=True):
        a = self.kernel.assistant
        if a:
            cmd = 'Proceed' if approved else 'Cancel'
            res = a.handle_user_response(cmd)
            self._log_voice(res)
            if 'Mission Complete' in res or 'aborted' in res:
                self.after(2000, lambda: self._island_lbl.config(text='🛡️ SOVEREIGN DEFENSE ACTIVE', fg=PAL['cyan']))
        comp_view = tk.Text(comp_p, font=FONT_MED, bg=PAL['bg2'], fg=PAL['text'], height=10)
        comp_view.pack(fill='x', pady=10)

        def do_comp():
            res = self.kernel.buyhatke.compare_platforms(prod_ent.get())
            comp_view.delete('1.0', 'end')
            for site, p in res.items():
                comp_view.insert('end', f'{site.ljust(15)}: ₹{p:,}\n')
        ttk.Button(comp_p, text='Compare Prices Now', command=do_comp).pack()
        usp_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['usp anal'] = usp_p
        tk.Label(usp_p, text='AI USP Analysis & Strategy (Praxie Hub)', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w')
        u_view = tk.Text(usp_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['text'], height=12)
        u_view.pack(fill='x', pady=10)

        def run_usp():
            res = self.kernel.buyhatke.analyze_usp_matrix('Legal_IT')
            u_view.delete('1.0', 'end')
            self._log(u_view, 'STRATEGIC USP MATRIX\n' + '─' * 30 + '\n', 'HEAD')
            for k, v in res.items():
                self._log(u_view, f'{k}: {v}\n', 'OK')
        ttk.Button(usp_p, text='Analyze Competitive USP', command=run_usp).pack()
        mkt_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['market'] = mkt_p
        tk.Label(mkt_p, text='Market Intelligence & Gaps (SEMrush Hub)', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(anchor='w')
        m_view = tk.Text(mkt_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['gold'], height=10)
        m_view.pack(fill='x', pady=10)

        def run_mkt():
            res = self.kernel.buyhatke.market_intel_discovery('LawTech')
            m_view.delete('1.0', 'end')
            self._log(m_view, 'MARKET DISCOVERY REPORT\n', 'HEAD')
            for k, v in res.items():
                m_view.insert('end', f'{k}: {v}\n')
        ttk.Button(mkt_p, text='Discover Market Gaps', command=run_mkt).pack()
        crm_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['crm'] = crm_p
        tk.Label(crm_p, text='CRM & Lead Pipeline (Salesforce Hub)', font=FONT_MED, fg=PAL['teal'], bg=PAL['bg']).pack(anchor='w')
        c_view2 = tk.Text(crm_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['text'], height=10)
        c_view2.pack(fill='x', pady=10)

        def run_crm():
            leads = self.kernel.buyhatke.crm_lead_pipeline()
            c_view2.delete('1.0', 'end')
            self._log(c_view2, 'LIVE LEAD PIPELINE\n' + '─' * 30 + '\n', 'HEAD')
            for l in leads:
                c_view2.insert('end', f"👤 {l['Lead']} - Score: {l['Score']} - Status: {l['Status']}\n")
        ttk.Button(crm_p, text='Sync CRM Pipeline', command=run_crm).pack()
        log_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['logistics'] = log_p
        tk.Label(log_p, text='Integrated Logistics Hub (EDI Tracking)', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w')
        l_view2 = tk.Text(log_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['text'], height=10)
        l_view2.pack(fill='x', pady=10)

        def track_awb():
            res = self.kernel.buyhatke.track_shipment_edi('SIGMA-AWB-9021')
            l_view2.delete('1.0', 'end')
            self._log(l_view2, 'LIVE TRACKING REPORT (Ekart/Delhivery Link)\n', 'HEAD')
            for k, v in res.items():
                l_view2.insert('end', f'{k}: {v}\n')
        ttk.Button(log_p, text='Track Shipment (SIGMA-AWB-9021)', command=track_awb).pack()
        b2b_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['b2b'] = b2b_p
        tk.Label(b2b_p, text='B2B Supply Chain & Inquiry Manager', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(anchor='w')
        b_view2 = tk.Text(b2b_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['gold'], height=10)
        b_view2.pack(fill='x', pady=10)

        def run_b2b():
            res = self.kernel.buyhatke.b2b_market_tracker('Raw_Materials')
            b_view2.delete('1.0', 'end')
            for k, v in res.items():
                b_view2.insert('end', f'{k}: {v}\n')
        ttk.Button(b2b_p, text='Refresh B2B Inquiries', command=run_b2b).pack()
        soc_p = tk.Frame(container, bg=PAL['bg'])
        hatke_sub['social'] = soc_p
        tk.Label(soc_p, text='Social Commerce & Reseller Network', font=FONT_MED, fg=PAL['teal'], bg=PAL['bg']).pack(anchor='w')
        s_view2 = tk.Text(soc_p, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['text'], height=10)
        s_view2.pack(fill='x', pady=10)

        def run_soc():
            res = self.kernel.buyhatke.analyze_social_commerce()
            s_view2.delete('1.0', 'end')
            self._log(s_view2, 'RESELLER PERFORMANCE (Meesho Hub)\n', 'HEAD')
            for r in res:
                s_view2.insert('end', f"👤 {r['Reseller']} | Orders: {r['Orders']} | Earned: ₹{r['Commission']}\n")
        ttk.Button(soc_p, text='Analyze Reseller Performance', command=run_soc).pack()
        hatke_sub['tracker'].pack(fill='both', expand=True)