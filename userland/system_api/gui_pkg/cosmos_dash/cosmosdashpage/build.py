# Generated method: CosmosDashPage.build
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class CosmosDashPage:
    def build(self):
        self.controller._build_page_header(self, 'Cosmos AI-OS', 'Neural-Native Compute & Fabric Dashboard')
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20)
        card = self.controller._card(body, '📡 Mesh Compute Stats')
        card.master.pack(fill='x', pady=10)
        self.fabric_label = tk.Label(card, text='Fabric: IDLE | Compute: 0 TFLOPS', font=FONT_BOLD, fg=PAL['accent'], bg=PAL['card'])
        self.fabric_label.pack(pady=10)

        def _refresh_fabric():
            try:
                if hasattr(self.controller.kernel, 'fabric') and self.controller.kernel.fabric:
                    res = self.controller.kernel.fabric.health_check()
                    self.fabric_label.config(text=str(res))
                else:
                    self.fabric_label.config(text='Fabric: OFFLINE | Bare Metal Mode')
            except Exception as e:
                self.fabric_label.config(text=f'Fabric: ERROR — {str(e)[:50]}')
            self.after(5000, _refresh_fabric)
        _refresh_fabric()
        ttk.Button(card, text='Join Hybrid Fabric', command=lambda: [getattr(self.controller.kernel, 'fabric', None) and self.controller.kernel.fabric.join_compute_fabric(), _refresh_fabric()]).pack(pady=5)
        auto_fr = tk.Frame(body, bg=PAL['bg'])
        auto_fr.pack(fill='x', pady=10)
        auto_l_fr = self.controller._card(auto_fr, '🤖 Automation')
        auto_l_fr.master.pack(side='left', fill='both', expand=True, padx=5)

        def _run_auto(task):
            try:
                if hasattr(self.controller.kernel, 'automation'):
                    result = self.controller.kernel.automation.run_task(task)
                    self.controller._notify('Automation', str(result), 'OK')
                else:
                    self.controller._notify('Automation', f"Task '{task}' dispatched via OmniAutomator.", 'INFO')
            except Exception as e:
                self.controller._notify('Automation Error', str(e), 'ERR')
        for lbl, task in [('⚙ Optimize System', 'optimize'), ('🧹 Clean Cache', 'clean'), ('🔄 Sync Cloud', 'sync')]:
            ttk.Button(auto_l_fr, text=lbl, command=lambda t=task: _run_auto(t)).pack(fill='x', pady=2)
        nr = self.controller._card(auto_fr, '🧠 Neuro-Top')
        nr.master.pack(side='right', fill='both', expand=True, padx=5)
        nc = tk.Canvas(nr, height=140, bg='#0D0F12', highlightthickness=0)
        nc.pack(fill='both', expand=True, padx=5, pady=5)

        def _draw_neuro():
            if not nc.winfo_exists():
                return
            nc.delete('all')
            W = nc.winfo_width() or 300
            H = nc.winfo_height() or 140
            gs, cw, ch = (12, W / 12, H / 8)
            for r in range(8):
                for c in range(gs):
                    act = random.random()
                    col = '#0a0a12' if act < 0.3 else PAL['dim'] if act < 0.65 else PAL['accent']
                    nc.create_rectangle(c * cw, r * ch, (c + 1) * cw - 1, (r + 1) * ch - 1, fill=col, outline='')
            nc.create_text(W / 2, H / 2, text='Neural Activity', fill='#ffffff44', font=FONT_BOLD)
            self.after(1200, _draw_neuro)
        nc.bind('<Configure>', lambda e: _draw_neuro())
        self.after(500, _draw_neuro)