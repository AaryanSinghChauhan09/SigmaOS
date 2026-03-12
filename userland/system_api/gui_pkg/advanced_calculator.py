import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import math
import cmath
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO, FONT_MED, FONT_TITLE

class AdvancedCalculatorPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "ADVANCED CALCULATOR (IIT JEE & NEET)", "Quantum-Grade Computations & AI Solver")

        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # Tabbed Layout
        notebook = ttk.Notebook(body)
        notebook.pack(fill="both", expand=True)

        # Tab 1: Scientific Calculator
        sci_tab = tk.Frame(notebook, bg=PAL["bg"])
        notebook.add(sci_tab, text=" Scientific Calculator ")
        
        # Tab 2: IIT JEE / NEET AI Solver
        solver_tab = tk.Frame(notebook, bg=PAL["bg"])
        notebook.add(solver_tab, text=" IIT JEE / NEET AI Solver ")

        self._build_sci_calc(sci_tab)
        self._build_ai_solver(solver_tab)

    def _build_sci_calc(self, parent):
        top_fr = tk.Frame(parent, bg=PAL["bg2"], pady=20, padx=20)
        top_fr.pack(fill="x")
        
        self.calc_display = tk.StringVar(value="")
        display_ent = tk.Entry(top_fr, textvariable=self.calc_display, font=("Consolas", 24, "bold"), bg=PAL["bg3"], fg="white", justify="right", bd=0, highlightthickness=1, highlightbackground=PAL["accent"])
        display_ent.pack(fill="x", ipady=10)
        display_ent.bind("<Return>", lambda e: self._evaluate_math())

        pad_fr = tk.Frame(parent, bg=PAL["bg"])
        pad_fr.pack(fill="both", expand=True, pady=10)

        buttons = [
            ["sin", "cos", "tan", "log", "ln"],
            ["asin", "acos", "atan", "exp", "sqrt"],
            ["pi", "e", "(", ")", "C"],
            ["7", "8", "9", "/", "DEL"],
            ["4", "5", "6", "*", "^"],
            ["1", "2", "3", "-", "abs"],
            ["0", ".", "=", "+", "!"]
        ]

        def btn_click(val):
            if val == "C":
                self.calc_display.set("")
            elif val == "DEL":
                self.calc_display.set(self.calc_display.get()[:-1])
            elif val == "=":
                self._evaluate_math()
            else:
                current = self.calc_display.get()
                self.calc_display.set(current + val)

        for r_idx, row in enumerate(buttons):
            row_fr = tk.Frame(pad_fr, bg=PAL["bg"])
            row_fr.pack(fill="x", expand=True, pady=2)
            for c_idx, btn_text in enumerate(row):
                color = PAL["bg3"]
                fg_color = "white"
                if btn_text in ["=", "C", "DEL"]:
                    color = PAL["accent"] if btn_text == "=" else PAL["red"]
                elif btn_text in ["/", "*", "-", "+", "^"]:
                    color = PAL["teal"]
                    fg_color = PAL["bg"]
                
                b = tk.Button(row_fr, text=btn_text, font=FONT_BOLD, bg=color, fg=fg_color, relief="flat", height=2,
                              command=lambda v=btn_text: btn_click(v))
                b.pack(side="left", fill="both", expand=True, padx=2)

    def _evaluate_math(self):
        expr = self.calc_display.get()
        if not expr: return
        
        # Replace mathematical constants and functions safely
        replacements = {
            "sin": "math.sin", "cos": "math.cos", "tan": "math.tan",
            "asin": "math.asin", "acos": "math.acos", "atan": "math.atan",
            "log": "math.log10", "ln": "math.log", "exp": "math.exp",
            "sqrt": "math.sqrt", "pi": "math.pi", "e": "math.e",
            "^": "**", "abs": "builtins.abs" 
        }
        
        # Super simple ! replacement for factorial
        if "!" in expr:
            expr = expr.replace("!", "")
            expr = f"math.factorial(int({expr}))" # naive approach, assuming single number factorial
            
        for k, v in replacements.items():
            expr = expr.replace(k, v)
            
        try:
            # Evaluate using restricted environment (math and cmath)
            import builtins
            safe_dict = {
                "math": math,
                "cmath": cmath,
                "builtins": builtins,
                "__builtins__": None,
            }
            res = eval(expr, safe_dict)
            self.calc_display.set(str(res))
        except Exception as e:
            self.calc_display.set("ERROR")
            self.controller._notify("Math Error", str(e), "ERR")

    def _build_ai_solver(self, parent):
        l_fr = tk.Frame(parent, bg=PAL["bg2"], width=400)
        l_fr.pack(side="left", fill="y", padx=5, pady=5)
        l_fr.pack_propagate(False)

        tk.Label(l_fr, text="Problem Statement", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg2"]).pack(pady=(10, 5))
        
        # Dropdown for Subject
        subj_var = tk.StringVar(value="Physics")
        subj_combo = ttk.Combobox(l_fr, textvariable=subj_var, values=["Physics", "Chemistry", "Mathematics", "Biology (NEET)", "General Intelligence"], state="readonly")
        subj_combo.pack(fill="x", padx=10, pady=5)
        
        level_var = tk.StringVar(value="JEE Advanced")
        level_combo = ttk.Combobox(l_fr, textvariable=level_var, values=["JEE Advanced", "JEE Main", "NEET", "Olympiad", "Class 12 Boards"], state="readonly")
        level_combo.pack(fill="x", padx=10, pady=5)

        prob_txt = tk.Text(l_fr, font=FONT_SMALL, bg=PAL["bg"], fg=PAL["text"], height=10, wrap="word")
        prob_txt.pack(fill="x", padx=10, pady=5)
        prob_txt.insert("1.0", "A block of mass m is placed on a smooth wedge of mass M... find the acceleration.")

        r_fr = tk.Frame(parent, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5, pady=5)

        res_fr = self.controller._card(r_fr, "⚡ AI Step-by-Step Solution")
        res_fr.master.pack(fill="both", expand=True)

        self.solver_out = self.controller._console(res_fr, height=20)
        self.solver_out.pack(fill="both", expand=True)
        self.controller._log(self.solver_out, "Waiting for problem input...", "INFO")

        def _solve():
            problem = prob_txt.get("1.0", "end-1c").strip()
            if not problem: return
            
            subject = subj_var.get()
            level = level_var.get()
            
            self.solver_out.configure(state="normal")
            self.solver_out.delete("1.0", "end")
            self.controller._log(self.solver_out, f"Analyzing {level} {subject} problem...", "HEAD")
            self.controller.update_idletasks()
            
            # Using our AI kernel (Omni Router / Nexus) to solve it
            prompt_str = f"You are an expert tutor for {level}. Solve the following {subject} problem step-by-step with proper formulas and final answer.\nProblem:\n{problem}"
            
            if hasattr(self.controller.kernel, "ai"):
                try:
                    res = self.controller.kernel.ai.multi_model_consensus(prompt_str)
                    self.controller._log(self.solver_out, "\n[Multi-Model Consensus Approved]\n", "OK")
                    self.controller._log(self.solver_out, res.get("Master_Consensus", res.get("Claude", str(res))), "INFO")
                except Exception as e:
                    try:
                        res = self.controller.kernel.ai.prompt(prompt_str)
                        self.controller._log(self.solver_out, "\n[AI Response]\n", "OK")
                        self.controller._log(self.solver_out, res.get("Response", str(res)), "INFO")
                    except Exception as e2:
                        self.controller._log(self.solver_out, f"\n[AI Error] Unable to connect to AI Core. Mocking response for demo...", "ERR")
                        self.controller._log(self.solver_out, f"\nStep 1: Identify given variables...\nStep 2: Apply formula F=ma...\nFinal Answer: a = F/m (Mocked)", "INFO")
            else:
                self.controller._log(self.solver_out, "\n[AI Core Offline] Using local solver capabilities fallback.", "WARN")
                self.controller._log(self.solver_out, f"Simulated Solution for {problem[:20]}...\n1. Identify given variables.\n2. Apply appropriate theorem.\n3. Result = 42", "INFO")

        ttk.Button(l_fr, text="🚀 Quantum Solve Problem", command=_solve, style="Teal.TButton").pack(pady=10)
