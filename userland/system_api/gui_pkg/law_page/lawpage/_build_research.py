# Generated method: LawPage._build_research
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_research(self, parent):
        l_fr = tk.Frame(parent, bg=PAL['bg2'], width=350)
        l_fr.pack(side='left', fill='both', padx=5, pady=5)
        l_fr.pack_propagate(False)
        tk.Label(l_fr, text='Bare Act / CaseIQ', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg2']).pack(pady=5)
        s_ent = ttk.Entry(l_fr)
        s_ent.pack(fill='x', padx=10)
        s_ent.insert(0, 'BNSS_2023 Section 154')
        res_text = tk.Text(l_fr, font=FONT_SMALL, bg=PAL['bg'], fg=PAL['text'], height=15)
        res_text.pack(fill='both', expand=True, padx=10, pady=5)

        def do_research():
            txt = s_ent.get()
            q = self.kernel.law.ai_case_iq(txt)
            res_text.delete('1.0', 'end')
            res_text.insert('end', f'CaseIQ Suggestions:\n' + '─' * 20 + '\n')
            for r in q:
                res_text.insert('end', f"• {r.get('Reference')}: {r.get('Meaning')}\n\n")
        ttk.Button(l_fr, text='Execute CaseIQ Search', command=do_research).pack(pady=5)
        r_fr = tk.Frame(parent, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5, pady=5)
        tk.Label(r_fr, text='Internal Case Database', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w')
        web_f = tk.Frame(r_fr, bg=PAL['bg'])
        web_f.pack(fill='x', pady=10)
        for site in ['IndianKanoon', 'IndiaCode']:
            ttk.Button(web_f, text=f'Sync {site}', command=lambda: self.gui._notify('Law', f'Syncing with {site}...', 'OK')).pack(side='left', padx=5)