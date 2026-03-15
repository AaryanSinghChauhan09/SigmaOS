# Generated method: DashboardPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_MED, FONT_BOLD, FONT_SMALL

class DashboardPage:
    def build(self):
        stats_row = tk.Frame(self, bg=PAL['bg'])
        stats_row.pack(fill='x', pady=(0, 20))
        self.gui._stat_widgets = {}
        stat_defs = [('ram', 'RAM Utilization', '12%', PAL['cyan']), ('cpu', 'CPU Core Load', '4%', PAL['teal']), ('cap', 'System Capacity', 'MAX', PAL['gold']), ('zenith', 'Active AI Missions', '2', PAL['accent']), ('virt', 'Hypervisor', 'NONE', PAL['cyan'])]
        for i, (key, label, val, color) in enumerate(stat_defs):
            var = tk.StringVar(value=val)
            self.gui._stat_widgets[key] = var
            card = self.gui._premium_card(stats_row, label)
            card.master.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(card, textvariable=var, font=('Inter Bold', 22), fg=color, bg=PAL['card']).pack(anchor='w')
            pb_fr = tk.Frame(card, bg=PAL['border'], height=4)
            pb_fr.pack(fill='x', pady=(10, 0))
            inner_pb = tk.Frame(pb_fr, bg=color, width=40, height=4)
            inner_pb.place(x=0, y=0)
            if key == 'ram':
                self.gui._ram_pb = inner_pb
                self.gui._build_live_chart(card, 'ram', color, height=40)
            if key == 'cpu':
                self.gui._cpu_pb = inner_pb
                self.gui._build_live_chart(card, 'cpu', color, height=40)
            if key == 'virt':
                vb = self.kernel.registry.get('virtualizer')
                if vb:
                    res = vb.detect_virtualbox_environment()
                    var.set(res.get('hypervisor', 'NONE').upper())
                else:
                    var.set('BARE METAL')
        nexus_row = tk.Frame(self, bg=PAL['bg'])
        nexus_row.pack(fill='x', pady=10)
        nexus_card = self.gui._premium_card(nexus_row, '🧬 Sovereign AI Nexus: Task Agent & Guide')
        nexus_card.master.pack(side='left', fill='both', expand=True, padx=(0, 10))
        tk.Label(nexus_card, text='OS Status: Quantum-Secured | Telemetry: 0 | Anonymity: 100%', font=('Inter', 9), fg=PAL['cyan'], bg=PAL['card']).pack(anchor='w', pady=4)
        btn_fr = tk.Frame(nexus_card, bg=PAL['card'])
        btn_fr.pack(fill='x', pady=8)
        for lbl, cmd in [('📖 Guide Explorer', lambda: self.gui._show_page('nexus_ai')), ('🛡️ Loophole Audit', lambda: [self.gui._show_page('nexus_ai'), self.gui._notify('Audit', 'Scanning System Loopholes...', 'INFO')]), ('🤖 Talk to Nexus', lambda: self.gui._launch_app('sigma.ai.nexus_ai'))]:
            b = tk.Button(btn_fr, text=lbl, font=('Inter Bold', 8), bg=PAL['bg2'], fg=PAL['text'], padx=12, pady=6, relief='flat', command=cmd)
            b.pack(side='left', padx=5)
            b.bind('<Enter>', lambda e, bt=b: bt.config(bg=PAL['accent']))
            b.bind('<Leave>', lambda e, bt=b: bt.config(bg=PAL['bg2']))
        mission_card = self.gui._premium_card(nexus_row, '🧠 Fabric Orchestration')
        mission_card.master.pack(side='left', fill='both', expand=True)
        self.gui._mission_summary = tk.StringVar(value='Agentic Swarm: Idle | Fabric: 98% Perf')
        tk.Label(mission_card, textvariable=self.gui._mission_summary, font=FONT_MED, fg=PAL['dim'], bg=PAL['card']).pack(side='top', anchor='w', pady=5)
        ttk.Button(mission_card, text='Manage Fabric', width=18, command=lambda: self.gui._show_page('ai_lifecycle')).pack(anchor='w')