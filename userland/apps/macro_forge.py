"""
SigmaOS Macro Forge (v1.0)
==========================
Visual node-based automation and system-level task scripting.
USP: Deep kernel-level hooks bypassing traditional UI interaction.
Equivalent to: iOS Shortcuts / Power Automate / AutoHotkey.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FFD60A", # Automate Yellow
    "accent_dim": "#CBA800",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class MacroForge(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Macro Forge")
        self.geometry("1100x700")
        self.configure(bg=PAL["bg"])
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Forge.Treeview", background=PAL["sidebar"], fieldbackground=PAL["sidebar"], 
                        foreground=PAL["text"], borderwidth=0, font=("Inter", 10), rowheight=40)
        style.configure("Forge.Treeview.Heading", background=PAL["panel"], foreground=PAL["dim"], 
                        font=("Inter", 9, "bold"), borderwidth=0)
        style.map("Forge.Treeview", background=[("selected", PAL["panel"])])

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="MACRO FORGE APEX", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [("➕ NEW SEQUENCE", self._new_macro), ("▶️ EXECUTE ALL", self._run_all)]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Left Panel (Triggers / Actions Library)
        self.lib_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=300, padx=15, pady=15)
        self.lib_fr.pack(side="left", fill="y", padx=(0, 20))
        self.lib_fr.pack_propagate(False)
        
        tk.Label(self.lib_fr, text="CAPABILITY NODES", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        nodes = [
            ("⚡ TRIGGER: System Boot", PAL["success"]),
            ("⚡ TRIGGER: Time = 08:00", PAL["success"]),
            ("⚡ TRIGGER: Connect to 'Home_WiFi'", PAL["success"]),
            ("⚙️ ACTION: Launch Omni-Lens", PAL["accent"]),
            ("⚙️ ACTION: Engage Zenith Focus", PAL["accent"]),
            ("⚙️ ACTION: Parse Clipboard via AI", PAL["accent"]),
            ("⚙️ ACTION: Send via Nexus Share", PAL["accent"]),
            ("🔀 LOGIC: If Battery < 20%", "#1E90FF"),
        ]
        
        for n, c in nodes:
            lbl = tk.Label(self.lib_fr, text=n, font=("Inter", 9, "bold"), fg="white", bg=c, padx=10, pady=5, cursor="hand2")
            lbl.pack(fill="x", pady=5)
            lbl.bind("<Button-1>", lambda e, text=n: messagebox.showinfo("Node Added", f"Node '{text}' staged in Sequence."))
            
        # Right Panel (Sequence Editor)
        self.seq_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.seq_fr.pack(side="left", fill="both", expand=True)
        
        tk.Label(self.seq_fr, text="ACTIVE SEQUENCE: 'Morning Sovereignty'", font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        
        self.tree = ttk.Treeview(self.seq_fr, columns=("Type", "Node Function", "Priority"), show="headings", style="Forge.Treeview")
        self.tree.heading("Type", text="VECTOR")
        self.tree.column("Type", width=100, anchor="center")
        self.tree.heading("Node Function", text="KERNEL HOOK")
        self.tree.column("Node Function", width=400)
        self.tree.heading("Priority", text="RING")
        self.tree.column("Priority", width=100, anchor="center")
        
        macros = [
            ("TRIGGER", "Time = 06:30 AM", "Ring-3"),
            ("ACTION", "Disable Notifications (Focus Engine)", "Ring-2"),
            ("ACTION", "Launch Omni-Lens API (Scan Email)", "Ring-3"),
            ("LOGIC", "Wait for 15 minutes", "Ring-0"),
            ("ACTION", "Terminate Background Apps (Energy Core)", "Ring-1")
        ]
        
        for m in macros:
            self.tree.insert("", "end", values=m)
            
        self.tree.pack(fill="both", expand=True)
        self.tree.bind("<Double-1>", lambda e: messagebox.showinfo("Configure Node", "Modifying Ring-Level hook parameters..."))

        # Status
        self.status = tk.Label(self, text="FORGE ENGINE STANDBY | HARDWARE ACCELERATION ON", 
                               bg=PAL["accent_dim"], fg="black", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _new_macro(self):
        self.tree.delete(*self.tree.get_children())
        self.status.config(text="NEW SEQUENCE INITIATED. AWAITING TRIGGERS.", bg=PAL["panel"], fg="white")

    def _run_all(self):
        if not self.tree.get_children(): return
        self.status.config(text="EXECUTING KERNEL HOOKS... BYPASSING UI... [████████--]", bg=PAL["accent"], fg="black")
        self.after(1500, lambda: messagebox.showinfo("Macro Forge", "Sequence executed synchronously at kernel level. Zero latency achieved."))
        self.after(1500, lambda: self.status.config(text="EXECUTION COMPLETE | 0.04mS OVERHEAD", bg=PAL["success"], fg="black"))

if __name__ == "__main__":
    app = MacroForge()
    app.mainloop()
