"""
SigmaOS Diksha Digital Portal v1.0
===================================
USP: Personalized Digital Learning, QR Interop, and Textbook Library.
Integrated into SigmaOS Education Ecosystem.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import json

class DikshaPortal(tk.Toplevel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.title("SigmaOS • Diksha Academic Portal")
        self.geometry("1000x700")
        self.configure(bg="#0A0B10")
        
        self.styles = {
            "bg": "#0A0B10",
            "accent": "#4F46E5",
            "card": "#1E1B4B",
            "text": "#F8FAFC"
        }
        
        self._build_interface()

    def _build_interface(self):
        # Header
        head = tk.Frame(self, bg=self.styles["accent"], height=100)
        head.pack(fill="x")
        tk.Label(head, text="DIKSHA DIGITAL LEARNING", font=("Segoe UI", 20, "bold"), fg="white", bg=self.styles["accent"]).pack(pady=25)
        
        # Content
        main = tk.Frame(self, bg=self.styles["bg"], padx=40, pady=40)
        main.pack(fill="both", expand=True)
        
        # Categories
        cat_fr = tk.Frame(main, bg=self.styles["bg"])
        cat_fr.pack(fill="x", pady=20)
        
        cats = [
            ("Digital Textbooks", "Access 1-12 NCERT Library"),
            ("Smart QR Linker", "Scan simulation codes"),
            ("Teacher Hub", "Resources & Lesson Plans"),
            ("Student Progress", "Performance Analytics")
        ]
        
        for name, desc in cats:
            c = tk.Frame(main, bg=self.styles["card"], padx=20, pady=20, highlightthickness=1, highlightbackground="#312E81")
            c.pack(fill="x", pady=10)
            
            tk.Label(c, text=name, font=("Segoe UI Bold", 14), fg=self.styles["accent"], bg=self.styles["card"]).pack(anchor="w")
            tk.Label(c, text=desc, font=("Segoe UI", 10), fg="#94A3B8", bg=self.styles["card"]).pack(anchor="w")
            
            tk.Button(c, text="OPEN MODULE", bg=self.styles["accent"], fg="white", relief="flat", padx=15, 
                      command=lambda n=name: self._launch_feature(n)).pack(side="right", pady=( -40, 0))

    def _launch_feature(self, name):
        if "QR" in name:
            self._simulate_qr()
        elif "Textbooks" in name:
            messagebox.showinfo("Library", "Hydrating Offline NCERT Repository... (Simulated)")
        else:
            messagebox.showinfo("Portal", f"Launching {name} via Sovereign Cloud.")

    def _simulate_qr(self):
        qr_win = tk.Toplevel(self)
        qr_win.title("Smart QR Scanner")
        qr_win.geometry("400x400")
        qr_win.configure(bg="#000")
        
        tk.Label(qr_win, text="SIMULATED CAMERA FEED", fg="white", bg="#222", height=15).pack(fill="x", pady=20)
        
        code_ent = tk.Entry(qr_win, bg="#111", fg="lime", insertbackground="lime")
        code_ent.pack(pady=10)
        code_ent.insert(0, "NCERT-B12-TITRATION")
        
        def mock_scan():
            code = code_ent.get()
            messagebox.showinfo("QR Scanned", f"Linked to Research Experiment: {code}")
            qr_win.destroy()
            
        tk.Button(qr_win, text="RESOLVE QR CODE", command=mock_scan, bg="lime", fg="black", relief="flat").pack(pady=10)

if __name__ == "__main__":
    root = tk.Tk()
    root.withdraw()
    DikshaPortal()
    root.mainloop()
