# Generated method: SigmaGUI._build_store_page
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
    def _build_store_page(self):
        """Dynamic Sovereign Forge: Categorized, high-performance app delivery."""
        p = tk.Frame(self._content, bg=PAL['bg'])
        self._pages['store'] = p
        header = tk.Frame(p, bg=PAL['bg'])
        header.pack(fill='x', pady=(0, 10))
        tk.Label(header, text='📦  Sigma Sovereign Forge', font=FONT_LOGO, fg=PAL['cyan'], bg=PAL['bg']).pack(side='left')

        def _hydrate_all_ag():
            self._log_voice('Hydrating full Antigravity Suite... Performance lock engaged.')
            catalog = self.kernel.app_store.get_catalog()
            count = 0
            for app in catalog:
                if app['developer'] == 'Antigravity':
                    self.kernel.app_store.install(app['app_id'])
                    c_val = int(count)
                    count = c_val + 1
            messagebox.showinfo('Sigma Forge', f'Successfully hydrated {count} Antigravity assets.')
            self._show_page('store')
        ttk.Button(header, text='⚡ Hydrate Antigravity Suite', command=_hydrate_all_ag).pack(side='right', padx=10)
        tabs = ttk.Notebook(p)
        tabs.pack(fill='both', expand=True)
        categories = self.kernel.app_store.get_categories()
        icon_map = {'Games': '🎮', 'AI': '🧠', 'Productivity': '💼', 'Development': '💻', 'Security': '🛡️', 'System': '⚙️', 'Communication': '📧', 'Automation': '⚡', 'Documentation': '📚', 'Finance': '💵', 'Media': '🎨'}
        for cat in categories:
            f = tk.Frame(tabs, bg=PAL['bg'])
            icon = icon_map.get(cat, '📦')
            tabs.add(f, text=f' {icon} {cat} ')
            canvas = tk.Canvas(f, bg=PAL['bg'], highlightthickness=0)
            sb = ttk.Scrollbar(f, orient='vertical', command=canvas.yview)
            grid = tk.Frame(canvas, bg=PAL['bg'])
            canvas.create_window((0, 0), window=grid, anchor='nw')
            grid.bind('<Configure>', lambda e: canvas.configure(scrollregion=canvas.bbox('all')))
            canvas.configure(yscrollcommand=sb.set)
            canvas.pack(side='left', fill='both', expand=True)
            sb.pack(side='right', fill='y')
            apps = self.kernel.app_store.get_catalog(category=cat)
            for i, app in enumerate(apps):
                row, col = (i // 3, i % 3)
                item = tk.Frame(grid, bg=PAL['card'], padx=10, pady=10, width=280, height=200)
                item.grid(row=row, column=col, padx=10, pady=10)
                item.pack_propagate(False)
                tk.Label(item, text=icon, font=('Segoe UI', 32), bg=PAL['card']).pack()
                tk.Label(item, text=app['name'], font=FONT_BOLD, fg=PAL['text'], bg=PAL['card']).pack()
                tk.Label(item, text=f"{app['size_mb']} MB | ⭐ {app['rating']}", font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack()
                tk.Label(item, text=app['description'], font=('Segoe UI', 8), fg=PAL['text'], bg=PAL['card'], wraplength=250).pack(pady=10)
                btn_frame = tk.Frame(item, bg=PAL['card'])
                btn_frame.pack(side='bottom', fill='x', pady=5)
                status_text = '⬇️ Download & Install' if not app['installed'] else '🚀 Launch App'
                color = PAL['accent'] if not app['installed'] else PAL['green']
                btn = tk.Button(btn_frame, text=status_text, bg=color, fg='white', font=FONT_BOLD, relief='flat', command=lambda a=app['app_id'], inst=app['installed']: self._install_app(a) if not inst else self._launch_app(a))
                btn.pack(fill='x', padx=10)