"""
SigmaOS Markdown & Rich Text Viewer v1.0
Renders Markdown in tkinter — zero 3rd-party deps.
"""
import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os

PAL={"bg":"#0D0F18","panel":"#13162A","card":"#1A1E30","accent":"#6C63FF",
     "text":"#E8E8F0","dim":"#9090A0","border":"#2A2D45","code":"#1E2840",
     "h1":"#6C63FF","h2":"#F59E0B","h3":"#22C55E","quote":"#9090A0"}

SAMPLE = """# SigmaOS Documentation

## About SigmaOS
**SigmaOS** is the world's most *sovereign* operating system built purely in Python.

## Features
- Zero third-party dependencies
- Full NCERT Virtual Lab (Physics, Chemistry, Biology, Maths)
- Scientific Calculator & Unit Converter
- Games Engine with 20 lightweight games

## Quick Start
```python
from sigma_core.kernel import SigmaKernel
k = SigmaKernel()
k.boot()
```

> "Sovereignty is not given; it is engineered." — SigmaOS Manifesto

## Classes Supported in NCERT Lab
| Subject | Classes | Experiments |
|---------|---------|-------------|
| Physics | 6–12 | 14 |
| Chemistry | 6–12 | 12 |
| Biology | 6–12 | 10 |
| Maths | 1–12 | 14 |
"""

