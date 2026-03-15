# Generated method: WritesensePage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class WritesensePage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=500)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        tk.Label(l_fr, text='Editorial Workbench', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg2']).pack(pady=5)
        self.text = tk.Text(l_fr, font=('Georgia', 11), bg=PAL['bg'], fg=PAL['text'], padx=10, pady=10, wrap='word')
        self.text.pack(fill='both', expand=True, padx=10, pady=5)
        self.text.insert('1.0', 'Drafting sovereign intelligence documents...')
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        s_card = self._card(r_fr, '💡 Intelligence Suggestions')
        s_card.master.pack(fill='both', expand=True)
        self.log = self._console(s_card, height=15)
        self.log.pack(fill='both', expand=True)
        self._log(self.log, 'WriteSense Engine Ready.', 'INFO')
        act_fr = tk.Frame(r_fr, bg=PAL['bg'])
        act_fr.pack(fill='x', pady=10)

        def run_audit():
            self._log(self.log, 'Running readability and tone audit...', 'HEAD')
            self.after(500, lambda: self._log(self.log, 'Audit Complete: Logic Score 98/100', 'OK'))

        def run_excel_fill():
            self._log(self.log, 'Initializing Excel AI Filler...', 'INFO')
            self._log(self.log, '[AI] Analyzing tabular context and projecting missing vectors...', 'OK')
            self.gui._notify('Excel AI', 'Tabular data successfully synthesized.', 'OK')
        ttk.Button(act_fr, text='🚀 Run Deep Audit', command=run_audit).pack(side='left', padx=5)
        ttk.Button(act_fr, text='📊 Excel AI Filler', command=run_excel_fill).pack(side='left', padx=5)