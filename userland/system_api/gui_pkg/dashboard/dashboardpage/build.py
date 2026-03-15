"""
Auto-split from userland\system_api\gui_pkg\dashboard.py — DashboardPage.build
"""

import tkinter as tk
from tkinter import ttk, scrolledtext
import random
import time
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO, FONT_MED, FONT_TITLE



class DashboardPage:
    def build(self):
        is_child = self.controller._is_child_mode()
        container = tk.Frame(self, bg=PAL['bg'])
        container.pack(fill='both', expand=True, padx=20, pady=10)
        stats_row = tk.Frame(container, bg=PAL['bg'])
        stats_row.pack(fill='x', pady=(0, 20))
        if is_child:
            stat_defs = [('ram', 'MEMORY HAPPY', '12%', PAL['cyan']), ('cpu', 'BRAIN SPEED', '4%', PAL['teal']), ('cap', 'FUN METER', 'MAX', PAL['gold']), ('events', 'HAPPY EVENTS', '0', PAL['accent']), ('sec', 'SAFETY SCORE', '100', PAL['green'])]
        else:
            stat_defs = [('ram', 'RAM UTILIZATION', '12%', PAL['cyan']), ('cpu', 'CPU CORE LOAD', '4%', PAL['teal']), ('cap', 'SYSTEM CAPACITY', 'MAX', PAL['gold']), ('events', 'KERNEL EVENTS', '0', PAL['accent']), ('sec', 'SECURITY SCORE', '100', PAL['green'])]
        for key, label, val, color in stat_defs:
            var = tk.StringVar(value=val)
            self.controller._stat_widgets[key] = var
            card = self.controller._card(stats_row, label)
            card.master.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(card, textvariable=var, font=('Inter Bold', 20), fg=color, bg=PAL['card']).pack(anchor='w')
            pb_fr = tk.Frame(card, bg=PAL['border'], height=4)
            pb_fr.pack(fill='x', pady=(10, 0))
            inner_pb = tk.Frame(pb_fr, bg=color, width=40, height=4)
            inner_pb.place(x=0, y=0)
            if key == 'ram':
                self.controller._ram_pb = inner_pb
            if key == 'cpu':
                self.controller._cpu_pb = inner_pb
        hero_fr = tk.Frame(container, bg=PAL['bg'])
        hero_fr.pack(fill='x', pady=(0, 20))
        heatmap_title = '🌈 RAINBOW SIGNAL' if is_child else '⚡ REAL-TIME KERNEL HEATMAP'
        self.heatmap_fr = self.controller._card(hero_fr, heatmap_title)
        self.heatmap_fr.master.pack(side='left', fill='both', expand=True, padx=(0, 10))
        self.heatmap_canvas = tk.Canvas(self.heatmap_fr, height=40, bg=PAL['bg3'], highlightthickness=0)
        self.heatmap_canvas.pack(fill='x', pady=5)
        self.controller._heatmap_canvas = self.heatmap_canvas
        self._draw_heatmap()
        cr_title = '🤖 FRIENDLY ROBOT' if is_child else '🛡️ COMPETITOR CRUSHER'
        cr_card = self.controller._card(hero_fr, cr_title)
        cr_card.master.pack(side='left', fill='both', expand=True)
        crusher = self.controller.kernel.registry.get('crusher')
        health_txt = 'Ready to Play!' if is_child else crusher.health_check() if crusher else 'Crusher Offline'
        curr_cr = tk.Label(cr_card, text=health_txt, bg=PAL['card'], fg=PAL['gold'], font=FONT_BOLD)
        curr_cr.pack(side='left', padx=10)

        def _defeat():
            msg = 'Working Hard!' if is_child else crusher.defeat_telemetry() if crusher else 'N/A'
            self.controller._notify('OS', msg, 'OK')
        btn_txt = 'Say Hello' if is_child else 'Defeat Telemetry'
        ttk.Button(cr_card, text=btn_txt, command=_defeat).pack(side='right', padx=10)
        body = tk.Frame(container, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        qa_title = 'FUN HELP' if is_child else 'GLOBAL COMMANDS'
        qa = self.controller._card(body, qa_title)
        qa.master.pack(side='left', fill='both', expand=False, padx=(0, 10))
        qa.master.configure(width=220)
        qa.master.pack_propagate(False)

        def _btn(lbl, cmd, color=PAL['accent']):
            b = tk.Button(qa, text=lbl, command=cmd, font=FONT_SMALL, bg=PAL['bg3'], fg=PAL['text'], activebackground=color, relief='flat', bd=0, pady=8)
            b.pack(fill='x', pady=4)
            b.bind('<Enter>', lambda e: b.config(bg=color))
            b.bind('<Leave>', lambda e: b.config(bg=PAL['bg3']))
        if is_child:
            _btn('Start Playground', self.controller._do_boot, PAL['blue'])
            _btn('Go to Academy', lambda: self.controller._show_page('gurukul_academy'), PAL['purple'])
            _btn('Wash Hands', lambda: self.controller._notify('Reminder', 'Time to wash your hands!', 'OK'), PAL['teal'])
            _btn('Funny Game', lambda: self.controller._show_page('gaming_hub'), PAL['accent'])
        else:
            _btn('Run Boot Sequence', self.controller._do_boot, PAL['blue'])
            _btn('Aether Sync', lambda: self.controller._show_page('aether_orch'), PAL['purple'])
            _btn('Security Audit', lambda: self.controller._show_page('war_room'), PAL['red'])
            _btn('Performance Tune', self.controller._do_health, PAL['teal'])
            _btn('Mission Control', self.controller._show_mission_control, PAL['accent'])
        log_title = 'OS HAPPY LOG' if is_child else 'SOVEREIGN EVENT TELEMETRY'
        log_card = self.controller._card(body, log_title)
        log_card.master.pack(side='left', fill='both', expand=True)
        self.dash_log = self.controller._console(log_card, height=20)
        self.dash_log.pack(fill='both', expand=True, pady=(0, 10))
        self.controller._dash_log = self.dash_log
        self.controller._log(self.dash_log, 'SigmaOS ready for fun!' if is_child else 'Dashboard Online. Monitoring Kernel Bus...', 'HEAD')
        right_title = 'FRIENDLY SCORE' if is_child else 'COMPETITOR BLAME'
        met_card = self.controller._card(body, right_title)
        met_card.master.pack(side='left', fill='both', expand=False, padx=(10, 0))
        met_card.master.configure(width=280)
        met_card.master.pack_propagate(False)
        self.blame_scroll = tk.Frame(met_card, bg=PAL['card'])
        self.blame_scroll.pack(fill='both', expand=True, pady=5)
        self._refresh_blame()

        def _purge():
            log_msg = '✨ TIDYING UP: Making everything clean and fast.' if is_child else '⚡ PURGING SHIM DEBT: Cycle reclamation engaged.'
            self.controller._log(self.dash_log, log_msg, 'WARN')
            if hasattr(self.controller.kernel, 'perf'):
                res = self.controller.kernel.perf.steal_cycle_from_shims()
                ok_msg = '✔ All clean!' if is_child else f"✔ Reclaimed {res['reclaimed_tflops']} TFLOPS."
                self.controller._log(self.dash_log, ok_msg, 'OK')
            self._refresh_blame()
        btn_txt = 'TIDY UP' if is_child else 'PURGE SHIMS'
        tk.Button(met_card, text=btn_txt, font=('Inter Bold', 8), bg=PAL['red'], fg='white', bd=0, command=_purge).pack(side='bottom', fill='x', pady=5)