class MarkdownViewer(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS Markdown Viewer")
        self.geometry("1000x680"); self.configure(bg=PAL["bg"])
        self._current_file = None
        self._build()
        self._render(SAMPLE)

    def _build(self):
        # Toolbar
        tb = tk.Frame(self, bg=PAL["panel"], height=50)
        tb.pack(fill="x"); tb.pack_propagate(False)
        tk.Label(tb, text="📄 MARKDOWN VIEWER", fg=PAL["accent"],
                 bg=PAL["panel"], font=("Segoe UI Bold",13)).pack(side="left",padx=18,pady=10)
        tk.Button(tb, text="📂 Open", bg=PAL["card"], fg=PAL["text"],
                  font=("Segoe UI",9), relief="flat", padx=12,
                  command=self._open).pack(side="left",padx=4,pady=10)
        tk.Button(tb, text="💾 Save HTML", bg=PAL["card"], fg=PAL["text"],
                  font=("Segoe UI",9), relief="flat", padx=12,
                  command=self._save_html).pack(side="left",padx=4,pady=10)
        self._file_lbl = tk.Label(tb, text="No file open", fg=PAL["dim"],
                                   bg=PAL["panel"], font=("Segoe UI",8))
        self._file_lbl.pack(side="right", padx=18)

        # Split view: raw | rendered
        pane = tk.PanedWindow(self, orient="horizontal", bg=PAL["bg"],
                              sashwidth=4, sashrelief="flat")
        pane.pack(fill="both", expand=True, padx=8, pady=8)

        # Raw editor
        raw_fr = tk.Frame(pane, bg=PAL["bg"]); pane.add(raw_fr, minsize=300)
        tk.Label(raw_fr, text="SOURCE", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",8,"bold"), pady=4).pack(anchor="w")
        self._editor = scrolledtext.ScrolledText(raw_fr, bg=PAL["card"], fg=PAL["text"],
                                                  font=("Cascadia Code",10), borderwidth=0,
                                                  padx=12, pady=12, insertbackground="white")
        self._editor.pack(fill="both", expand=True)
        self._editor.bind("<KeyRelease>", lambda e: self._render(self._editor.get("1.0","end")))

        # Rendered view
        rnd_fr = tk.Frame(pane, bg=PAL["bg"]); pane.add(rnd_fr, minsize=400)
        tk.Label(rnd_fr, text="PREVIEW", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",8,"bold"), pady=4).pack(anchor="w")
        self._view = scrolledtext.ScrolledText(rnd_fr, bg="#0A0C18", fg=PAL["text"],
                                               font=("Segoe UI",11), borderwidth=0,
                                               padx=20, pady=20, state="disabled",
                                               wrap="word", insertbackground="white")
        self._view.pack(fill="both", expand=True)
        self._setup_tags()

    def _setup_tags(self):
        v=self._view
        v.tag_config("h1",foreground=PAL["h1"],font=("Segoe UI Bold",22),spacing3=8)
        v.tag_config("h2",foreground=PAL["h2"],font=("Segoe UI Bold",17),spacing3=6)
        v.tag_config("h3",foreground=PAL["h3"],font=("Segoe UI Bold",14),spacing3=4)
        v.tag_config("bold",font=("Segoe UI Bold",11))
        v.tag_config("italic",font=("Segoe UI Italic",11))
        v.tag_config("code_inline",foreground="#E2C36A",font=("Cascadia Code",10),
                     background=PAL["code"])
        v.tag_config("code_block",foreground="#A8E6CF",font=("Cascadia Code",9),
                     background=PAL["code"],lmargin1=20,lmargin2=20,spacing1=4,spacing3=4)
        v.tag_config("bullet",lmargin1=20,lmargin2=30,spacing1=2)
        v.tag_config("quote",foreground=PAL["quote"],lmargin1=24,font=("Segoe UI Italic",11))
        v.tag_config("hr",foreground=PAL["border"])
        v.tag_config("table_head",foreground=PAL["accent"],font=("Segoe UI Bold",10))
        v.tag_config("table_row",foreground=PAL["text"],font=("Cascadia Code",9))

    def _render(self, md_text):
        v=self._view
        v.configure(state="normal"); v.delete("1.0","end")
        in_code=False; code_buf=[]
        lines=md_text.split("\n")
        for line in lines:
            # Code block
            if line.strip().startswith("```"):
                if in_code:
                    v.insert("end","\n".join(code_buf)+"\n","code_block")
                    code_buf=[]; in_code=False
                else: in_code=True
                continue
            if in_code: code_buf.append(line); continue

            # Headings
            if line.startswith("### "): v.insert("end",line[4:]+"\n","h3"); continue
            if line.startswith("## "):  v.insert("end",line[3:]+"\n","h2"); continue
            if line.startswith("# "):   v.insert("end",line[2:]+"\n","h1"); continue
            # HR
            if re.match(r'^[-*_]{3,}$', line.strip()):
                v.insert("end","─"*58+"\n","hr"); continue
            # Blockquote
            if line.startswith("> "):
                v.insert("end","│ "+line[2:]+"\n","quote"); continue
            # Table
            if "|" in line and line.strip().startswith("|"):
                if re.match(r'^\|[-| :]+\|$',line.strip()): continue
                cells = [c.strip() for c in line.strip().strip("|").split("|")]
                row_txt = "  ".join(f"{c:<18}" for c in cells)
                tag="table_head" if all(c.isupper() or len(c)<5 for c in cells) else "table_row"
                v.insert("end",row_txt+"\n",tag); continue
            # Bullet
            if re.match(r'^[\-\*\+]\s', line):
                self._insert_inline(v, "  •  "+line[2:]+"\n", "bullet"); continue
            # Numbered list
            if re.match(r'^\d+\.\s', line):
                self._insert_inline(v, "  "+line+"\n", "bullet"); continue
            # Normal paragraph
            self._insert_inline(v, line+"\n")
        if code_buf: v.insert("end","\n".join(code_buf)+"\n","code_block")
        v.configure(state="disabled")
        # Update editor
        try:
            cur = self._editor.get("1.0","end-1c")
            if cur != md_text.rstrip("\n"):
                self._editor.delete("1.0","end")
                self._editor.insert("1.0", md_text)
        except Exception: pass

    def _insert_inline(self, widget, text, base_tag=""):
        parts = re.split(r'(\*\*.*?\*\*|\*.*?\*|`.*?`)', text)
        for part in parts:
            if part.startswith("**") and part.endswith("**"):
                widget.insert("end", part[2:-2], "bold")
            elif part.startswith("*") and part.endswith("*"):
                widget.insert("end", part[1:-1], "italic")
            elif part.startswith("`") and part.endswith("`"):
                widget.insert("end", part[1:-1], "code_inline")
            else:
                widget.insert("end", part, base_tag)

    def _open(self):
        path = filedialog.askopenfilename(filetypes=[("Markdown","*.md *.txt"),("All","*.*")])
        if path:
            try:
                with open(path, encoding="utf-8") as f: content=f.read()
                self._current_file=path
                self._file_lbl.config(text=os.path.basename(path))
                self._render(content)
            except Exception as e:
                tk.messagebox.showerror("Error", str(e))

    def _save_html(self):
        path = filedialog.asksaveasfilename(defaultextension=".html",
                                            filetypes=[("HTML","*.html")])
        if path:
            md=self._editor.get("1.0","end")
            html=f"<html><body><pre>{md}</pre></body></html>"
            with open(path,"w",encoding="utf-8") as f: f.write(html)

def launch(kernel=None):
    MarkdownViewer(kernel).mainloop()

if __name__=="__main__":
    launch()
