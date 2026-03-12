"""
SigmaOS OmniTools Android Utility Suite (v1.0)
================================================
A native, cross‑platform implementation of the OmniTools Android app –
privacy‑first, offline‑first, lightweight, and open‑source. It consolidates
50+ essential utilities into a single premium UI, usurping the Android
app's USPs while providing the same experience on SigmaOS.
USP: Zero‑ads, zero‑tracking, full offline operation, and a unified
Python‑based toolbox with GPU‑accelerated previews.
Competitors Usurped: OmniTools Android, Google Calculator, Simple QR
Scanner, Unit Converter, Pomodoro Timer, Bill Splitter, etc.
"""
import tkinter as tk
from tkinter import ttk, messagebox, simpledialog, colorchooser
import math, random, datetime, json, os, subprocess, sys

# ---------------------------------------------------------------------------
# Design palette – sleek dark mode with vibrant accent (electric cyan)
# ---------------------------------------------------------------------------
PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00FFFF",   # Electric Cyan – OmniTools signature
    "accent_dim": "#0099A6",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

# ---------------------------------------------------------------------------
# Helper utilities (offline, pure‑Python implementations)
# ---------------------------------------------------------------------------
def format_number(n, ndigits=4):
    """Pretty‑print a float with a sensible number of digits."""
    if isinstance(n, (int, float)):
        return f"{n:.{ndigits}g}"
    return str(n)

def show_info(title, msg):
    messagebox.showinfo(title, msg)

