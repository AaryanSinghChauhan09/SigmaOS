"""
SigmaOS NCERT Logic Circuit Simulator v1.2
Interactive Gate Simulation for Physics & CS (Classes 11-12)
100% stdlib/tkinter
"""
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Logic Circuit Simulator")
        self.geometry("900x650")
        self.configure(bg="#0B0D17")
        self.sim_state = {"A": 0, "B": 0, "Gate": "AND"}
        self._build_ui()

    def _build_ui(self):
        for w in self.winfo_children(): w.destroy()
        
        hdr = tk.Frame(self, bg="#11142A", height=60)
        hdr.pack(fill="x")
        tk.Label(hdr, text="🔌 INTERACTIVE LOGIC GATES PRO", fg="#F59E0B", bg="#11142A", font=("Segoe UI Bold", 14)).pack(pady=15)

        canv_fr = tk.Frame(self, bg="#0B0D17")
        canv_fr.pack(fill="both", expand=True)
        self.canvas = tk.Canvas(canv_fr, bg="#0D0F18", highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, padx=40, pady=40)

        ctrl = tk.Frame(self, bg="#11142A", height=100)
        ctrl.pack(fill="x")

        tk.Label(ctrl, text="Input A:", fg="white", bg="#11142A").pack(side="left", padx=10)
        self.btn_a = tk.Button(ctrl, text=str(self.sim_state["A"]), width=4, command=lambda: self._toggle("A"), bg="#1A1E30", fg="white", relief="flat")
        self.btn_a.pack(side="left", padx=5)

        tk.Label(ctrl, text="Input B:", fg="white", bg="#11142A").pack(side="left", padx=10)
        self.btn_b = tk.Button(ctrl, text=str(self.sim_state["B"]), width=4, command=lambda: self._toggle("B"), bg="#1A1E30", fg="white", relief="flat")
        self.btn_b.pack(side="left", padx=5)

        tk.Label(ctrl, text="Select Gate:", fg="white", bg="#11142A").pack(side="left", padx=20)
        self.gate_cb = ttk.Combobox(ctrl, values=["AND", "OR", "NAND", "NOR", "XOR", "NOT"], width=10, state="readonly")
        self.gate_cb.set(self.sim_state.get("Gate", "AND"))
        self.gate_cb.pack(side="left", padx=5)
        self.gate_cb.bind("<<ComboboxSelected>>", lambda e: self._update())
        
        tk.Button(ctrl, text="VIEW TRUTH TABLE", command=self._show_truth, bg="#6C63FF", fg="white", relief="flat", padx=15).pack(side="right", padx=30)
        self._update()

    def _toggle(self, var):
        self.sim_state[var] = 1 - int(self.sim_state[var])
        if var == "A": self.btn_a.config(text=str(self.sim_state["A"]))
        else: self.btn_b.config(text=str(self.sim_state["B"]))
        self._update()

    def _update(self):
        self.canvas.delete("all")
        a, b = int(self.sim_state["A"]), int(self.sim_state["B"])
        g = self.gate_cb.get()
        self.sim_state["Gate"] = g
        
        # Calculate result
        if g == "AND": res = a and b
        elif g == "OR": res = a or b
        elif g == "NAND": res = not (a and b)
        elif g == "NOR": res = not (a or b)
        elif g == "XOR": res = a != b
        elif g == "NOT": res = not a
        else: res = 0
        res = int(res)

        # Draw Inputs
        color_a = "#00D26A" if a else "#FF4D4D"
        color_b = "#00D26A" if b else "#FF4D4D"
        color_res = "#00D26A" if res else "#FF4D4D"

        self.canvas.create_line(50, 150, 200, 150, fill=color_a, width=4)
        if g != "NOT":
            self.canvas.create_line(50, 250, 200, 250, fill=color_b, width=4)
        
        # Draw Gate Body
        self.canvas.create_rectangle(200, 120, 350, 280, fill="#1A1E30", outline="white", width=2)
        self.canvas.create_text(275, 200, text=g, fill="white", font=("Segoe UI Bold", 16))

        # Draw Output
        self.canvas.create_line(350, 200, 500, 200, fill=color_res, width=4)
        self.canvas.create_oval(510, 180, 550, 220, fill=color_res, outline="white")
        self.canvas.create_text(530, 200, text=str(res), fill="black", font=("Segoe UI Bold", 12))

    def _show_truth(self):
        g = self.gate_cb.get()
        table = f"TRUTH TABLE: {g}\n A | B | OUT\n-----------\n"
        for ia in [0, 1]:
            for ib in ([0, 1] if g != "NOT" else [0]):
                if g == "AND": r = ia and ib
                elif g == "OR": r = ia or ib
                elif g == "NAND": r = not (ia and ib)
                elif g == "NOR": r = not (ia or ib)
                elif g == "XOR": r = ia != ib
                elif g == "NOT": r = not ia
                else: r = 0
                table += f" {ia} | {'-' if g=='NOT' else ib} | {int(r)}\n"
        messagebox.showinfo("Logic Lab", table)

if __name__ == "__main__":
    LogicSimulator().mainloop()
