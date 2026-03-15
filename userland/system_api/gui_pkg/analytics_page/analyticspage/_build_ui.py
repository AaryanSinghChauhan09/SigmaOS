# Generated method: AnalyticsPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_SMALL, FONT_BOLD, FONT_MED

class AnalyticsPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        metrics_fr = tk.Frame(body, bg=PAL['bg2'])
        metrics_fr.pack(fill='x', padx=15, pady=6)
        metrics = [('Velocity', '32 pts/sprint', PAL['cyan']), ('Throughput', '8.4 tasks/week', PAL['teal']), ('Bug Ratio', '12%', PAL['gold']), ('On-Time %', '89%', PAL['green'])]
        for label, val, col in metrics:
            m = tk.Frame(metrics_fr, bg=PAL['card'], padx=16, pady=8)
            m.pack(side='left', expand=True, fill='x', padx=6)
            tk.Label(m, text=val, font=('Segoe UI', 18, 'bold'), fg=col, bg=PAL['card']).pack()
            tk.Label(m, text=label, font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack()
        v_card = self._card(body, 'Sprint Velocity Trend')
        v_card.master.pack(fill='x', padx=15, pady=6)
        self.v_canv = tk.Canvas(v_card, bg=PAL['card'], height=140, highlightthickness=0)
        self.v_canv.pack(fill='x', padx=5, pady=5)

        def draw_velocity(event=None):
            self.v_canv.delete('all')
            W = self.v_canv.winfo_width() or 600
            H = 140
            sprints = [18, 22, 25, 29, 27, 32, 31, 35, 32]
            pad = 30
            max_v = max(sprints) + 5
            step = (W - 2 * pad) / (len(sprints) - 1)
            pts = [(pad + i * step, H - pad - s / max_v * (H - 2 * pad)) for i, s in enumerate(sprints)]
            poly = [pad, H - pad] + [c for p in pts for c in p] + [pts[-1][0], H - pad]
            self.v_canv.create_polygon(poly, fill='#1a2a3a', outline='')
            for i in range(len(pts) - 1):
                self.v_canv.create_line(pts[i], pts[i + 1], fill=PAL['cyan'], width=2.5, smooth=True)
            for i, (xp, yp) in enumerate(pts):
                self.v_canv.create_oval(xp - 3, yp - 3, xp + 3, yp + 3, fill=PAL['cyan'], outline='')
                self.v_canv.create_text(xp, H - 12, text=f'S{41 + i}', fill=PAL['dim'], font=('Segoe UI', 7))
                self.v_canv.create_text(xp, yp - 10, text=str(sprints[i]), fill=PAL['text'], font=('Segoe UI', 7))
        self.v_canv.bind('<Configure>', draw_velocity)
        self.after(150, draw_velocity)
        bot = tk.Frame(body, bg=PAL['bg2'])
        bot.pack(fill='x', padx=15, pady=6)
        tk.Label(bot, text='Total Time Logged: 142h  |  12 Sprints  |  Active Contributors: 4', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['bg2']).pack(side='left')
        ttk.Button(bot, text='Export CSV', command=lambda: self.gui._log_voice('Reports exported to /workspace/reports/')).pack(side='right')