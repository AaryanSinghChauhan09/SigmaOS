# Generated method: PromptOMaticPage._build_content
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_LOGO, FONT_MONO

class PromptOMaticPage:
    def _build_content(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=300)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        sel_c = self._card(l_fr, 'Target AI Models')
        sel_c.master.pack(fill='x', pady=10)
        self.gui._pom_targets = {'OpenAI/ChatGPT': tk.BooleanVar(value=True), 'Anthropic/Claude': tk.BooleanVar(value=True), 'Google/Gemini': tk.BooleanVar(value=True), 'Meta/Llama-3': tk.BooleanVar(value=False), 'Perplexity': tk.BooleanVar(value=False)}
        for name, var in self.gui._pom_targets.items():
            ttk.Checkbutton(sel_c, text=name, variable=var).pack(anchor='w', pady=2)
        auth_c = self._card(l_fr, 'Workspace Auto-Login')
        auth_c.master.pack(fill='x', pady=10)
        self.gui._pom_autologin = tk.BooleanVar(value=True)
        ttk.Checkbutton(auth_c, text='Engage Sovereign Auto-Login', variable=self.gui._pom_autologin).pack(anchor='w')
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='right', fill='both', expand=True, padx=5)
        txt_c = self._card(r_fr, 'Universal Dispatch Command')
        txt_c.master.pack(fill='both', expand=True)
        self._prompt_txt = scrolledtext.ScrolledText(txt_c, height=12, font=FONT_MONO, bg='#000', fg=PAL['text'])
        self._prompt_txt.pack(fill='both', expand=True, pady=10)

        def _distribute():
            p = self._prompt_txt.get('1.0', 'end').strip()
            targets = [k for k, v in self.gui._pom_targets.items() if v.get()]
            self._notify('PROMPT-O-MATIC', f'Distributing intent to {len(targets)} models...', 'OK')
            self._morphic_island('AI DISPATCH ACTIVE', PAL['purple'])
        ttk.Button(txt_c, text='🚀 DISTRIBUTE PROMPT', command=_distribute).pack(side='right')