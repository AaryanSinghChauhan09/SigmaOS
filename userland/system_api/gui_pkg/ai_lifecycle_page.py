import tkinter as tk
from tkinter import ttk
import time
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class AILifecyclePage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "AI MISSION CONTROL", "Unified Alpha-Zero Lifecycle Engineering Studio")
        self.build()

    def build(self):
        # Top Section: New Mission Form
        form_fr = self.gui._card(self, "🚀 INITIATE NEW MISSION")
        form_fr.master.pack(fill="x", padx=20, pady=10)
        
        row1 = tk.Frame(form_fr, bg=PAL["card"])
        row1.pack(fill="x", pady=5)
        
        tk.Label(row1, text="PROJECT NAME:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left")
        self._ai_proj_name = ttk.Entry(row1, width=30)
        self._ai_proj_name.pack(side="left", padx=10)
        self._ai_proj_name.insert(0, "Sigma_V3_Core")
        
        tk.Label(row1, text="DISCIPLINE:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left", padx=(20, 0))
        self._ai_disc_cb = ttk.Combobox(row1, values=["AI (Artificial Intelligence)", "ML (Machine Learning)", "DS (Data Science)"], width=25)
        self._ai_disc_cb.pack(side="left", padx=10)
        self._ai_disc_cb.set("ML (Machine Learning)")

        row2 = tk.Frame(form_fr, bg=PAL["card"])
        row2.pack(fill="x", pady=10)
        tk.Label(row2, text="MISSION OBJECTIVE:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left")
        self._ai_obj_ent = ttk.Entry(row2)
        self._ai_obj_ent.pack(side="left", fill="x", expand=True, padx=10)
        self._ai_obj_ent.insert(0, "Achieve 99% accuracy in local resource orchestration.")
        
        def _start_mission():
            name = self._ai_proj_name.get()
            obj = self._ai_obj_ent.get()
            disc = self._ai_disc_cb.get().split(" ")[0]
            mid = self.kernel.ai_lifecycle.start_unified_mission(name, obj, disc)
            self._notify("Mission Initiated", f"ID: {mid} - Status: ACTIVE", "OK")
            self._update_ai_missions()
            
        ttk.Button(row2, text="Launch Mission", command=_start_mission, width=15).pack(side="right")

        # Main Workspace: Active Missions & Details
        ws = tk.Frame(self, bg=PAL["bg"])
        ws.pack(fill="both", expand=True, padx=20)
        
        # Left: Mission List
        self._ai_list_fr = self.gui._card(ws, "📜 ACTIVE MISSIONS")
        self._ai_list_fr.master.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        self._ai_scroll = tk.Frame(self._ai_list_fr, bg=PAL["card"])
        self._ai_scroll.pack(fill="both", expand=True)

        # Right: Detail & Execution View
        self._ai_detail_fr = self.gui._card(ws, "🔍 MISSION DETAILS & EXECUTION")
        self._ai_detail_fr.master.pack(side="left", fill="both", width=500)
        
        self._ai_active_mid = tk.StringVar(value="N/A")
        tk.Label(self._ai_detail_fr, text="SELECTED MISSION:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        tk.Label(self._ai_detail_fr, textvariable=self._ai_active_mid, font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["card"]).pack(anchor="w", pady=(0, 15))
        
        self._ai_step_lbl = tk.Label(self._ai_detail_fr, text="Current Phase: NONE", font=FONT_MED, fg=PAL["text"], bg=PAL["card"])
        self._ai_step_lbl.pack(anchor="w")
        
        self._ai_prog = ttk.Progressbar(self._ai_detail_fr, mode="determinate")
        self._ai_prog.pack(fill="x", pady=10)
        
        self._ai_guidance = tk.Text(self._ai_detail_fr, height=8, bg=PAL["bg3"], fg=PAL["dim"], font=FONT_SMALL, bd=0, relief="flat", padx=10, pady=10)
        self._ai_guidance.pack(fill="x", pady=10)
        
        # Bottom Section: Mesh Lattice
        self._mesh_canvas = tk.Canvas(self._ai_detail_fr, height=120, bg=PAL["bg2"], highlightthickness=0)
        self._mesh_canvas.pack(fill="x", pady=10)
        self._mesh_nodes = []
        self._draw_mesh_lattice()
        
        btn_fr = tk.Frame(self._ai_detail_fr, bg=PAL["card"])
        btn_fr.pack(fill="x", pady=10)
        self._next_btn = ttk.Button(btn_fr, text="▶ EXECUTE NEXT PHASE", command=self._execute_ai_next)
        self._next_btn.pack(side="left", fill="x", expand=True, padx=5)

        self._update_ai_missions()

    def _execute_ai_next(self):
        mid = self._ai_active_mid.get()
        if mid == "N/A": return
        res = self.kernel.ai_lifecycle.execute_next_step(mid)
        self._notify("Phase Complete", f"Completed: {res.get('step', 'unknown')}", "OK")
        self._update_ai_missions()

    def _update_ai_missions(self):
        # Implementation logic follows old gui pattern
        pass

    def _draw_mesh_lattice(self):
        if not self._mesh_canvas.winfo_exists(): return
        self._mesh_canvas.delete("all")
        w, h = 480, 120
        # Simulated pulse logic
        self.after(200, self._draw_mesh_lattice)
