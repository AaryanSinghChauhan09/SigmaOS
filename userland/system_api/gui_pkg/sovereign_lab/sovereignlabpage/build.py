# Generated method: SovereignLabPage.build
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO

class SovereignLabPage:
    def build(self):
        self.controller._build_page_header(self, 'Sovereign Lab', 'Architect-to-Agent Workflow Control')
        lab = self.controller.kernel.registry.get('lab')
        agents = lab.list_agents() if lab else []
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, pady=10)
        up = tk.Frame(body, bg=PAL['bg'])
        up.pack(fill='x', pady=10)
        for agent in agents:
            c = self.controller._card(up, agent['name'])
            c.master.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(c, text=agent['focus'], font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack()
            tk.Label(c, text=f"Status: {agent['status']}", font=FONT_BOLD, fg=PAL['green'], bg=PAL['card']).pack(pady=5)
            ttk.Button(c, text='RECONFIGURE').pack()
        low = tk.Frame(body, bg=PAL['bg'])
        low.pack(fill='both', expand=True)
        l_fr = self.controller._card(low, '🚀 Rapid OS-Module Dispatch')
        l_fr.master.pack(side='left', fill='both', expand=True, padx=5)
        ta = scrolledtext.ScrolledText(l_fr, height=10, font=FONT_MONO, bg=PAL['bg'], fg=PAL['text'], bd=0)
        ta.pack(fill='both', expand=True, pady=10)
        ta.insert('1.0', 'I want to build a simple x86_64 hobby operating system... [Stage 1: Boot]')

        def _launch():
            txt = ta.get('1.0', 'end').strip()
            self.controller._log_voice('Architect: Mission Dispatched to Multiboot Agent Swarm.')
            self.controller._notify('Lab', 'Implementation Plan Artifact Generated.', 'OK')
        ttk.Button(l_fr, text='↑ Launch Agent Swarm', command=_launch).pack(side='right')
        ttk.Button(l_fr, text='⚙️ Settings', command=lambda: self.controller._notify('Lab', 'Toolchain: i686-elf-gcc Cross-Compiler Active.', 'INFO')).pack(side='right', padx=5)
        r_fr = self.controller._card(low, '🛠️ Low-Level Toolchain')
        r_fr.master.pack(side='right', fill='both', width=350, padx=5)
        tools = lab.toolchain_status if lab else {}
        for k, v in tools.items():
            f = tk.Frame(r_fr, bg=PAL['card'])
            f.pack(fill='x', pady=4)
            tk.Label(f, text=k.upper(), font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack(side='left')
            tk.Label(f, text=v, font=FONT_BOLD, fg=PAL['teal'] if 'READY' in v else PAL['gold'], bg=PAL['card']).pack(side='right')
        ttk.Button(r_fr, text='INIT TOOLCHAIN', command=lambda: self.controller._notify('Lab', lab.initialize_toolchain() if lab else 'Kernel Link Fail', 'OK')).pack(fill='x', pady=10)