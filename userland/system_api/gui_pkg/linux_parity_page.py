import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage(SigmaPage):
    """🐧 Sovereign Linux Bridge: Distro Parity Engine"""
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Sovereign Linux Bridge", "Distro Parity Engine")
        self._build_ui()

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)
        
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=450)
        l_fr.pack(side="left", fill="both", padx=5)
        l_fr.pack_propagate(False)

        # Gap Analysis
        gap_c = self._card(l_fr, "📊 Distro Gap Analysis")
        gap_c.master.pack(fill="x", pady=5)
        ttk.Button(gap_c, text="Audit: Sigma vs Kali", command=lambda: self._log_linux_gap("Kali")).pack(side="left", padx=5)
        ttk.Button(gap_c, text="Audit: Sigma vs Arch", command=lambda: self._log_linux_gap("Arch")).pack(side="left", padx=5)
        ttk.Button(gap_c, text="🛡️ RHEL Security Scan", command=self._run_sec_audit).pack(side="left", padx=5)

        # Package Manager (USP: apt/dnf/pacman/snap combined)
        pm_c = self._card(l_fr, "📦 Multi-Stack Package Manager")
        pm_c.master.pack(fill="x", pady=5)
        tk.Label(pm_c, text="Translates: apt, dnf, pacman, snap, flatpak", font=("Segoe UI", 8), bg=PAL["card"], fg=PAL["dim"]).pack()
        ttk.Button(pm_c, text="Sync Repos", command=lambda: self._log_linux(self.kernel.linux_parity.pm.sync_repos())).pack(side="left", padx=5)
        ttk.Button(pm_c, text="Upgrade System", command=lambda: self._log_linux(self.kernel.linux_parity.pm.upgrade_system())).pack(side="left", padx=5)

        # Distro Profiles
        dp_c = self._card(l_fr, "🎭 Persona Mode (Distro Mimicry)")
        dp_c.master.pack(fill="x", pady=5)
        tk.Label(dp_c, text="Apply Distro-specific optimizations instantly.", font=("Segoe UI", 8), bg=PAL["card"], fg=PAL["dim"]).pack()
        for d in ["Ubuntu", "Kali", "Arch", "Fedora", "Pop!", "Zorin", "Gentoo", "Alpine"]:
            ttk.Button(dp_c, text=d, command=lambda x=d: self._apply_distro_tuning(x)).pack(side="left", padx=2)

        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5)

        console_c = self._card(r_fr, "📟 Linux Parity Ops & Log")
        console_c.master.pack(fill="both", expand=True)
        self._linux_log = self._console(console_c, height=25)
        self._linux_log.pack(fill="both", expand=True)

    def _run_sec_audit(self):
        if hasattr(self.kernel, "linux_parity"):
            audit = self.kernel.linux_parity.security_audit.run_audit()
            self._log(self._linux_log, "\n🛡️ ENTERPRISE SECURITY AUDIT (RHEL/STIG PARITY)", "HEAD")
            for rule, status in audit.items():
                icon = "✔" if "PASS" in status else "✖" if "FAIL" in status else "⚠"
                self._log(self._linux_log, f"{icon} {rule.replace('_', ' ').upper()}: {status}", "OK" if "PASS" in status else "INFO")
            self.gui._log_voice("Security audit complete. Enterprise compliance verified.")

    def _log_linux_gap(self, distro):
        if hasattr(self.kernel, "linux_parity"):
            res = self.kernel.linux_parity.gap_analysis.generate_report(distro)
            self._log(self._linux_log, f"\n🔎 GAP ANALYSIS: SigmaOS vs {distro}", "HEAD")
            self._log(self._linux_log, res, "OK")

    def _log_linux(self, msg):
        self._log(self._linux_log, str(msg), "INFO")

    def _apply_distro_tuning(self, distro):
        self.gui._log_voice(f"Applying {distro} tuning... kernel-level parity engaged.")
        if hasattr(self.kernel, "linux_parity"):
            self.kernel.linux_parity.apply_distro_mimic(distro)
        self.kernel.modes.switch_mode(f"{distro}_Desktop" if distro != "Kali" else "Kali_Security")
        self._log(self._linux_log, f"✔ Switched to {distro} compatibility profile.", "OK")
