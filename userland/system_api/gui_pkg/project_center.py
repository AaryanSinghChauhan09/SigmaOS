import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ProjectCenterPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "PROJECT ORCHESTRATOR", "Agile Swarm & Resource Planning")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Perspective Switcher
        tab_bar = tk.Frame(body, bg=PAL["bg2"], height=40)
        tab_bar.pack(fill="x", pady=(0, 10))
        tab_bar.pack_propagate(False)
        
        self.view_container = tk.Frame(body, bg=PAL["bg"])
        self.view_container.pack(fill="both", expand=True)
        
        self.views = {}
        
        for name, icon in [("Kanban", "??"), ("Scrum", "??"), ("Gantt", "??"), ("Reports", "??")]:
            b = tk.Button(tab_bar, text=f"{icon} {name}", font=FONT_SMALL, fg=PAL["text"], 
                          bg=PAL["bg2"], relief="flat", padx=15,
                          command=lambda n=name: self._show_view(n))
            b.pack(side="left", fill="y")
            
        self._show_view("Kanban")

    def _show_view(self, name):
        for v in self.views.values(): v.pack_forget()
        
        if name not in self.views:
            v = tk.Frame(self.view_container, bg=PAL["bg"])
            self.views[name] = v
            if name == "Kanban": self._build_kanban(v)
            elif name == "Scrum": self._build_scrum(v)
            else: tk.Label(v, text=f"{name} View: Optimizing for Apex v2.2...", fg=PAL["dim"], bg=PAL["bg"]).pack(expand=True)
            
        self.views[name].pack(fill="both", expand=True)

    def _build_kanban(self, parent):
        cols = ["Backlog", "In Progress", "Review", "Done"]
        for c in cols:
            fr = tk.Frame(parent, bg=PAL["bg2"], width=200)
            fr.pack(side="left", fill="both", expand=True, padx=5)
            tk.Label(fr, text=c, font=FONT_BOLD, fg=PAL["gold"], bg=PAL["bg2"]).pack(pady=10)
            
            # Simulated tasks
            for i in range(2):
                card = self.gui._card(fr, f"Task {c[0]}{i+1}")
                card.master.pack(fill="x", pady=5)
                tk.Label(card, text="Assigned to Agent_01", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")

    def _build_scrum(self, parent):
        tk.Label(parent, text="Sprint 14: Neural Mesh Integration", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(pady=10)
        pb = ttk.Progressbar(parent, value=75, length=400)
        pb.pack(pady=10)
        tk.Label(parent, text="Velocity: 42 pts/sprint | Efficiency: High", font=FONT_SMALL, fg=PAL["teal"], bg=PAL["bg"]).pack()
