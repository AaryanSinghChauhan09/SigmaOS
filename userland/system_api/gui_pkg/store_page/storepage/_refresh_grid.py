# Generated method: StorePage._refresh_grid
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class StorePage:
    def _refresh_grid(self):
        for w in self._grid.winfo_children():
            w.destroy()
        cat_filter = self._store_cat.get()
        q = self._search_v.get().lower()
        child_active = self.gui._is_child_mode()
        visible = []
        for a in self._all_apps:
            if (cat_filter == 'All' or a[1] == cat_filter) and (not q or q in a[0].lower() or q in a[2].lower()) and (not child_active or a[6]):
                visible.append(a)
        for i, (name, tag, desc, icon, aid, color, is_safe) in enumerate(visible):
            r, c = divmod(i, 3)
            card = tk.Frame(self._grid, bg=PAL['card'], width=310, height=230, highlightthickness=1, highlightbackground=PAL['border'])
            card.grid(row=r, column=c, padx=10, pady=10, sticky='nsew')
            card.pack_propagate(False)
            band = tk.Frame(card, bg=color, height=4)
            band.pack(fill='x')
            head = tk.Frame(card, bg=PAL['card'], pady=8)
            head.pack(fill='x', padx=12)
            tk.Label(head, text=icon, font=('Segoe UI Symbol', 22), bg=PAL['card'], fg=color).pack(side='left')
            clean_name = name.lstrip('🧬⚡♟🎲🚪📝♫🎨🔒')
            tk.Label(head, text=clean_name, font=FONT_BOLD, bg=PAL['card'], fg='white').pack(side='left', padx=8)
            tk.Label(card, text=desc, font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card'], wraplength=270, justify='left').pack(fill='x', padx=12, pady=8)
            btn = tk.Button(card, text='START', font=('Segoe UI', 9, 'bold'), bg=color, fg='white', relief='flat', pady=7, command=lambda a=aid: self.gui._launch_app(a))
            btn.pack(side='bottom', fill='x', padx=12, pady=10)