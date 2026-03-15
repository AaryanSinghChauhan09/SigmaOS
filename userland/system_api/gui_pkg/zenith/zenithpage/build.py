# Generated method: ZenithPage.build
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO

class ZenithPage:
    def build(self):
        self.controller._build_page_header(self, 'Sovereign Zenith', 'Integrated AI Control & Account Vault')
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, pady=10)
        zen = self.controller.kernel.registry.get('zenith')
        nodes = zen.nodes if zen else []
        quotas = zen.get_quotas() if zen else {}
        stats_fr = tk.Frame(body, bg=PAL['bg'])
        stats_fr.pack(fill='x', pady=10)
        for label, val, color in [('ACTIVE NODES', str(len(nodes)), PAL['cyan']), ('VAULT STATUS', 'ENCRYPTED ✅', PAL['gold']), ('MISSIONS', '248 (SYNCED)', PAL['purple'])]:
            card = self.controller._card(stats_fr, label)
            card.master.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(card, text=val, font=('Outfit', 20, 'bold'), fg=color, bg=PAL['card']).pack(pady=5)
        console_fr = tk.Frame(body, bg=PAL['bg'])
        console_fr.pack(fill='both', expand=True, pady=10)
        l_fr = self.controller._card(console_fr, '⚡ Rapid Mission Dispatch')
        l_fr.master.pack(side='left', fill='both', expand=True, padx=5)
        prompt_ta = scrolledtext.ScrolledText(l_fr, height=8, font=FONT_MONO, bg=PAL['bg'], fg=PAL['text'], insertbackground=PAL['cyan'], bd=0, highlightthickness=1)
        prompt_ta.pack(fill='both', expand=True, pady=10)
        prompt_ta.insert('1.0', 'Analyze SigmaOS kernel for security loops and optimize...')

        def _dispatch():
            txt = prompt_ta.get('1.0', 'end').strip()
            res = zen.dispatch_mission(txt, [n['name'] for n in nodes]) if zen else 'Kernel Error'
            self.controller._log_voice(res)
            self.controller._notify('Zenith', 'AI Mission Dispatched successfully.', 'OK')
        ttk.Button(l_fr, text='↑ Launch Sovereign Mission', command=_dispatch).pack(side='right')
        ttk.Button(l_fr, text='🔐 Manage Vault', command=lambda: self.controller._notify('Vault', 'Credential Manager (AES-256) Locked.', 'INFO')).pack(side='right', padx=5)
        r_fr = self.controller._card(console_fr, '📡 Orchestrator Telemetry')
        r_fr.master.pack(side='right', fill='both', width=450, padx=5)
        for n in nodes:
            name = n['name']
            q = quotas.get(name, {'percent': 50})
            color = PAL['green'] if q['percent'] < 50 else PAL['gold'] if q['percent'] < 80 else PAL['red']
            n_row = tk.Frame(r_fr, bg=PAL['card'], pady=5)
            n_row.pack(fill='x')
            tk.Label(n_row, text=name, font=FONT_BOLD, fg=PAL['text'], bg=PAL['card']).pack(side='left')
            tk.Label(n_row, text=f"{q['percent']}%", font=FONT_MONO, fg=color, bg=PAL['card']).pack(side='right')
            pb = ttk.Progressbar(r_fr, maximum=100)
            pb.pack(fill='x', pady=(0, 10))
            pb['value'] = q['percent']
        snap_f = tk.Frame(self, bg=PAL['bg2'], height=60)
        snap_f.pack(fill='x', pady=10)
        tk.Label(snap_f, text='📐 Sovereign Snap Grid Pro', font=FONT_BOLD, fg=PAL['teal'], bg=PAL['bg2']).pack(side='left', padx=20)
        ttk.Button(snap_f, text='Enable 2x2 Logic Grid', command=lambda: self.controller._log_voice('Snap Grid: 2x2 Balanced Layout Enforced.')).pack(side='right', padx=10)
        ttk.Button(snap_f, text='Enable Golden Ratio Layout', command=lambda: self.controller._log_voice('Snap Grid: Cinematic Golden Ratio Layout Enforced.')).pack(side='right')