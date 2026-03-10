"""
SigmaOS Sovereign MeshTalk Apex Pro (v3.0)
==========================================
Encrypted, P2P communication for SigmaOS.
USP: Neural Decryption Shims & Aether Mesh Pulse Visualization.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

PAL = {
    "bg": "#0B0C0F",
    "sidebar": "#16181D",
    "accent": "#AF52DE", # Deep Purple
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "border": "#2C2F38",
    "chat_bg": "#12131A"
}

class MeshTalk(tk.Toplevel):
    def __init__(self, master=None):
        super().__init__(master)
        self.title("Sovereign MeshTalk Apex Pro")
        self.geometry("1150x800")
        self.config(bg=PAL["bg"])
        
        self._setup_styles()
        self._build_ui()
        
        self._add_log("AETHER_MESH", "P2P QUANTUM TUNNEL ESTABLISHED.", color=PAL["success"])
        self._add_log("NODE_712", "Sovereignty protocol active. Decryption keys localized.")

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Mesh.TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("Mesh.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], 
                        padding=[15, 8], font=("Inter", 9, "bold"))
        style.map("Mesh.TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # 1. Main Paned Layout
        self.panes = ttk.PanedWindow(self, orient="horizontal")
        self.panes.pack(fill="both", expand=True)

        # Sidebar: Channels & Nodes
        self.sidebar = tk.Frame(self.panes, bg=PAL["sidebar"], width=240, padx=20, pady=25)
        self.panes.add(self.sidebar, weight=1)
        self.sidebar.pack_propagate(False)

        tk.Label(self.sidebar, text="AETHER MESH", font=("Inter", 12, "bold"), fg=PAL["accent"], bg=PAL["sidebar"]).pack(anchor="w")
        
        tk.Label(self.sidebar, text="SECURE CHANNELS", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"], pady=(25, 10)).pack(anchor="w")
        for node in ["#general-mesh", "#dev-nodes", "#quantum-sync", "#aether-chat"]:
            tk.Label(self.sidebar, text=node, font=("Inter", 10), fg=PAL["text"], 
                     bg=PAL["sidebar"], pady=8, cursor="hand2").pack(anchor="w")

        tk.Label(self.sidebar, text="NODES ONLINE", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"], pady=(25, 10)).pack(anchor="w")
        for user in [("● Admin_Prime", PAL["success"]), ("● Mesh_Node_A", PAL["success"]), ("● Ghost_7", PAL["dim"])]:
             tk.Label(self.sidebar, text=user[0], font=("Inter", 9), fg=user[1], 
                      bg=PAL["sidebar"], pady=5).pack(anchor="w")

        # Chat Area (Vertical Panes)
        self.chat_pane = ttk.PanedWindow(self.panes, orient="vertical")
        self.panes.add(self.chat_pane, weight=4)

        # Chat Header
        self.header = tk.Frame(self.chat_pane, bg=PAL["bg"], height=60, padx=25, pady=20)
        self.chat_pane.add(self.header, weight=1)
        tk.Label(self.header, text="#general-mesh", font=("Inter", 14, "bold"), fg=PAL["text"], bg=PAL["bg"]).pack(side="left")
        tk.Label(self.header, text="| P2P Tunnel: ENCRYPTED (AES-GCM)", font=("Inter", 9), fg=PAL["success"], bg=PAL["bg"]).pack(side="left", padx=15, pady=8)

        # Chat Log
        self.log_fr = tk.Frame(self.chat_pane, bg=PAL["chat_bg"])
        self.chat_pane.add(self.log_fr, weight=6)
        
        self.chat_log = scrolledtext.ScrolledText(self.log_fr, bg=PAL["chat_bg"], fg=PAL["text"], font=("Inter", 11), 
                                                 state="disabled", borderwidth=0, padx=30, pady=30, insertbackground="white")
        self.chat_log.pack(fill="both", expand=True)

        # Message Entry
        self.entry_fr = tk.Frame(self.chat_pane, bg=PAL["bg"], pady=20, padx=25)
        self.chat_pane.add(self.entry_fr, weight=2)
        
        self.msg_var = tk.StringVar()
        self.entry = tk.Entry(self.entry_fr, textvariable=self.msg_var, bg="#000", fg=PAL["text"], 
                             font=("Inter", 11), insertbackground="white", borderwidth=0, 
                             highlightthickness=1, highlightbackground=PAL["border"], padx=15)
        self.entry.pack(fill="both", pady=5)
        self.entry.bind("<Return>", self.send_message)
        
        tk.Label(self.entry_fr, text="Neural Decryption: SHIM_ACTIVE | P2P LATENCY: 1.2ms", 
                 font=("Inter", 7, "bold"), fg=PAL["dim"], bg=PAL["bg"], pady=5).pack(anchor="w")

        # Right Toolbar (Analysis)
        self.right_bar = tk.Frame(self.panes, bg=PAL["sidebar"], width=260, padx=20, pady=25)
        self.panes.add(self.right_bar, weight=1)
        self.right_bar.pack_propagate(False)

        tk.Label(self.right_bar, text="MESH HEALTH", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"]).pack(anchor="w")
        
        self.health_canvas = tk.Canvas(self.right_bar, width=220, height=100, bg=PAL["sidebar"], highlightthickness=0)
        self.health_canvas.pack(pady=20)
        self._animate_health(0)

        self._item_val(self.right_bar, "Active Nodes", "3,402", PAL["success"])
        self._item_val(self.right_bar, "Mesh Encryption", "SHA3-X-P2P", PAL["accent"])

    def _item_val(self, parent, key, val, color):
        f = tk.Frame(parent, bg=PAL["sidebar"], pady=10)
        f.pack(fill="x")
        tk.Label(f, text=key, font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"]).pack(anchor="w")
        tk.Label(f, text=val, font=("Inter", 10, "bold"), fg=color, bg=PAL["sidebar"]).pack(anchor="w")

    def _animate_health(self, step):
        self.health_canvas.delete("all")
        for i in range(15):
             h = random.randint(10, 80)
             self.health_canvas.create_rectangle(i*15, 100-h, i*15+10, 100, fill=PAL["accent"], outline="")
        self.after(200, lambda: self._animate_health(step + 1))

    def send_message(self, event=None):
        msg = self.msg_var.get()
        if msg:
            self._add_log("LOC_NODE", msg, color=PAL["accent"])
            self.msg_var.set("")
            
            # Simulated Peer Response
            if "hello" in msg.lower():
                self.after(1000, lambda: self._add_log("REMOTE_NODE_4", "Salutations from the Aether Mesh."))

    def _add_log(self, author, msg, color="#E8E8E8"):
        self.chat_log.config(state="normal")
        ts = time.strftime("%H:%M")
        self.chat_log.insert(tk.END, f"[{ts}] ", "dim")
        self.chat_log.insert(tk.END, f"{author}: ", author)
        self.chat_log.insert(tk.END, f"{msg}\n\n")
        self.chat_log.tag_configure(author, foreground=color, font=("Inter Bold", 11))
        self.chat_log.tag_configure("dim", foreground=PAL["dim"], font=("Inter", 9))
        self.chat_log.config(state="disabled")
        self.chat_log.see(tk.END)

if __name__ == "__main__":
    app = MeshTalk()
    app.mainloop()
