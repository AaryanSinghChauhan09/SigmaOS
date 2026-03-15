# Generated method: ClawPage._build_interface
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_MED, FONT_MONO
from sigma_core.ai.sovereign_claw import SovereignClaw

class ClawPage:
    def _build_interface(self):
        main = tk.Frame(self, bg=PAL['bg'], padx=40, pady=20)
        main.pack(fill='both', expand=True)
        left = tk.Frame(main, bg=PAL['bg'], width=400)
        left.pack(side='left', fill='both', expand=True)
        card = self._card(left, 'Claw Agent Status')
        card.master.pack(fill='x', pady=(0, 20))
        self.status_lbl = tk.Label(card, text='● STANDBY', font=FONT_MED, fg=PAL['green'], bg=PAL['card'])
        self.status_lbl.pack(pady=10)
        tk.Label(card, text='Current Goal: N/A', font=FONT_MONO, fg=PAL['dim'], bg=PAL['card']).pack()
        prompt_fr = self._card(left, 'Personal Computer Intent')
        prompt_fr.master.pack(fill='x')
        self.prompt_ent = tk.Entry(prompt_fr, bg=PAL['bg2'], fg=PAL['text'], insertbackground=PAL['accent'], font=FONT_MED, bd=0)
        self.prompt_ent.pack(fill='x', pady=10, ipady=8)
        self.prompt_ent.bind('<Return>', lambda e: self._execute())
        tk.Button(prompt_fr, text='ENGAGE SOVEREIGN CLAW', bg=PAL['accent'], fg='white', font=FONT_BOLD, relief='flat', command=self._execute).pack(pady=10, fill='x')
        right = tk.Frame(main, bg=PAL['bg'], padx=(20, 0))
        right.pack(side='right', fill='both', expand=True)
        tk.Label(right, text='ACTION TRACE', font=FONT_BOLD, fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.log = scrolledtext.ScrolledText(right, bg=PAL['card'], fg=PAL['cyan'], font=FONT_MONO, bd=0)
        self.log.pack(fill='both', expand=True, pady=10)
        self.log.tag_config('user', foreground=PAL['accent'])
        self.log.tag_config('sys', foreground='white')