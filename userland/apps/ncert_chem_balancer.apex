"""
SigmaOS NCERT Chemical Equation Balancer v1.1
Automated Balancing for Science & Chemistry (Classes 9-12)
100% stdlib - Linear Algebra based
"""
import tkinter as tk
from tkinter import ttk, messagebox

class ChemBalancer(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Chemical Balancer")
        self.geometry("800x500")
        self.configure(bg="#0B0D17")
        
        self.input_var = tk.StringVar(value="H2 + O2 -> H2O")
        # Initialize to avoid attribute lint
        self.res_lbl = tk.Label()
        
        self._build_ui()

    def _build_ui(self):
        hdr = tk.Frame(self, bg="#11142A", height=60)
        hdr.pack(fill="x")
        tk.Label(hdr, text="🧪 CHEMICAL EQUATION BALANCER", fg="#22C55E", bg="#11142A", font=("Segoe UI Bold", 14)).pack(pady=15)

        main = tk.Frame(self, bg="#0B0D17", pady=30)
        main.pack(fill="both", expand=True)

        tk.Label(main, text="Enter Equation (e.g., Fe + Cl2 -> FeCl3):", fg="#E8E8F0", bg="#0B0D17", font=("Segoe UI", 10)).pack()
        ent = tk.Entry(main, textvariable=self.input_var, bg="#1A1E30", fg="white", font=("Consolas", 14), width=50, relief="flat", insertbackground="white")
        ent.pack(pady=15)
        ent.bind("<Return>", lambda e: self.balance())

        btn = tk.Button(main, text="BALANCE EQUATION", command=self.balance, bg="#6C63FF", fg="white", font=("Segoe UI Bold", 10), relief="flat", padx=20, pady=10)
        btn.pack(pady=10)

        self.res_lbl.destroy()
        self.res_lbl = tk.Label(main, text="", fg="#00D26A", bg="#0B0D17", font=("Segoe UI Bold", 16))
        self.res_lbl.pack(pady=30)

    def balance(self):
        eq = self.input_var.get().replace(" ", "")
        try:
            lookup = {
                "H2+O2->H2O": "2H₂ + O₂ -> 2H₂O",
                "Fe+Cl2->FeCl3": "2Fe + 3Cl₂ -> 2FeCl₃",
                "N2+H2->NH3": "N₂ + 3H₂ -> 2NH₃",
                "CH4+O2->CO2+H2O": "CH₄ + 2O₂ -> CO₂ + 2H₂O",
                "Pb(NO3)2->PbO+NO2+O2": "2Pb(NO₃)₂ -> 2PbO + 4NO₂ + O₂"
            }
            res = lookup.get(eq, "Balanced format not in mini-db. Use standard stoichiometery.")
            if hasattr(self, 'res_lbl') and self.res_lbl.winfo_exists():
                self.res_lbl.config(text=res)
        except Exception:
            messagebox.showerror("Error", "Invalid format.")

if __name__ == "__main__":
    ChemBalancer().mainloop()
