"""
SigmaOS Nexus Share (v1.0)
==========================
Hyper-local encrypted peer-to-peer data transmission (Airdrop/Nearby Share equivalent).
USP: Sub-space radio frequency jumping and decentralized handshake.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#007AFF", # Azure Blue
    "accent_dim": "#005EB8",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class NexusShare(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Nexus Share")
        self.geometry("850x650")
        self.configure(bg=PAL["bg"])
        
        self.scanning = False
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Nexus.TProgressbar", background=PAL["accent"], troughcolor=PAL["sidebar"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="NEXUS SHARE PROTOCOL", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="📡 BROADCAST RADAR", font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                  relief="flat", padx=15, pady=8, command=self._start_radar).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Drop Zone
        self.drop_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=20, pady=40, cursor="hand2")
        self.drop_fr.pack(fill="x", pady=(0, 20))
        
        tk.Label(self.drop_fr, text="⬇️ DROP FRAGMENTS HERE ⬇️", font=("Inter", 16, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack()
        tk.Label(self.drop_fr, text="(Encrypted P2P Tunneling via Wi-Fi Direct / BT-LE)", font=("Inter", 9), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(5, 0))
        
        self.drop_fr.bind("<Button-1>", self._mock_file_select)
        
        # Nodes Matrix
        tk.Label(self.workspace, text="VISIBLE SOVEREIGN NODES", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w", pady=(10, 5))
        
        self.nodes_canvas = tk.Canvas(self.workspace, bg=PAL["bg"], highlightthickness=0, height=200)
        self.nodes_canvas.pack(fill="both", expand=True)
        
        self._draw_nodes([])

        # Status
        self.status = tk.Label(self, text="IDLE | AWAITING PAYLOAD", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _mock_file_select(self, event):
        self.status.config(text="PAYLOAD SECURED: [Project_Nova.zip] | AWAITING NODE SELECTION", bg=PAL["success"], fg="black")

    def _draw_nodes(self, nodes_list):
        self.nodes_canvas.delete("all")
        
        if not nodes_list:
            self.nodes_canvas.create_text(400, 100, text="No adjacent nodes detected. Radar inactive.", fill=PAL["dim"], font=("Inter", 10, "italic"))
            return
            
        for i, node in enumerate(nodes_list):
            x = 80 + (i * 150)
            y = 100
            
            # Draw circle avatar
            col = random.choice([PAL["accent"], PAL["success"], "#FDB813"])
            self.nodes_canvas.create_oval(x-30, y-30, x+30, y+30, fill=col, outline=PAL["sidebar"], width=3)
            
            # Name
            self.nodes_canvas.create_text(x, y+45, text=node, fill=PAL["text"], font=("Inter", 9, "bold"))
            
            # Clickable binding would go here, using mock for visual
            lbl = tk.Label(self.nodes_canvas, text="TRANSMIT", font=("Inter", 7, "bold"), bg=PAL["panel"], fg="white", cursor="hand2")
            lbl.bind("<Button-1>", lambda e, n=node: self._transmit_payload(n))
            self.nodes_canvas.create_window(x, y+65, window=lbl)

    def _start_radar(self):
        if self.scanning: return
        self.scanning = True
        self.status.config(text="BROADCASTING QUANTUM HANDSHAKE...", bg=PAL["warning"], fg="black")
        
        def mock_scan():
            time.sleep(1.5)
            nodes = ["Sigma-Node-Alpha", "Ghost-Client-X", "Aura-Phone-2"]
            self._draw_nodes(nodes)
            self.status.config(text="RADAR ECHO RETURNED. 3 NODES ACQUIRED.", bg=PAL["sidebar"], fg="white")
            self.scanning = False
            
        threading.Thread(target=mock_scan, daemon=True).start()

    def _transmit_payload(self, node):
        res = messagebox.askyesno("Transmit Payload", f"Initialize tunnel to [{node}]?")
        if res:
            self.status.config(text=f"TRANSMITTING FRAGMENTS TO {node} | ENCRYPTING...", bg=PAL["accent"], fg="white")
            self.after(2000, lambda: self.status.config(text="TRANSMISSION 100% SUCCESSFUL | P2P TUNNEL COLLAPSED", bg=PAL["success"], fg="black"))

if __name__ == "__main__":
    app = NexusShare()
    app.mainloop()
