# Generated method: FabricPage.build
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD

class FabricPage:
    def build(self):
        self.controller._build_page_header(self, 'Mesh Compute Fabric', 'Distributed Sovereign Power')
        mon_fr = tk.Frame(self, bg=PAL['bg'])
        mon_fr.pack(fill='x', pady=10)
        card = self.controller._card(mon_fr, 'Local & Mesh Stats')
        self.fabric_label = tk.Label(card, text='Fabric: IDLE | Compute: 0 TFLOPS', font=FONT_BOLD, fg=PAL['accent'], bg=PAL['card'])
        self.fabric_label.pack(pady=10)

        def refresh():
            if hasattr(self.controller.kernel, 'fabric') and self.controller.kernel.fabric:
                res = self.controller.kernel.fabric.health_check()
                self.fabric_label.config(text=res)
            else:
                self.fabric_label.config(text='Fabric: OFFLINE')
        tk.Button(card, text='Join Hybrid Fabric', bg=PAL['accent'], fg='white', command=lambda: [getattr(self.controller.kernel, 'fabric', None) and self.controller.kernel.fabric.join_compute_fabric(), refresh()]).pack(pady=5)
        refresh()