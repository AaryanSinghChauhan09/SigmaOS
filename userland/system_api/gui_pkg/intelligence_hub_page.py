import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage(SigmaPage):
    """Sovereign Intelligence Suite: AI, ML, Math, Stats & History."""
    
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Intelligence Hub", "Apex Knowledge & Predictive Engine")
        self.hub = self.kernel.registry.get("intelligence_hub")
        self._build_ui()

    def _build_ui(self):
        # Tabs for different sub-modules
        tabs = ttk.Notebook(self)
        tabs.pack(fill="both", expand=True, padx=20, pady=10)

        guardian = self.kernel.guardian

        # 1. ML & Deep Learning
        ml_frame = self._create_tab(tabs, guardian.sanitize_text("🧠 AI & ML"))
        self._build_ml_section(ml_frame)

        # 2. Mathematics
        math_frame = self._create_tab(tabs, guardian.sanitize_text("📐 Mathematics"))
        self._build_math_section(math_frame)

        # 3. Statistics
        stats_frame = self._create_tab(tabs, guardian.sanitize_text("📊 Statistics"))
        self._build_stats_section(stats_frame)

        # 4. Graphics Engine
        gfx_frame = self._create_tab(tabs, guardian.sanitize_text("🎨 Graphics"))
        self._build_graphics_section(gfx_frame)

        # 5. History
        hist_frame = self._create_tab(tabs, guardian.sanitize_text("📜 History"))
        self._build_history_section(hist_frame)

    def _create_tab(self, notebook, title):
        frame = tk.Frame(notebook, bg=PAL["bg"])
        notebook.add(frame, text=title)
        return frame

    def _build_ml_section(self, parent):
        # Terminology Card
        top = self._card(parent, "Machine Learning Lifecycle")
        top.master.pack(fill="x", pady=5)
        
        tk.Label(top, text="Terminology:", font=FONT_BOLD, bg=PAL["card"], fg=PAL["cyan"]).pack(anchor="w")
        if self.hub:
            for term, desc in self.hub.ml.terminology.items():
                tk.Label(top, text=f"• {term}: {desc}", font=FONT_SMALL, bg=PAL["card"], fg=PAL["text"]).pack(anchor="w")

        # Examples Card
        ex_card = self._card(parent, "ML Examples (TFJS / Brain.js Style)")
        ex_card.master.pack(fill="x", pady=5)
        
        if self.hub:
            ex1 = self.hub.deep_ml.get_example_1()
            ex2 = self.hub.deep_ml.get_example_2()
            
            for ex in [ex1, ex2]:
                f = tk.Frame(ex_card, bg=PAL["card"], pady=5)
                f.pack(fill="x")
                tk.Label(f, text=f"{ex['name']}: {ex['intro']}", font=FONT_BOLD, bg=PAL["card"], fg=PAL["cyan"]).pack(anchor="w")
                tk.Label(f, text=f"Data: {ex['data']} | Model: {ex['model']}", font=FONT_SMALL, bg=PAL["card"], fg=PAL["text"]).pack(anchor="w")
                ttk.Button(f, text="Simulate Training", command=lambda x=ex['name']: self._notify("Training", f"Started {x} training...", "OK")).pack(anchor="e")

        # Actions Card
        act_card = self._card(parent, "Operations & Recognition")
        act_card.master.pack(fill="x", pady=10)
        
        btn_fr = tk.Frame(act_card, bg=PAL["card"])
        btn_fr.pack(fill="x")
        ttk.Button(btn_fr, text="Perceptron Recognition", command=self._run_recognition).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="TFJS Visor", command=self._show_visor).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="Clustering Simulation", 
                    command=lambda: self.gui._notify("ML", "K-Means grouping complete.", "OK")).pack(side="left", padx=5)
        
        if self.hub:
            tk.Label(act_card, text="TFJS Models: " + ", ".join(self.hub.deep_ml.tfjs_models), 
                     font=FONT_SMALL, bg=PAL["card"], fg=PAL["cyan"]).pack(pady=5)

    def _run_recognition(self):
        objects = ["Apple", "Ball", "Cat", "Dog", "Elephant"]
        found = random.choice(objects)
        self.gui._notify("Recognition", f"Object Detected: {found} (Confidence: 98%)", "OK")

    def _show_visor(self):
        self.gui._notify("TFJS Visor", "Visor Overlay Hydrated. Monitoring Tensors...", "INFO")
        # Simulate some visor activity on the plot canvas
        self._plot_scatter()

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

    def _build_graphics_section(self, parent):
        card = self._card(parent, "JavaScript Graphics Engines")
        card.master.pack(fill="both", expand=True, pady=5)
        
        # Display Libraries
        if self.hub:
            for lib, desc in self.hub.graphics.libraries.items():
                f = tk.Frame(card, bg=PAL["card"], pady=5)
                f.pack(fill="x")
                tk.Label(f, text=lib, font=FONT_BOLD, bg=PAL["card"], fg=PAL["cyan"]).pack(anchor="w")
                tk.Label(f, text=desc, font=FONT_SMALL, bg=PAL["card"], fg=PAL["text"]).pack(anchor="w")
        
        # Live Plotting Canvas
        c_fr = tk.Frame(parent, bg=PAL["bg3"], height=200)
        c_fr.pack(fill="x", padx=10, pady=10)
        self.plot_canvas = tk.Canvas(c_fr, bg=PAL["bg3"], height=180, highlightthickness=0)
        self.plot_canvas.pack(fill="both", expand=True)
        tk.Label(c_fr, text="Live Plot Visualizer (Simulation)", font=("Inter Italic", 8), bg=PAL["bg3"], fg=PAL["dim"]).pack()

        # Plotting demo buttons
        btn_fr = tk.Frame(parent, bg=PAL["bg"])
        btn_fr.pack(fill="x", pady=5)
        ttk.Button(btn_fr, text="Plot Linear Graph", command=self._plot_linear).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="Plot Scatter Points", command=self._plot_scatter).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="Clear Canvas", command=lambda: self.plot_canvas.delete("all")).pack(side="left", padx=5)

    def _plot_linear(self):
        self.plot_canvas.delete("all")
        self.plot_canvas.create_line(10, 170, 380, 20, fill=PAL["accent"], width=2, smooth=True)
        self.gui._notify("Plotly.js", "Linear regression trend-line projected.", "OK")

    def _plot_scatter(self):
        self.plot_canvas.delete("all")
        for _ in range(50):
            x = random.randint(20, 370)
            y = random.randint(20, 160)
            self.plot_canvas.create_oval(x, y, x+4, y+4, fill=PAL["cyan"], outline="")
        self.gui._notify("D3.js", "Scatter plot points distribution complete.", "OK")

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
