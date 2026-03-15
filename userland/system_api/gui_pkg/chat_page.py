import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SigmaChatPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Sovereign Comm-Tunnel", "Military-Grade P2P Encryption | Zero-Knowledge Identity")
        self.engine = getattr(self.kernel, "chat_engine", None)
        self._build_ui()
        
        # Subscribe to chat events
        if self.kernel:
            self.kernel.bus.subscribe("social.chat.msg", self._on_msg_received)

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # Main Layout: Sidebar (Peers) + Chat Area
        self.panes = tk.PanedWindow(body, orient="horizontal", bg=PAL["bg"], sashwidth=2, bd=0)
        self.panes.pack(fill="both", expand=True)

        # --- Sidebar: Secure Peers ---
        sidebar = tk.Frame(self.panes, bg=PAL["bg2"], width=250)
        self.panes.add(sidebar)
        
        tk.Label(sidebar, text="🔒 VERIFIED PEERS", font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["bg2"], pady=10).pack()
        
        self.peer_list = tk.Listbox(
            sidebar, bg=PAL["bg3"], fg=PAL["text"], font=("Segoe UI", 9),
            selectbackground=PAL["accent"], borderwidth=0, highlightthickness=0
        )
        self.peer_list.pack(fill="both", expand=True, padx=5, pady=5)
        
        # Stats Card in Sidebar
        stats_fr = self.gui._premium_card(sidebar, "Tunnel Stats", icon="⚡")
        stats_fr.master.pack(fill="x", side="bottom", padx=5, pady=5)
        self.stats_var = tk.StringVar(value="Active Tunnels: 0\nE2EE: AES-GCM")
        tk.Label(stats_fr, textvariable=self.stats_var, font=FONT_SMALL, fg=PAL["text"], bg=PAL["bg2"], justify="left").pack(anchor="w")

        # --- Main Chat Area ---
        chat_main = tk.Frame(self.panes, bg=PAL["bg"])
        self.panes.add(chat_main)

        # Chat Header (Identity Info)
        header = tk.Frame(chat_main, bg=PAL["bg3"], pady=10, padx=15)
        header.pack(fill="x")
        
        id_fr = tk.Frame(header, bg=PAL["bg3"])
        id_fr.pack(side="left")
        
        self.my_sid_var = tk.StringVar(value="Local SID: OFFLINE")
        if self.engine and hasattr(self.engine, 'identity'):
             self.my_sid_var.set(f"Local SID: {self.engine.identity.sid}")
             
        tk.Label(id_fr, textvariable=self.my_sid_var, font=FONT_BOLD, fg=PAL["gold"], bg=PAL["bg3"]).pack(side="left")
        
        self.alias_var = tk.StringVar(value="(@User)")
        if self.engine and hasattr(self.engine, 'identity'):
             self.alias_var.set(f"(@{self.engine.identity.alias})")
             
        tk.Label(id_fr, textvariable=self.alias_var, font=FONT_SMALL, fg=PAL["cyan"], bg=PAL["bg3"]).pack(side="left", padx=5)

        tk.Button(header, text="🎭 SHIFT ALIAS", font=("Inter", 7, "bold"), bg=PAL["bg4"], fg=PAL["text"], 
                  relief="flat", command=self._show_alias_switcher).pack(side="right", padx=10)
        
        tk.Label(header, text="● SECURE TUNNEL ACTIVE", font=FONT_SMALL, fg=PAL["green"], bg=PAL["bg3"]).pack(side="right")

        # Chat Log
        log_container = tk.Frame(chat_main, bg=PAL["bg"])
        log_container.pack(fill="both", expand=True, pady=10)
        
        self.chat_log = tk.Text(
            log_container, bg=PAL["bg2"], fg=PAL["text"], font=("Segoe UI", 10),
            padx=15, pady=15, borderwidth=0, highlightthickness=0, state="disabled"
        )
        self.chat_log.pack(fill="both", expand=True)

        # Input Area
        input_fr = tk.Frame(chat_main, bg=PAL["bg3"], pady=15, padx=15)
        input_fr.pack(fill="x")
        
        self.msg_entry = self.gui._frosted_entry(input_fr, "Type a secure message...")
        self.msg_entry.container.pack(side="left", fill="x", expand=True, padx=(0, 10))
        self.msg_entry.bind("<Return>", lambda e: self._send_msg())
        
        send_btn = self.gui._pulsing_button(input_fr, "DISPATCH", self._send_msg)
        send_btn.pack(side="right")

        # UI Toggles
        tool_bar = tk.Frame(chat_main, bg=PAL["bg"], pady=5)
        tool_bar.pack(fill="x")
        
        self.stealth_mode = tk.BooleanVar(value=True)
        tk.Checkbutton(tool_bar, text="👻 Stealth Mode", variable=self.stealth_mode, bg=PAL["bg"], fg=PAL["dim"], selectcolor=PAL["bg2"]).pack(side="left", padx=10)
        
        self.shred_mode = tk.BooleanVar(value=True)
        tk.Checkbutton(tool_bar, text="🔥 Auto-Shred (60s)", variable=self.shred_mode, bg=PAL["bg"], fg=PAL["dim"], selectcolor=PAL["bg2"]).pack(side="left", padx=10)

        # Sync Initial State
        self._refresh_peers()

    def _refresh_peers(self):
        if not self.engine: 
             self.stats_var.set("Engine: OFFLINE\nE2EE: LOCKED")
             return
        self.peer_list.delete(0, "end")
        for sid in self.engine.peers:
            self.peer_list.insert("end", f"🔒 {sid}")
        self.stats_var.set(f"Active Tunnels: {len(self.engine.peers)}\nE2EE: AES-256-GCM")
        self.after(5000, self._refresh_peers)

    def _send_msg(self):
        text = self.msg_entry.get()
        if not text or text == "Type a secure message...": return
        
        if self.engine:
            res = self.engine.send_broadcast(text)
            self._insert_log("Me", text, is_me=True)
            self.msg_entry.delete(0, "end")
            self._notify("Sovereign Chat", res, "OK")
    
    def _on_msg_received(self, msg):
        sender = msg.get("sid", "UNKNOWN")
        text = msg.get("text", "")
        self._insert_log(sender, text)

    def _show_alias_switcher(self):
        new_alias = self.gui._prompt_input("Identity Shift", "Enter new ephemeral alias:")
        if new_alias and self.engine:
            res = self.engine.switch_alias(new_alias)
            self.alias_var.set(f"(@{new_alias})")
            self._notify("Sovereign Identity", res, "OK")
            self._insert_log("SYSTEM", "Zero-Knowledge Identity Rotated.", is_me=False)

    def _shred_ui_message(self, snippet):
        """USP: Visual Pixel Decay (Military-Grade UI feedback)."""
        import random
        chars = "!@#$%^&*()_+-=[]{}|;:,.<>?"
        self.chat_log.config(state="normal")
        
        content = self.chat_log.get("1.0", "end")
        if snippet in content:
            idx = content.find(snippet)
            # Simulate decay over 5 steps
            def _decay(step=0):
                if step > 5:
                    # Final purge from log
                    self.chat_log.config(state="normal")
                    self.chat_log.delete("1.0", "end")
                    self.chat_log.insert("end", "[METADATA PURGED BY SOVEREIGN WARDEN]\n", "shred")
                    self.chat_log.tag_config("shred", foreground=PAL["red"], font=FONT_SMALL)
                    self.chat_log.config(state="disabled")
                    return
                
                # Replace portion with entropy
                decayed = "".join(random.choice(chars) for _ in range(len(snippet)))
                self.chat_log.config(state="normal")
                # This is a simplified visual replacement
                self.chat_log.insert("end", f"\n[DECAYING]: {decayed}", "shred")
                self.chat_log.config(state="disabled")
                self.after(200, lambda: _decay(step + 1))
            
            _decay()
        
        self.chat_log.config(state="disabled")

    def _insert_log(self, sender, text, is_me=False):
        self.chat_log.config(state="normal")
        color = PAL["cyan"] if is_me else PAL["gold"]
        prefix = f"[{sender}]: "
        
        self.chat_log.insert("end", prefix, "prefix")
        self.chat_log.insert("end", f"{text}\n\n", "body")
        
        self.chat_log.tag_config("prefix", foreground=color, font=FONT_BOLD)
        self.chat_log.tag_config("body", foreground=PAL["text"])
        
        self.chat_log.see("end")
        self.chat_log.config(state="disabled")

        if self.shred_mode.get():
             # Start decay in 60s
             self.after(60000, lambda: self._shred_ui_message(text[:20]))
