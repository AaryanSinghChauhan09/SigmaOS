# Generated method: GovernorPage.build
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_TITLE, FONT_MONO

class GovernorPage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        l_fr = tk.Frame(body, bg=PAL['bg'], width=450)
        l_fr.pack(side='left', fill='both', padx=(0, 10))
        l_fr.pack_propagate(False)
        entropy_c = self.gui._card(l_fr, '🧠 Cognitive Entropy Detector')
        entropy_c.master.pack(fill='x', pady=(0, 10))
        self.ent_lbl = tk.Label(entropy_c, text='ENTROPY: 0.0%', font=FONT_TITLE, fg=PAL['cyan'], bg=PAL['card'])
        self.ent_lbl.pack(pady=10)
        self.rec_lbl = tk.Label(entropy_c, text='Recommendation: Scan required.', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card'], wraplength=400)
        self.rec_lbl.pack(pady=5)

        def _scan():
            res = self.kernel.governor.detect_cognitive_entropy()
            self.ent_lbl.config(text=f"ENTROPY: {res['entropy_level']}")
            self.rec_lbl.config(text=f"Rec: {res['recommendation']}", fg=PAL['gold'])
            self.gui._morphic_island(f"ENTROPY: {res['entropy_level']}", PAL['accent2'])
        ttk.Button(entropy_c, text='Force Cognitive Scan', command=_scan).pack(fill='x', pady=10)
        sync_c = self.gui._card(body, '📱 Sovereign Mind-Sync (Continuity)')
        sync_c.master.pack(side='left', fill='both', expand=True)
        tk.Label(sync_c, text='ACTIVE NODES IN MESH:', font=FONT_BOLD, fg=PAL['teal'], bg=PAL['card']).pack(anchor='w')
        nodes_fr = tk.Frame(sync_c, bg=PAL['card'])
        nodes_fr.pack(fill='x', pady=10)
        sync_mgr = self.kernel.mind_sync
        if sync_mgr:
            for node in sync_mgr.get_floating_sessions():
                f = tk.Frame(nodes_fr, bg=PAL['bg3'], pady=5, padx=10)
                f.pack(fill='x', pady=2)
                tk.Label(f, text=f'🌐 {node}', font=FONT_SMALL, fg=PAL['text'], bg=PAL['bg3']).pack(side='left')
                tk.Label(f, text='SYNCED', font=FONT_SMALL, fg=PAL['green'], bg=PAL['bg3']).pack(side='right')
        tk.Label(sync_c, text='\nShared Universal Clipboard:', font=FONT_BOLD, fg=PAL['dim'], bg=PAL['card']).pack(anchor='w')
        self.clip_ent = ttk.Entry(sync_c)
        self.clip_ent.pack(fill='x', pady=5)
        self.clip_ent.insert(0, 'Copy here to sync across nodes...')

        def _sync_clip():
            msg = sync_mgr.share_clipboard(self.clip_ent.get())
            self.gui._notify('Mind-Sync', msg, 'OK')
            self.gui._update_morphic_status('SYNC', 'PROPAGATED', PAL['teal'])
        ttk.Button(sync_c, text='Propagate to Mesh', command=_sync_clip).pack(pady=10)
        prio_c = self.gui._card(sync_c, '⚖️ Deadlock & Priority Monitor')
        prio_c.pack(fill='x', pady=10)
        tk.Label(prio_c, text='IPC Health: NOMINAL', font=FONT_MONO, fg=PAL['green'], bg=PAL['card']).pack(side='left')
        ttk.Button(prio_c, text='Resolve Inversions', state='disabled').pack(side='right')