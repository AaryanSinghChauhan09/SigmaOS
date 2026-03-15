"""
Auto-split from userland\system_api\gui_pkg\chat_page.py — SigmaChatPage._build_ui
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL



class SigmaChatPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        self.panes = tk.PanedWindow(body, orient='horizontal', bg=PAL['bg'], sashwidth=2, bd=0)
        self.panes.pack(fill='both', expand=True)
        sidebar = tk.Frame(self.panes, bg=PAL['bg2'], width=250)
        self.panes.add(sidebar)
        tk.Label(sidebar, text='🔒 VERIFIED PEERS', font=FONT_BOLD, fg=PAL['cyan'], bg=PAL['bg2'], pady=10).pack()
        self.peer_list = tk.Listbox(sidebar, bg=PAL['bg3'], fg=PAL['text'], font=('Segoe UI', 9), selectbackground=PAL['accent'], borderwidth=0, highlightthickness=0)
        self.peer_list.pack(fill='both', expand=True, padx=5, pady=5)
        stats_fr = self.gui._premium_card(sidebar, 'Tunnel Stats', icon='⚡')
        stats_fr.master.pack(fill='x', side='bottom', padx=5, pady=5)
        self.stats_var = tk.StringVar(value='Active Tunnels: 0\nE2EE: AES-GCM')
        tk.Label(stats_fr, textvariable=self.stats_var, font=FONT_SMALL, fg=PAL['text'], bg=PAL['bg2'], justify='left').pack(anchor='w')
        chat_main = tk.Frame(self.panes, bg=PAL['bg'])
        self.panes.add(chat_main)
        header = tk.Frame(chat_main, bg=PAL['bg3'], pady=10, padx=15)
        header.pack(fill='x')
        id_fr = tk.Frame(header, bg=PAL['bg3'])
        id_fr.pack(side='left')
        self.my_sid_var = tk.StringVar(value='Local SID: OFFLINE')
        if self.engine and hasattr(self.engine, 'identity'):
            self.my_sid_var.set(f'Local SID: {self.engine.identity.sid}')
        tk.Label(id_fr, textvariable=self.my_sid_var, font=FONT_BOLD, fg=PAL['gold'], bg=PAL['bg3']).pack(side='left')
        self.alias_var = tk.StringVar(value='(@User)')
        if self.engine and hasattr(self.engine, 'identity'):
            self.alias_var.set(f'(@{self.engine.identity.alias})')
        tk.Label(id_fr, textvariable=self.alias_var, font=FONT_SMALL, fg=PAL['cyan'], bg=PAL['bg3']).pack(side='left', padx=5)
        tk.Button(header, text='🎭 SHIFT ALIAS', font=('Inter', 7, 'bold'), bg=PAL['bg4'], fg=PAL['text'], relief='flat', command=self._show_alias_switcher).pack(side='right', padx=10)
        tk.Label(header, text='● SECURE TUNNEL ACTIVE', font=FONT_SMALL, fg=PAL['green'], bg=PAL['bg3']).pack(side='right')
        log_container = tk.Frame(chat_main, bg=PAL['bg'])
        log_container.pack(fill='both', expand=True, pady=10)
        self.chat_log = tk.Text(log_container, bg=PAL['bg2'], fg=PAL['text'], font=('Segoe UI', 10), padx=15, pady=15, borderwidth=0, highlightthickness=0, state='disabled')
        self.chat_log.pack(fill='both', expand=True)
        input_fr = tk.Frame(chat_main, bg=PAL['bg3'], pady=15, padx=15)
        input_fr.pack(fill='x')
        self.msg_entry = self.gui._frosted_entry(input_fr, 'Type a secure message...')
        self.msg_entry.container.pack(side='left', fill='x', expand=True, padx=(0, 10))
        self.msg_entry.bind('<Return>', lambda e: self._send_msg())
        send_btn = self.gui._pulsing_button(input_fr, 'DISPATCH', self._send_msg)
        send_btn.pack(side='right')
        tool_bar = tk.Frame(chat_main, bg=PAL['bg'], pady=5)
        tool_bar.pack(fill='x')
        self.stealth_mode = tk.BooleanVar(value=True)
        tk.Checkbutton(tool_bar, text='👻 Stealth Mode', variable=self.stealth_mode, bg=PAL['bg'], fg=PAL['dim'], selectcolor=PAL['bg2']).pack(side='left', padx=10)
        self.shred_mode = tk.BooleanVar(value=True)
        tk.Checkbutton(tool_bar, text='🔥 Auto-Shred (60s)', variable=self.shred_mode, bg=PAL['bg'], fg=PAL['dim'], selectcolor=PAL['bg2']).pack(side='left', padx=10)
        self._refresh_peers()
