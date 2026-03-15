import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "SOVEREIGN AETHER", "Hyper-Dynamic Kernel Mutation & Federated AI Mesh")
        self.build()

    def build(self):
        main = tk.Frame(self, bg=PAL["bg"])
        main.pack(fill="both", expand=True, padx=40, pady=20)
        
        # Grid layout
        left = tk.Frame(main, bg=PAL["bg"])
        left.pack(side="left", fill="both", expand=True, padx=(0, 20))
        
        right = tk.Frame(main, bg=PAL["bg"])
        right.pack(side="right", fill="both", expand=True)

        # 1. Kernel Mutation Control
        mut_card = self.gui._card(left, "CORE MUTATION (ASLR++)")
        mut_card.master.pack(fill="x", pady=(0, 10))
        
        stats = self.kernel.get_leadership_stats()
        mut_id = stats.get("Mutation_ID", "STABLE_OX_F3")
        
        tk.Label(mut_card, text=f"Active Mutation ID: {mut_id}", font=FONT_MONO, fg=PAL["cyan"], bg=PAL["card"]).pack(pady=10)
        
        ttk.Button(mut_card, text="FORCE KERNEL MUTATION", command=self._mutate).pack(pady=10)

        # 2. Merkle Integrity Audit
        audit_card = self.gui._card(left, "MERKLE TREE INTEGRITY SHIELD")
        audit_card.master.pack(fill="x", pady=(0, 10))
        
        tk.Label(audit_card, text="Validating Ring-0 binaries against Merkle root...", 
                 font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(pady=5)
        
        ttk.Button(audit_card, text="SCAN KERNEL INTEGRITY", command=self._run_audit).pack(pady=10)

        # 3. Federated Intelligence
        intel_card = self.gui._card(right, "FEDERATED KNOWLEDGE DISTILLATION")
        intel_card.master.pack(fill="x", pady=(0, 10))
        
        tk.Label(intel_card, text="Distilling intelligence from local mirrors (Synced) securely.", 
                 font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(pady=5)
        
        ttk.Button(intel_card, text="START LOCAL DISTILLATION", command=self._distill).pack(pady=10)

        # 4. OS Principle: BFT Mesh Telemetry
        mesh_card = self.gui._card(right, "BYZANTINE MESH POWER")
        mesh_card.master.pack(fill="x", pady=(0, 10))
        
        fabric = self.kernel.registry.get("fabric")
        if fabric:
            f_map = fabric.get_fabric_map()
            tk.Label(mesh_card, text=f"Mesh Contrib: {f_map['Mesh_External']}% | Consistency: {f_map['Predictive_HitRate']*100}%", 
                     font=FONT_BOLD, fg=PAL["teal"], bg=PAL["card"]).pack(pady=5)
        
        # 5. AI Principle: Morphic Heat Map
        heat_card = self.gui._card(left, "MORPHIC COGNITIVE HEAT MAP")
        heat_card.master.pack(fill="x", pady=(0, 10))
        
        if fabric:
            heat_map = fabric.get_morphic_heat_map()
            fr = tk.Frame(heat_card, bg=PAL["card"])
            fr.pack(pady=5)
            for sub, color in heat_map.items():
                lbl = tk.Label(fr, text=f"[{sub}]", font=FONT_MONO, fg=PAL["cyan"] if "Stable" in color else PAL["gold"], bg=PAL["card"])
                lbl.pack(side="left", padx=5)

    def _mutate(self):
        new_id = self.kernel.mutate_kernel_state()
        self.gui._notify("AETHER", f"Kernel layout mutated: {new_id}", "OK")
        self.gui._show_page("aether") 

    def _run_audit(self):
        res = self.kernel.verify_merkle_integrity("sigma_core")
        status = "VERIFIED" if res else "INTEGRITY_COMPROMISED"
        self.gui._notify("SECURITY", f"Merkle Audit: {status}", "OK" if res else "ERR")

    def _distill(self):
        distillator = self.kernel.registry.get("neural_distillator")
        if distillator:
            res = distillator.distill_from_mirrors()
            self.gui._notify("AI MESH", f"Distillation: {res}", "INFO")
        else:
            self.gui._notify("AI MESH", "Distillator Offline.", "ERR")