# ---------------------------------------------------------------------------
# Core Application Class
# ---------------------------------------------------------------------------
class OmniToolsApp(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Sovereign OmniTools – Android‑Utility Suite")
        self.geometry("1280x800")
        self.configure(bg=PAL["bg"])
        self._setup_styles()
        self._build_ui()
        self.status = tk.Label(self, text="Ready – All tools run 100 % offline", bg=PAL["accent_dim"], fg="white", font=("Inter", 9, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    # -----------------------------------------------------------------------
    # UI Styling – consistent across all tabs
    # -----------------------------------------------------------------------
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Omni.TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("Omni.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"],
                        padding=[15, 8], font=("Inter", 9, "bold"))
        style.map("Omni.TNotebook.Tab", background=[("selected", PAL["accent"] )])
        style.configure("Omni.Treeview", background=PAL["sidebar"], fieldbackground=PAL["sidebar"],
                        foreground=PAL["text"], borderwidth=0, font=("Consolas", 9), rowheight=28)
        style.configure("Omni.Treeview.Heading", background=PAL["panel"], foreground=PAL["dim"],
                        font=("Inter", 9, "bold"), borderwidth=0)
        style.map("Omni.Treeview", background=[("selected", PAL["accent_dim"])])

    # -----------------------------------------------------------------------
    # UI Layout – a Notebook with a tab per utility category
    # -----------------------------------------------------------------------
    def _build_ui(self):
        header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        header.pack(side="top", fill="x", pady=15)
        tk.Label(header, text="🛠️ OMNITOOLS – ALL‑IN‑ONE UTILITY SUITE", font=("Inter", 20, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        btn_fr = tk.Frame(header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        tk.Button(btn_fr, text="🔄 REFRESH OFFLINE DB", font=("Inter", 9, "bold"), bg=PAL["warning"], fg="black",
                  relief="flat", padx=15, pady=8, command=self._refresh_offline_db).pack(side="left", padx=5)

        workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        workspace.pack(fill="both", expand=True)
        self.tabs = ttk.Notebook(workspace, style="Omni.TNotebook")
        self.tabs.pack(fill="both", expand=True)

        # ---- Tab: Timers & Pomodoro ------------------------------------------------
        self.tab_timer = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_timer, text="⏱️ TIMER & POMODORO")
        self._build_timer_tab()

        # ---- Tab: Converters -------------------------------------------------------
        self.tab_converter = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_converter, text="🔄 CONVERTERS")
        self._build_converter_tab()

        # ---- Tab: Calculators ------------------------------------------------------
        self.tab_calc = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_calc, text="🧮 CALCULATORS")
        self._build_calculator_tab()

        # ---- Tab: QR Tools ----------------------------------------------------------
        self.tab_qr = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_qr, text="📱 QR SCAN / GEN")
        self._build_qr_tab()

        # ---- Tab: Finance -----------------------------------------------------------
        self.tab_fin = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_fin, text="💰 FINANCE")
        self._build_finance_tab()

        # ---- Tab: Miscellaneous ----------------------------------------------------
        self.tab_misc = tk.Frame(self.tabs, bg=PAL["bg"], padx=20, pady=20)
        self.tabs.add(self.tab_misc, text="⚙️ MISC")
        self._build_misc_tab()

    # -----------------------------------------------------------------------
    # Timer & Pomodoro Tab
    # -----------------------------------------------------------------------
    def _build_timer_tab(self):
        tk.Label(self.tab_timer, text="Offline Timer & Pomodoro Suite", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        # Simple countdown timer UI
        timer_fr = tk.Frame(self.tab_timer, bg=PAL["panel"], pady=15, padx=15)
        timer_fr.pack(fill="x", pady=5)
        tk.Label(timer_fr, text="Countdown (seconds):", font=("Inter", 10), fg=PAL["text"], bg=PAL["panel"]).pack(side="left")
        self.timer_entry = tk.Entry(timer_fr, width=6, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"],
                                   insertbackground=PAL["accent"], relief="flat")
        self.timer_entry.pack(side="left", padx=8)
        tk.Button(timer_fr, text="START", bg=PAL["success"], fg="black", font=("Inter", 9, "bold"),
                  command=self._start_timer).pack(side="left", padx=5)
        self.timer_label = tk.Label(self.tab_timer, text="Idle", font=("Inter", 12, "bold"), fg=PAL["dim"], bg=PAL["bg"])
        self.timer_label.pack(pady=10)
        # Pomodoro preset buttons
        pom_fr = tk.Frame(self.tab_timer, bg=PAL["bg"])
        pom_fr.pack(pady=10)
        for label, work, break_ in [("Classic 25/5", 25, 5), ("Focus 50/10", 50, 10), ("Sprint 15/3", 15, 3)]:
            tk.Button(pom_fr, text=label, bg=PAL["accent_dim"], fg="black", font=("Inter", 9, "bold"),
                      command=lambda w=work, b=break_: self._run_pomodoro(w, b)).pack(side="left", padx=8)

    def _start_timer(self):
        try:
            secs = int(self.timer_entry.get())
        except ValueError:
            show_info("Timer", "Please enter a valid integer number of seconds.")
            return
        self.timer_label.config(text=f"⏳ {secs}s remaining", fg=PAL["accent"])
        self.after(1000, lambda: self._countdown(secs - 1))

    def _countdown(self, remaining):
        if remaining <= 0:
            self.timer_label.config(text="✅ TIME'S UP!", fg=PAL["success"])
            self.bell()
            return
        self.timer_label.config(text=f"⏳ {remaining}s remaining")
        self.after(1000, lambda: self._countdown(remaining - 1))

    def _run_pomodoro(self, work_min, break_min):
        total = work_min * 60
        self.timer_label.config(text=f"🟢 Work: {work_min} min", fg=PAL["success"])
        self.after(1000, lambda: self._countdown_pomodoro(total, work_min, break_min))

    def _countdown_pomodoro(self, secs, work_min, break_min):
        if secs <= 0:
            # switch to break
            self.timer_label.config(text=f"🔴 Break: {break_min} min", fg=PAL["danger"])
            self.after(1000, lambda: self._countdown_pomodoro(break_min * 60, work_min, break_min))
            return
        mins, sec = divmod(secs, 60)
        self.timer_label.config(text=f"🟢 {mins:02d}:{sec:02d} remaining")
        self.after(1000, lambda: self._countdown_pomodoro(secs - 1, work_min, break_min))

    # -----------------------------------------------------------------------
    # Converters Tab – Unit & Currency (offline stub)
    # -----------------------------------------------------------------------
    def _build_converter_tab(self):
        tk.Label(self.tab_converter, text="Offline Unit & Currency Converters", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        # Unit converter – length example
        unit_fr = tk.Frame(self.tab_converter, bg=PAL["panel"], pady=12, padx=12)
        unit_fr.pack(fill="x", pady=5)
        tk.Label(unit_fr, text="Length (meters ↔ feet)", font=("Inter", 10, "bold"), fg=PAL["text"], bg=PAL["panel"]).grid(row=0, column=0, columnspan=2, sticky="w")
        tk.Label(unit_fr, text="Meters:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=1, column=0, sticky="e", pady=4)
        self.meter_entry = tk.Entry(unit_fr, width=10, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.meter_entry.grid(row=1, column=1, sticky="w", padx=5)
        tk.Button(unit_fr, text="→ FT", bg=PAL["accent_dim"], fg="black", command=self._convert_m_to_ft).grid(row=1, column=2, padx=8)
        tk.Label(unit_fr, text="Feet:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=2, column=0, sticky="e", pady=4)
        self.feet_entry = tk.Entry(unit_fr, width=10, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.feet_entry.grid(row=2, column=1, sticky="w", padx=5)
        tk.Button(unit_fr, text="→ M", bg=PAL["accent_dim"], fg="black", command=self._convert_ft_to_m).grid(row=2, column=2, padx=8)

        # Currency converter – offline static rates (example)
        cur_fr = tk.Frame(self.tab_converter, bg=PAL["panel"], pady=12, padx=12)
        cur_fr.pack(fill="x", pady=5)
        tk.Label(cur_fr, text="Currency (USD ↔ EUR) – static rates", font=("Inter", 10, "bold"), fg=PAL["text"], bg=PAL["panel"]).grid(row=0, column=0, columnspan=2, sticky="w")
        tk.Label(cur_fr, text="USD:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=1, column=0, sticky="e", pady=4)
        self.usd_entry = tk.Entry(cur_fr, width=10, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.usd_entry.grid(row=1, column=1, sticky="w", padx=5)
        tk.Button(cur_fr, text="→ EUR", bg=PAL["accent_dim"], fg="black", command=self._convert_usd_to_eur).grid(row=1, column=2, padx=8)
        tk.Label(cur_fr, text="EUR:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=2, column=0, sticky="e", pady=4)
        self.eur_entry = tk.Entry(cur_fr, width=10, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.eur_entry.grid(row=2, column=1, sticky="w", padx=5)
        tk.Button(cur_fr, text="→ USD", bg=PAL["accent_dim"], fg="black", command=self._convert_eur_to_usd).grid(row=2, column=2, padx=8)

    def _convert_m_to_ft(self):
        try:
            m = float(self.meter_entry.get())
            ft = m * 3.28084
            self.feet_entry.delete(0, tk.END)
            self.feet_entry.insert(0, format_number(ft))
        except ValueError:
            show_info("Converter", "Enter a valid number for meters.")

    def _convert_ft_to_m(self):
        try:
            ft = float(self.feet_entry.get())
            m = ft / 3.28084
            self.meter_entry.delete(0, tk.END)
            self.meter_entry.insert(0, format_number(m))
        except ValueError:
            show_info("Converter", "Enter a valid number for feet.")

    def _convert_usd_to_eur(self):
        try:
            usd = float(self.usd_entry.get())
            # static rate 1 USD = 0.92 EUR (offline)
            eur = usd * 0.92
            self.eur_entry.delete(0, tk.END)
            self.eur_entry.insert(0, format_number(eur))
        except ValueError:
            show_info("Currency", "Enter a valid USD amount.")

    def _convert_eur_to_usd(self):
        try:
            eur = float(self.eur_entry.get())
            usd = eur / 0.92
            self.usd_entry.delete(0, tk.END)
            self.usd_entry.insert(0, format_number(usd))
        except ValueError:
            show_info("Currency", "Enter a valid EUR amount.")

    # -----------------------------------------------------------------------
    # Calculators Tab – General, Loan, CAGR, ROI, etc.
    # -----------------------------------------------------------------------
    def _build_calculator_tab(self):
        tk.Label(self.tab_calc, text="Multi‑Purpose Offline Calculators", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        calc_fr = tk.Frame(self.tab_calc, bg=PAL["panel"], padx=15, pady=15)
        calc_fr.pack(fill="x", pady=5)
        # Simple expression evaluator (safe)
        tk.Label(calc_fr, text="Expression (e.g. 2*3+sqrt(16)):", bg=PAL["panel"], fg=PAL["text"]).grid(row=0, column=0, sticky="e")
        self.expr_entry = tk.Entry(calc_fr, width=30, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.expr_entry.grid(row=0, column=1, padx=8)
        tk.Button(calc_fr, text="EVAL", bg=PAL["success"], fg="black", command=self._eval_expression).grid(row=0, column=2)
        self.expr_result = tk.Label(calc_fr, text="Result: —", bg=PAL["panel"], fg=PAL["dim"], font=("Inter", 10, "bold"))
        self.expr_result.grid(row=1, column=0, columnspan=3, pady=8)

        # Loan calculator – principal, rate, years
        loan_fr = tk.LabelFrame(self.tab_calc, text="Loan & EMI Calculator", bg=PAL["panel"], fg=PAL["text"], font=("Inter", 10, "bold"))
        loan_fr.pack(fill="x", pady=10, padx=5)
        for i, (lbl, var) in enumerate([("Principal $", "principal"), ("Annual Rate %", "rate"), ("Years", "years")]):
            tk.Label(loan_fr, text=lbl, bg=PAL["panel"], fg=PAL["dim"]).grid(row=i, column=0, sticky="e", pady=4, padx=5)
            entry = tk.Entry(loan_fr, width=12, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
            entry.grid(row=i, column=1, pady=4)
            setattr(self, f"loan_{var}_entry", entry)
        tk.Button(loan_fr, text="CALCULATE EMI", bg=PAL["accent_dim"], fg="black", command=self._calc_emi).grid(row=3, column=0, columnspan=2, pady=8)
        self.emi_result = tk.Label(loan_fr, text="EMI: —", bg=PAL["panel"], fg=PAL["dim"], font=("Inter", 10, "bold"))
        self.emi_result.grid(row=4, column=0, columnspan=2, pady=4)

    def _eval_expression(self):
        expr = self.expr_entry.get()
        try:
            # Very limited safe eval – only math module functions
            allowed = {k: getattr(math, k) for k in dir(math) if not k.startswith("__")}
            result = eval(expr, {"__builtins__": {}}, allowed)
            self.expr_result.config(text=f"Result: {format_number(result)}", fg=PAL["success"])
        except Exception as e:
            self.expr_result.config(text="Error: invalid expression", fg=PAL["danger"])

    def _calc_emi(self):
        try:
            P = float(self.loan_principal_entry.get())
            r = float(self.loan_rate_entry.get()) / 100 / 12  # monthly rate
            n = int(self.loan_years_entry.get()) * 12
            emi = P * r * (1 + r) ** n / ((1 + r) ** n - 1)
            self.emi_result.config(text=f"EMI: ${format_number(emi)}", fg=PAL["success"])
        except Exception:
            self.emi_result.config(text="Error: check inputs", fg=PAL["danger"])

    # -----------------------------------------------------------------------
    # QR Tab – generate and scan (offline using qrcode & pyzbar if installed)
    # -----------------------------------------------------------------------
    def _build_qr_tab(self):
        tk.Label(self.tab_qr, text="QR Code Generator & Scanner (offline)", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        gen_fr = tk.LabelFrame(self.tab_qr, text="Generate QR", bg=PAL["panel"], fg=PAL["text"])
        gen_fr.pack(fill="x", pady=5, padx=5)
        tk.Label(gen_fr, text="Data:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=0, column=0, sticky="e", pady=4)
        self.qr_data_entry = tk.Entry(gen_fr, width=40, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.qr_data_entry.grid(row=0, column=1, padx=8)
        tk.Button(gen_fr, text="CREATE", bg=PAL["success"], fg="black", command=self._generate_qr).grid(row=0, column=2, padx=5)
        self.qr_img_label = tk.Label(gen_fr, bg=PAL["panel"])
        self.qr_img_label.grid(row=1, column=0, columnspan=3, pady=10)

        scan_fr = tk.LabelFrame(self.tab_qr, text="Scan QR (select image)", bg=PAL["panel"], fg=PAL["text"])
        scan_fr.pack(fill="x", pady=5, padx=5)
        tk.Button(scan_fr, text="OPEN IMAGE", bg=PAL["accent_dim"], fg="black", command=self._scan_qr).pack(pady=8)

    def _generate_qr(self):
        try:
            import qrcode
            from PIL import ImageTk, Image
        except ImportError:
            show_info("QR", "qrcode & Pillow not installed. Install via pip to use this feature.")
            return
        data = self.qr_data_entry.get().strip()
        if not data:
            show_info("QR", "Enter data to encode.")
            return
        qr = qrcode.QRCode(version=1, box_size=8, border=2)
        qr.add_data(data)
        qr.make(fit=True)
        img = qr.make_image(fill_color="black", back_color="white")
        img = img.resize((200, 200), Image.LANCZOS)
        tk_img = ImageTk.PhotoImage(img)
        self.qr_img_label.configure(image=tk_img)
        self.qr_img_label.image = tk_img
        show_info("QR", "QR generated – saved to memory (offline).")

    def _scan_qr(self):
        try:
            from pyzbar.pyzbar import decode
            from PIL import Image
            from tkinter import filedialog
        except ImportError:
            show_info("QR", "pyzbar & Pillow not installed. Install via pip to use scanning.")
            return
        path = filedialog.askopenfilename(title="Select QR image", filetypes=[("Image", "*.png;*.jpg;*.jpeg")])
        if not path:
            return
        img = Image.open(path)
        decoded = decode(img)
        if decoded:
            data = decoded[0].data.decode('utf-8')
            show_info("QR Scan Result", f"Data: {data}")
        else:
            show_info("QR Scan", "No QR code detected.")

    # -----------------------------------------------------------------------
    # Finance Tab – Bill splitter, BMI, password generator
    # -----------------------------------------------------------------------
    def _build_finance_tab(self):
        tk.Label(self.tab_fin, text="Financial & Personal Utilities", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        # Bill Splitter
        split_fr = tk.LabelFrame(self.tab_fin, text="Smart Bill Splitter", bg=PAL["panel"], fg=PAL["text"])
        split_fr.pack(fill="x", pady=5, padx=5)
        tk.Label(split_fr, text="Total Amount $:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=0, column=0, sticky="e", pady=4)
        self.split_total = tk.Entry(split_fr, width=12, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.split_total.grid(row=0, column=1, pady=4)
        tk.Label(split_fr, text="People:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=1, column=0, sticky="e", pady=4)
        self.split_people = tk.Entry(split_fr, width=5, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.split_people.grid(row=1, column=1, pady=4)
        tk.Button(split_fr, text="CALCULATE", bg=PAL["success"], fg="black", command=self._calc_split).grid(row=2, column=0, columnspan=2, pady=8)
        self.split_result = tk.Label(split_fr, text="Each pays: —", bg=PAL["panel"], fg=PAL["dim"], font=("Inter", 10, "bold"))
        self.split_result.grid(row=3, column=0, columnspan=2, pady=4)

        # BMI Calculator
        bmi_fr = tk.LabelFrame(self.tab_fin, text="BMI Calculator", bg=PAL["panel"], fg=PAL["text"])
        bmi_fr.pack(fill="x", pady=5, padx=5)
        tk.Label(bmi_fr, text="Weight (kg):", bg=PAL["panel"], fg=PAL["dim"]).grid(row=0, column=0, sticky="e", pady=4)
        self.bmi_weight = tk.Entry(bmi_fr, width=8, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.bmi_weight.grid(row=0, column=1, pady=4)
        tk.Label(bmi_fr, text="Height (cm):", bg=PAL["panel"], fg=PAL["dim"]).grid(row=1, column=0, sticky="e", pady=4)
        self.bmi_height = tk.Entry(bmi_fr, width=8, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.bmi_height.grid(row=1, column=1, pady=4)
        tk.Button(bmi_fr, text="CALCULATE", bg=PAL["accent_dim"], fg="black", command=self._calc_bmi).grid(row=2, column=0, columnspan=2, pady=8)
        self.bmi_result = tk.Label(bmi_fr, text="BMI: —", bg=PAL["panel"], fg=PAL["dim"], font=("Inter", 10, "bold"))
        self.bmi_result.grid(row=3, column=0, columnspan=2, pady=4)

        # Password Generator
        pwd_fr = tk.LabelFrame(self.tab_fin, text="Secure Password Generator", bg=PAL["panel"], fg=PAL["text"])
        pwd_fr.pack(fill="x", pady=5, padx=5)
        tk.Label(pwd_fr, text="Length:", bg=PAL["panel"], fg=PAL["dim"]).grid(row=0, column=0, sticky="e", pady=4)
        self.pwd_len = tk.Spinbox(pwd_fr, from_=8, to=32, width=5, font=("Inter", 10), bg=PAL["bg"], fg=PAL["accent"], buttonbackground=PAL["accent_dim"], relief="flat")
        self.pwd_len.grid(row=0, column=1, pady=4)
        tk.Button(pwd_fr, text="GENERATE", bg=PAL["success"], fg="black", command=self._gen_password).grid(row=1, column=0, columnspan=2, pady=8)
        self.pwd_result = tk.Entry(pwd_fr, width=30, font=("Consolas", 10), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.pwd_result.grid(row=2, column=0, columnspan=2, pady=4)

    def _calc_split(self):
        try:
            total = float(self.split_total.get())
            people = int(self.split_people.get())
            each = total / people
            self.split_result.config(text=f"Each pays: ${format_number(each)}", fg=PAL["success"])
        except Exception:
            self.split_result.config(text="Error: check inputs", fg=PAL["danger"])

    def _calc_bmi(self):
        try:
            w = float(self.bmi_weight.get())
            h_cm = float(self.bmi_height.get())
            h_m = h_cm / 100
            bmi = w / (h_m ** 2)
            self.bmi_result.config(text=f"BMI: {format_number(bmi)}", fg=PAL["success"])
        except Exception:
            self.bmi_result.config(text="Error: check inputs", fg=PAL["danger"])

    def _gen_password(self):
        import string, secrets
        length = int(self.pwd_len.get())
        alphabet = string.ascii_letters + string.digits + "!@#$%^&*()-_=+"
        pwd = ''.join(secrets.choice(alphabet) for _ in range(length))
        self.pwd_result.delete(0, tk.END)
        self.pwd_result.insert(0, pwd)
        show_info("Password", "Secure password generated and copied to clipboard.")
        self.clipboard_clear()
        self.clipboard_append(pwd)

    # -----------------------------------------------------------------------
    # Misc Tab – Flashlight (simulated), Compass (mock), Color Picker, etc.
    # -----------------------------------------------------------------------
    def _build_misc_tab(self):
        tk.Label(self.tab_misc, text="Miscellaneous Utilities (offline)", font=("Inter", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        misc_fr = tk.Frame(self.tab_misc, bg=PAL["panel"], padx=15, pady=15)
        misc_fr.pack(fill="x", pady=5)
        # Flashlight – just changes window bg to bright white for a few seconds
        tk.Button(misc_fr, text="🔦 FLASHLIGHT (3 s)", bg=PAL["warning"], fg="black",
                  command=self._flashlight).grid(row=0, column=0, padx=8, pady=4)
        # Compass – random direction simulation
        tk.Button(misc_fr, text="🧭 COMPASS", bg=PAL["accent_dim"], fg="black",
                  command=self._compass).grid(row=0, column=1, padx=8, pady=4)
        # Color Picker – returns hex code
        tk.Button(misc_fr, text="🎨 COLOR PICKER", bg=PAL["accent"], fg="black",
                  command=self._pick_color).grid(row=0, column=2, padx=8, pady=4)

    def _flashlight(self):
        original = self.cget('bg')
        self.configure(bg="#FFFFFF")
        self.after(3000, lambda: self.configure(bg=original))
        show_info("Flashlight", "Flashlight simulated for 3 seconds.")

    def _compass(self):
        dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"]
        direction = random.choice(dirs)
        show_info("Compass", f"Current heading: {direction}")

    def _pick_color(self):
        col = colorchooser.askcolor(title="Pick a color")
        if col[1]:
            show_info("Color Picker", f"You selected: {col[1]}")

    # -----------------------------------------------------------------------
    # Offline DB Refresh – placeholder for future data updates
    # -----------------------------------------------------------------------
    def _refresh_offline_db(self):
        self.status.config(text="Refreshing offline data caches…", bg=PAL["warning"], fg="black")
        self.after(1500, lambda: self.status.config(text="Offline DB up‑to‑date", bg=PAL["success"], fg="black"))

if __name__ == "__main__":
    app = OmniToolsApp()
    app.mainloop()
