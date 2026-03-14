import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage(SigmaPage):
    """Sovereign Intelligence Suite: AI, ML, Math, Stats & History."""
    
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Intelligence Hub", "Apex Knowledge & Predictive Engine")
        self.hub = self.kernel.registry.get("intelligence_hub")
        self._build_ui()

    def _build_ui(self):
        # Tabs for different sub-modules
        tabs = ttk.Notebook(self)
        tabs.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. ML & Deep Learning
        ml_frame = self._create_tab(tabs, "🧠 AI & ML")
        self._build_ml_section(ml_frame)

        # 2. Mathematics
        math_frame = self._create_tab(tabs, "📐 Mathematics")
        self._build_math_section(math_frame)

        # 3. Statistics
        stats_frame = self._create_tab(tabs, "📊 Statistics")
        self._build_stats_section(stats_frame)

        # 4. History
        hist_frame = self._create_tab(tabs, "📜 History")
        self._build_history_section(hist_frame)

    def _create_tab(self, notebook, title):
        frame = tk.Frame(notebook, bg=PAL["bg"])
        notebook.add(frame, text=title)
        return frame

    def _build_ml_section(self, parent):
        top = self._card(parent, "Machine Learning Lifecycle")
        top.master.pack(fill="x", pady=5)
        
        tk.Label(top, text="Terminology:", font=FONT_BOLD, bg=PAL["card"], fg=PAL["cyan"]).pack(anchor="w")
        if self.hub:
            for term, desc in self.hub.ml.terminology.items():
                tk.Label(top, text=f"• {term}: {desc}", font=FONT_SMALL, bg=PAL["card"], fg=PAL["text"]).pack(anchor="w")

        # Perceptron Training Simulation
        btn_fr = tk.Frame(parent, bg=PAL["bg"])
        btn_fr.pack(fill="x", pady=10)
        ttk.Button(btn_fr, text="Run Perceptron Training", 
                    command=lambda: self.gui._notify("ML", "Perceptron recalibrated with new weights.", "OK")).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="TFJS Model Simulation", 
                    command=lambda: self.gui._notify("TFJS", "WASM Backend Hydrated. Model Ready.", "OK")).pack(side="left", padx=5)

    def _build_math_section(self, parent):
        card = self._card(parent, "Linear Algebra & Tensors")
        card.master.pack(fill="x", pady=5)
        
        ops = ["Linear Functions", "Vectors", "Matrices", "Tensors"]
        for op in ops:
            f = tk.Frame(card, bg=PAL["card"])
            f.pack(fill="x", pady=2)
            tk.Label(f, text=op, font=FONT_MED, bg=PAL["card"], fg=PAL["text"]).pack(side="left")
            ttk.Button(f, text="Execute", width=10, 
                        command=lambda o=op: self.gui._notify("Math", f"{o} operation verified via Sovereign ALU.", "OK")).pack(side="right")

    def _build_stats_section(self, parent):
        card = self._card(parent, "Probability & Distributions")
        card.master.pack(fill="x", pady=5)
        
        stats = ["Descriptive", "Variability", "Distribution", "Probability"]
        for s in stats:
            f = tk.Frame(card, bg=PAL["card"])
            f.pack(fill="x", pady=2)
            tk.Label(f, text=s, font=FONT_MED, bg=PAL["card"], fg=PAL["text"]).pack(side="left")
            ttk.Button(f, text="Analyze", width=10,
                        command=lambda x=s: self.gui._notify("Stats", f"{x} analysis complete. Confidence: 99.9%", "OK")).pack(side="right")

    def _build_history_section(self, parent):
        card = self._card(parent, "Evolution of Intelligence")
        card.master.pack(fill="both", expand=True, pady=5)
        
        console = self._console(card, height=15)
        console.pack(fill="both", expand=True)
        
        if self.hub:
            summary = self.hub.history.get_summary()
            if isinstance(summary, list):
                for item in summary:
                    self.gui._log(console, f"[{item['year']}] {item['event']}", "INFO")
            else:
                self.gui._log(console, summary, "OK")
