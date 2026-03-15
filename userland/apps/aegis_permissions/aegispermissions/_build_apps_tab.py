# Generated method: AegisPermissions._build_apps_tab
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def _build_apps_tab(self):
        scroll_f = tk.Frame(self.tab_apps, bg=PAL['bg'])
        scroll_f.pack(fill='both', expand=True)
        for app, granted, denied in self.apps:
            card = tk.Frame(scroll_f, bg=PAL['panel'], pady=15, padx=20)
            card.pack(fill='x', pady=5)
            tk.Label(card, text=app, font=('Inter', 12, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(side='left', padx=10)
            btn_fr = tk.Frame(card, bg=PAL['panel'])
            btn_fr.pack(side='right')
            tk.Label(btn_fr, text=f'{len(granted)} Tokens', font=('Inter', 9, 'bold'), fg=PAL['success'], bg=PAL['panel']).pack(side='left', padx=10)
            tk.Label(btn_fr, text=f'{len(denied)} Blocked', font=('Inter', 9, 'bold'), fg=PAL['danger'], bg=PAL['panel']).pack(side='left', padx=10)
            tk.Button(btn_fr, text='INSPECT MATRIX', font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=10, pady=4, command=lambda a=app: self._inspect_app(a)).pack(side='left', padx=5)