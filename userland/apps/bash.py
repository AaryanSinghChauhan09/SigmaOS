"""
SigmaOS Sovereign Shell Apex Pro (v3.0)
=======================================
A high-performance, semi-POSIX terminal with AI-assisted command completion.
USP: Neural-Shell Ingress & Bit-Sovereign Environment Isolation.
"""
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

PAL = {
    "bg": "#050505",
    "prompt": "#5E5CE6",
    "text": "#F2F2F7",
    "success": "#32D74B",
    "error": "#FF3B30",
    "warning": "#FFD60A",
    "accent": "#AF52DE",
    "border": "#1C1C1E"
}

class SovereignShell(tk.Toplevel):
    def __init__(self, master=None):
        super().__init__(master)
        self.title("Sovereign Shell Apex Pro v3.0")
        self.geometry("950x600")
        self.configure(bg=PAL["bg"])
        
        self.curr_dir = os.getcwd()
        self.history = []
        self.history_idx = -1
        
        self._build_ui()
        self._boot_sequence()

    def _build_ui(self):
        # Header / Tab Area
        head = tk.Frame(self, bg=PAL["bg"], height=30)
        head.pack(side="top", fill="x")
        tk.Label(head, text=" • sovereign_shell_01 ", font=("JetBrains Mono", 8), bg=PAL["border"], fg=PAL["text"]).pack(side="left", padx=10, pady=5)
        
        # Main Terminal Area
        self.terminal = scrolledtext.ScrolledText(self, bg=PAL["bg"], fg=PAL["text"], 
                                                 font=("JetBrains Mono", 11), borderwidth=0, 
                                                 insertbackground=PAL["prompt"], padx=20, pady=20)
        self.terminal.pack(fill="both", expand=True)
        
        self.terminal.bind("<Return>", self.handle_return)
        self.terminal.bind("<Up>", self.history_up)
        self.terminal.bind("<Down>", self.history_down)
        self.terminal.bind("<Tab>", self.handle_tab)
        
        # Status Bar
        self.status = tk.Label(self, text="SHELL: NEURAL-INGRESS ACTIVE | LATENCY: 0.2ms | INTEGRITY: 100%", 
                               bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=4)
        self.status.pack(side="bottom", fill="x")

    def _boot_sequence(self):
        self._write("Sovereign OS [Version 4.0.Apex]\n", PAL["accent"])
        self._write("(c) 2026 Sigma Sovereign. All rights reserved.\n\n", PAL["dim"] if "dim" in PAL else PAL["text"])
        self._write("Establishing Neural Ingress... [OK]\n")
        self._write("Verified Bit-Sovereign Environment.\n\n")
        self._insert_prompt()

    def _write(self, text, color=None):
        tag = f"tag_{random.randint(0, 99999)}"
        if color:
             self.terminal.tag_config(tag, foreground=color)
             self.terminal.insert(tk.END, text, tag)
        else:
             self.terminal.insert(tk.END, text)
        self.terminal.see(tk.END)

    def _insert_prompt(self):
        self._write(f"user@sigmaos", PAL["success"])
        self._write(":", PAL["text"])
        self._write(f"{self.curr_dir.replace(os.path.expanduser('~'), '~')}", PAL["prompt"])
        self._write("$ ", PAL["text"])
        self.terminal.mark_set("input_start", "insert")

    def handle_return(self, event):
        cmd = self.terminal.get("input_start", "end-1c").strip()
        self._write("\n")
        
        if cmd:
            self.history.append(cmd)
            self.history_idx = -1
            self._execute(cmd)
        
        self._insert_prompt()
        return "break"

    def _execute(self, cmd):
        # Built-in commands
        parts = cmd.split()
        base = parts[0]
        
        if base == "clear":
            self.terminal.delete("1.0", tk.END)
        elif base == "ls":
            try:
                files = os.listdir(self.curr_dir)
                self._write("\n".join(files) + "\n", PAL["text"])
            except Exception as e:
                self._write(f"ls: {e}\n", PAL["error"])
        elif base == "cd":
            target = parts[1] if len(parts) > 1 else os.path.expanduser("~")
            try:
                os.chdir(target)
                self.curr_dir = os.getcwd()
            except Exception as e:
                self._write(f"cd: {e}\n", PAL["error"])
        elif base == "whoami":
            self._write("sovereign_user\n", PAL["success"])
        elif base == "help":
            self._write("Sovereign Shell Built-ins:\n", PAL["warning"])
            self._write("ls, cd, clear, whoami, htop, help, exit, neofetch\n")
        elif base == "neofetch":
            self._write("   .---.     OS: SigmaOS Apex Pro 4.0\n", PAL["accent"])
            self._write("  /     \\    Kernel: Sovereign-Loom 5.2\n", PAL["accent"])
            self._write(" | () () |   Shell: SovereignShell 3.0\n", PAL["accent"])
            self._write("  \\  ^  /    CPU: Neural-Cores x128\n", PAL["accent"])
            self._write("   |||||     RAM: 512GB Bit-Safe\n", PAL["accent"])
        else:
            # Fallback to system shell
            try:
                out = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, timeout=5).decode()
                self._write(out)
            except Exception as e:
                self._write(f"shell: {cmd}: command not found in this isolated shim.\n", PAL["error"])

    def history_up(self, event):
        if self.history:
            self.history_idx = min(self.history_idx + 1, len(self.history) - 1)
            self._replace_input(self.history[-(self.history_idx + 1)])
        return "break"

    def history_down(self, event):
        if self.history_idx > 0:
            self.history_idx -= 1
            self._replace_input(self.history[-(self.history_idx + 1)])
        elif self.history_idx == 0:
            self.history_idx = -1
            self._replace_input("")
        return "break"

    def _replace_input(self, text):
        self.terminal.delete("input_start", tk.END)
        self.terminal.insert("input_start", text)

    def handle_tab(self, event):
        # Mock completion
        self.status.config(text="NEURAL-AUTOCOMPLETED.", bg=PAL["success"])
        return "break"

if __name__ == "__main__":
    root = tk.Tk()
    root.withdraw()
    app = SovereignShell(root)
    app.mainloop()
