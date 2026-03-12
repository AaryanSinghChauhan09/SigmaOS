import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD

class AGPhysicsPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "ANTIGRAVITY ENGINE", "Zero-G Physics & Verlet UI Interaction Control")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Instantiate kernel module dynamically
        if not hasattr(self.controller.kernel, 'ag_physics'):
            try:
                from antigravity_engine import SigmaAntigravityEngine
                self.controller.kernel.ag_physics = SigmaAntigravityEngine(self.controller.kernel)
            except Exception: pass
            
        c = self.controller._card(main_panel, "Zero-G Drift Engine Status")
        c.master.pack(fill="x", pady=5)
        
        def _toggle_ag():
            if hasattr(self.controller.kernel, 'ag_physics'):
                new_state = not self.controller.kernel.ag_physics.is_active
                self.controller.kernel.ag_physics.toggle_drift(new_state)
                ag_btn.configure(text=f"Physics Engine: {'ACTIVE 🪐' if new_state else 'STABLE/GRAVITY'}")
                self.controller._notify("Physics Engine", "Drift Protocol updated.", "OK")

        ag_btn = ttk.Button(c, text="Physics Engine: STABLE/GRAVITY", command=_toggle_ag)
        ag_btn.pack(pady=10)
        
        def _gather():
            if hasattr(self.controller, 'trigger_action'):
                self.controller.trigger_action("ag.gather")

        ttk.Button(main_panel, text="⬇️ Trigger Gravity Gather (Pulse)", command=_gather).pack(pady=10)
        
        tk.Label(main_panel, text="Mass Projection: Heavier apps (Store, Enterprise) drift slower than lightweight ones (Aether). Paradox: Higher RAM usage increases 'mass' for the drift engine.",
                 font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["bg"], wraplength=500).pack(pady=20)
