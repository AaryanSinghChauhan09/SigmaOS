# Generated method: AetherPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage:
    def build(self):
        main = tk.Frame(self, bg=PAL['bg'])
        main.pack(fill='both', expand=True, padx=40, pady=20)
        left = tk.Frame(main, bg=PAL['bg'])
        left.pack(side='left', fill='both', expand=True, padx=(0, 20))
        right = tk.Frame(main, bg=PAL['bg'])
        right.pack(side='right', fill='both', expand=True)
        mut_card = self.gui._card(left, 'CORE MUTATION (ASLR++)')
        mut_card.master.pack(fill='x', pady=(0, 10))
        stats = self.kernel.get_leadership_stats()
        mut_id = stats.get('Mutation_ID', 'STABLE_OX_F3')
        tk.Label(mut_card, text=f'Active Mutation ID: {mut_id}', font=FONT_MONO, fg=PAL['cyan'], bg=PAL['card']).pack(pady=10)
        ttk.Button(mut_card, text='FORCE KERNEL MUTATION', command=self._mutate).pack(pady=10)
        audit_card = self.gui._card(left, 'MERKLE TREE INTEGRITY SHIELD')
        audit_card.master.pack(fill='x', pady=(0, 10))
        tk.Label(audit_card, text='Validating Ring-0 binaries against Merkle root...', font=FONT_SMALL, fg=PAL['text'], bg=PAL['card']).pack(pady=5)
        ttk.Button(audit_card, text='SCAN KERNEL INTEGRITY', command=self._run_audit).pack(pady=10)
        intel_card = self.gui._card(right, 'FEDERATED KNOWLEDGE DISTILLATION')
        intel_card.master.pack(fill='x', pady=(0, 10))
        tk.Label(intel_card, text='Distilling intelligence from local mirrors (Synced) securely.', font=FONT_SMALL, fg=PAL['text'], bg=PAL['card']).pack(pady=5)
        ttk.Button(intel_card, text='START LOCAL DISTILLATION', command=self._distill).pack(pady=10)
        mesh_card = self.gui._card(right, 'BYZANTINE MESH POWER')
        mesh_card.master.pack(fill='x', pady=(0, 10))
        fabric = self.kernel.registry.get('fabric')
        if fabric:
            f_map = fabric.get_fabric_map()
            tk.Label(mesh_card, text=f"Mesh Contrib: {f_map['Mesh_External']}% | Consistency: {f_map['Predictive_HitRate'] * 100}%", font=FONT_BOLD, fg=PAL['teal'], bg=PAL['card']).pack(pady=5)
        heat_card = self.gui._card(left, 'MORPHIC COGNITIVE HEAT MAP')
        heat_card.master.pack(fill='x', pady=(0, 10))
        if fabric:
            heat_map = fabric.get_morphic_heat_map()
            fr = tk.Frame(heat_card, bg=PAL['card'])
            fr.pack(pady=5)
            for sub, color in heat_map.items():
                lbl = tk.Label(fr, text=f'[{sub}]', font=FONT_MONO, fg=PAL['cyan'] if 'Stable' in color else PAL['gold'], bg=PAL['card'])
                lbl.pack(side='left', padx=5)