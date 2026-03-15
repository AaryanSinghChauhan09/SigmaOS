# Generated method: MailOrchestratorPage._build_ui
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MONO

class MailOrchestratorPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        ctrl_fr = self._card(body, 'Campaign Data Source')
        ctrl_fr.master.pack(fill='x', pady=(0, 10))
        tk.Label(ctrl_fr, text='Import CSV / JSON / CRM Leads:', bg=PAL['card'], fg=PAL['dim'], font=FONT_SMALL).pack(side='left')
        ttk.Button(ctrl_fr, text='📁 Upload Leads', command=lambda: self.gui._log_voice('Lead manifest uploaded: 452 entries found.')).pack(side='left', padx=10)
        panes = tk.Frame(body, bg=PAL['bg'])
        panes.pack(fill='both', expand=True)
        edit_fr = self._card(panes, 'Sovereign Template Editor')
        edit_fr.master.pack(side='left', fill='both', expand=True, padx=(0, 10))
        tool_bar = tk.Frame(edit_fr, bg=PAL['card'])
        tool_bar.pack(fill='x', pady=(0, 5))
        for token in ['{Name}', '{Company}', '{Project}', '{Deadline}']:
            tk.Button(tool_bar, text=token, font=('Inter', 8), bg=PAL['bg3'], fg=PAL['cyan'], relief='flat', padx=5).pack(side='left', padx=2)
        self._template_text = tk.Text(edit_fr, bg=PAL['bg2'], fg=PAL['text'], insertbackground=PAL['cyan'], font=('Consolas', 10), height=15)
        self._template_text.pack(fill='both', expand=True)
        self._template_text.insert('1.0', 'Hello {Name},\n\nI noticed your work at {Company} on the {Project} initiative.\n\nBest,\nSigma Core')
        ai_fr = self._card(panes, 'AI Cognitive Assistant')
        ai_fr.master.pack(side='right', fill='both', width=350)
        ai_fr.pack_propagate(False)
        tk.Label(ai_fr, text='Morphic Tone Switcher', font=FONT_BOLD, bg=PAL['card'], fg=PAL['accent']).pack(pady=5)
        tones = ['Professional', 'Persuasive', 'Casual', 'Urgent', 'Empathetic']
        for tone in tones:
            btn = tk.Button(ai_fr, text=tone, bg=PAL['bg3'], fg=PAL['dim'], relief='flat', pady=5, command=lambda t=tone: self._apply_tone(t))
            btn.pack(fill='x', pady=2)
        tk.Label(ai_fr, text='Cognitive Analysis:', font=FONT_BOLD, bg=PAL['card'], fg=PAL['dim']).pack(pady=(20, 5))
        self.analysis_lbl = tk.Label(ai_fr, text='Readability: High\nSentiment: Neutral\nSpam Risk: Low', bg=PAL['card'], fg=PAL['green'], justify='left', font=FONT_SMALL)
        self.analysis_lbl.pack(fill='x')
        tk.Label(ai_fr, text='Magic Draft Tools (MailMagic USP)', font=FONT_BOLD, bg=PAL['card'], fg=PAL['accent2']).pack(pady=(20, 5))
        ttk.Button(ai_fr, text='✨ AI Summarize Thread', command=lambda: self.gui._log_voice('MailMagic: Distilling 12 emails into 3 bullet points...')).pack(fill='x', pady=2)
        ttk.Button(ai_fr, text='🖋️ Professional AI Rewrite', command=lambda: self.gui._log_voice('Duet: Optimizing draft for executive clarity...')).pack(fill='x', pady=2)
        ttk.Button(body, text='🚀 Dispatch Campaign (Mail Merge)', style='Teal.TButton', command=self._dispatch).pack(fill='x', pady=10)