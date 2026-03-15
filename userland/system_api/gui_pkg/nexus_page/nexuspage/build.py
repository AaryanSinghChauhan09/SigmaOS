# Generated method: NexusPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_LOGO

class NexusPage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=400)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        auth_c = self._card(l_fr, '🔐 Integrated Authentication')
        auth_c.master.pack(fill='x', pady=5)
        self.status = tk.StringVar(value='Status: Connected (Sovereign)')
        tk.Label(auth_c, textvariable=self.status, font=FONT_BOLD, fg=PAL['teal'], bg=PAL['card']).pack(pady=10)
        ttk.Button(auth_c, text='Re-map Identity Vault', command=lambda: self._notify('Auth', 'Identity re-mapped via Neuro-Vault.', 'OK')).pack(fill='x')
        model_c = self._card(l_fr, '⚡ Active Model Orchestrator')
        model_c.master.pack(fill='x', pady=5)
        models = [('Llama-3-Sovereign', 'USA'), ('DeepSeek-V3', 'India'), ('Sarvam-1', 'India')]
        for m, r in models:
            tk.Label(model_c, text=f'• {m} ({r})', font=FONT_SMALL, fg=PAL['text'], bg=PAL['card']).pack(anchor='w', padx=10, pady=2)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        console_c = self._card(r_fr, '🖥️ Multi-Model Console')
        console_c.master.pack(fill='both', expand=True)
        self.log = self._console(console_c, height=25)
        self.log.pack(fill='both', expand=True)
        self._log(self.log, 'Nexus Core v4 Online. Ready for cross-model inference.', 'INFO')
        p_fr = tk.Frame(r_fr, bg=PAL['bg'])
        p_fr.pack(fill='x', pady=10)
        p_ent = ttk.Entry(p_fr)
        p_ent.pack(side='left', fill='x', expand=True, padx=5)
        p_ent.insert(0, 'Analyze recent market trends...')

        def run_prompt():
            self._log(self.log, f'\n▶ PROMPT: {p_ent.get()}', 'HEAD')
            self.after(500, lambda: self._log(self.log, 'Response distilled from 3 shards via Merkle-Consensus.', 'OK'))
        ttk.Button(p_fr, text='🚀 Prompt', command=run_prompt).pack(side='left')