# Generated method: OmniTweakDaemon._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniTweakDaemon:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-TWEAK & DAEMON FORGE', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🔄 REBUILD KERNEL HEADERS', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', padx=15, pady=8, command=self._rebuild_kernel).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(self.workspace, style='Tweak.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_de = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_de, text='DESKTOP ENVIRONMENT (DE)')
        self._build_de_tab()
        self.tab_daemon = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_daemon, text='DAEMON FORGE (systemd/cron)')
        self._build_daemon_tab()
        self.tab_dot = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_dot, text='DOTFILES MATRIX (.config)')
        self._build_dotfiles_tab()
        self.status = tk.Label(self, text='LINUX-TIER CUSTOMIZATION ACTIVE | ROOT TTY SECURED', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')