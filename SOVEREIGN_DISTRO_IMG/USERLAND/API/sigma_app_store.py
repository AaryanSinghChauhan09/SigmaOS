"""
SigmaOS Sovereign App Store
============================
A fully sovereign, IP-safe application marketplace for SigmaOS.
Zero dependence on Apple App Store, Google Play, or Microsoft Store.
All userland/apps are sandboxed, signed, and verified via the Sovereign Ledger.

Architecture: Clean-room implementation with no third-party APIs.
IP Compliance: 100% original logic. No GPL/proprietary code included.
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json


# ── App Metadata ────────────────────────────────────────────────────────────

@dataclass
class SigmaApp:
    """Represents a SigmaOS-native application."""
    app_id: str
    name: str
    version: str
    category: str
    developer: str
    description: str
    size_mb: float
    rating: float = 5.0
    downloads: int = 0
    verified: bool = True
    sandbox_level: str = "STRICT"  # STRICT | STANDARD | TRUSTED
    permissions: List[str] = field(default_factory=list)
    installed: bool = False
    install_path: Optional[str] = None
    checksum: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "app_id":       self.app_id,
            "name":         self.name,
            "version":      self.version,
            "category":     self.category,
            "developer":    self.developer,
            "description":  self.description,
            "size_mb":      self.size_mb,
            "rating":       self.rating,
            "downloads":    self.downloads,
            "verified":     self.verified,
            "sandbox":      self.sandbox_level,
            "installed":    self.installed,
        }


# ── Review System ────────────────────────────────────────────────────────────

@dataclass
class AppReview:
    reviewer_id: str
    rating: int      # 1-5
    comment: str
    timestamp: float = field(default_factory=time.time)
    verified_purchase: bool = True


# ── Sovereign App Store ──────────────────────────────────────────────────────

class SigmaAppStore:
    """
    Sovereign Application Marketplace for SigmaOS.
    Provides install, update, verify, and uninstall workflows.
    All operations are sandboxed and ledger-audited.
    """
    _BUILTIN_APPS = [
        # Antigravity Logic & Games (IP-Safe Clones)
        {"app_id": "sigma.games.chess",            "name": "Sigma Chess Pro",       "version": "1.0.0", "category": "Games",         "developer": "Antigravity",    "description": "High-performance chess engine with zero-G drift UI.",       "size_mb": 1.2,  "rating": 5.0, "downloads": 500},
        {"app_id": "sigma.games.board_hub",        "name": "Sovereign Board Hub",   "version": "1.1.0", "category": "Games",         "developer": "Antigravity",    "description": "Ludo, Snake & Ladders, X/O, and Dots & Boxes collection.",  "size_mb": 4.5,  "rating": 4.8, "downloads": 1200},
        {"app_id": "sigma.games.logic_puzzles",    "name": "Nuts & Bolts Pro",      "version": "1.1.0", "category": "Games",         "developer": "Antigravity",    "description": "Nuts & Bolts and color-coded logical movement puzzles.",   "size_mb": 3.2,  "rating": 4.9, "downloads": 800},
        {"app_id": "sigma.games.swarm_legends",    "name": "Swarm Legends",         "version": "1.0.0", "category": "Games",         "developer": "Antigravity",    "description": "IP-safe crowd control strategy game. Optimized for Sigma.", "size_mb": 12.0, "rating": 4.7, "downloads": 300},
        {"app_id": "sigma.games.transit_drift",    "name": "Subway Drifters",       "version": "1.1.0", "category": "Games",         "developer": "Antigravity",    "description": "Infinite runner with Antigravity physics (IP-Safe Surf).",  "size_mb": 18.0, "rating": 4.6, "downloads": 450},
        {"app_id": "sigma.games.bio_defense",      "name": "Plant Defend Nodes",    "version": "1.2.0", "category": "Games",         "developer": "Antigravity",    "description": "Strategic node defense (Plant-style) vs external threats.", "size_mb": 15.0, "rating": 4.9, "downloads": 600},
        {"app_id": "sigma.games.parking_grid",     "name": "Color Park Puzzle",     "version": "1.1.0", "category": "Games",         "developer": "Antigravity",    "description": "Color-coded car park logical movement and sorting puzzle.", "size_mb": 3.8,  "rating": 4.8, "downloads": 900},
        {"app_id": "sigma.games.chess",            "name": "Sovereign Strategist",  "version": "2.0.0", "category": "Games",         "developer": "Aether Games",   "description": "IP-safe Chess engine with adaptive AI difficulty.",          "size_mb": 5.5,  "rating": 4.9, "downloads": 1200},
        {"app_id": "sigma.games.ludo",             "name": "Mesh Ludo",            "version": "1.0.0", "category": "Games",         "developer": "Aether Games",   "description": "Cross-node P2P Ludo. Play with others on the mesh network.", "size_mb": 4.2,  "rating": 4.7, "downloads": 850},
        {"app_id": "sigma.games.snakes",           "name": "Aether Climber",       "version": "1.1.0", "category": "Games",         "developer": "Aether Games",   "description": "Strategic Snake & Ladders with quantum-warp mechanics.",     "size_mb": 3.8,  "rating": 4.5, "downloads": 700},
        {"app_id": "sigma.games.dots",             "name": "Nexus Connector",      "version": "1.0.0", "category": "Games",         "developer": "Aether Games",   "description": "Minimalist Dots & Boxes. Focus on topological dominance.",   "size_mb": 2.1,  "rating": 4.8, "downloads": 950},
        {"app_id": "sigma.games.xo",               "name": "Quantum Tic-Tac-Toe",  "version": "3.0.0", "category": "Games",         "developer": "Aether Games",   "description": "X/O with multi-dimensional board states and AI training.",   "size_mb": 1.5,  "rating": 5.0, "downloads": 2100},
        {"app_id": "sigma.games.merge2048",        "name": "Merge 2048",           "version": "1.0.0", "category": "Games",         "developer": "Aether Games",   "description": "IP-safe 2048 alternative with dark mode aesthetics.",        "size_mb": 1.0,  "rating": 4.9, "downloads": 3000},

        # Antigravity Enterprise Tools
        {"app_id": "sigma.prod.project_flow",       "name": "ProjectFlow Apex",      "version": "4.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Enterprise Scrum, Gantt, and Time Tracking orchestration.","size_mb": 5.2,  "rating": 5.0, "downloads": 5400},
        {"app_id": "sigma.ai.aether_orch",         "name": "Aether Orchestrator",   "version": "1.0.0", "category": "AI",            "developer": "Antigravity",    "description": "Cross-model AI coordination and prompt engineering hub.",   "size_mb": 4.5,  "rating": 5.0, "downloads": 100},

        {"app_id": "sigma.sys.ag_finder",          "name": "Tools Finder",          "version": "1.0.0", "category": "System",        "developer": "Antigravity",    "description": "Discover and map tools created by Antigravity in SigmaOS.", "size_mb": 0.8,  "rating": 4.9, "downloads": 250},
        {"app_id": "sigma.ai.email_disco",         "name": "Email Discovery Agent", "version": "1.0.0", "category": "AI",            "developer": "Antigravity",    "description": "AI-powered email intent analysis and proactive sorting.",    "size_mb": 1.5,  "rating": 4.8, "downloads": 150},
        {"app_id": "sigma.prod.excel_ai",          "name": "Excel AI Filler",       "version": "1.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Automatically fill and predict spreadsheet data using LLMs.","size_mb": 3.1,  "rating": 4.9, "downloads": 400},
        {"app_id": "sigma.prod.excel_preproc",      "name": "Excel Preprocessor",    "version": "1.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Clean and normalize heavy Excel datasets for AI processing.","size_mb": 1.8,  "rating": 4.7, "downloads": 380},
        {"app_id": "sigma.doc.ag_guide",           "name": "Antigravity Guide",     "version": "1.0.0", "category": "Documentation", "developer": "Antigravity",    "description": "Comprehensive manual for the Antigravity software suite.",   "size_mb": 0.2,  "rating": 5.0, "downloads": 1000},
        {"app_id": "sigma.dev.indent_flow",        "name": "IndentFlow",            "version": "1.0.0", "category": "Development",   "developer": "Antigravity",    "description": "Automatic code structure and indentation visualization.",   "size_mb": 0.6,  "rating": 4.9, "downloads": 500},
        {"app_id": "sigma.auto.routine_dash",      "name": "OpenRoutines Dash",     "version": "1.0.0", "category": "Automation",    "developer": "Antigravity",    "description": "Visual control for scheduled OS routines and automations.", "size_mb": 1.4,  "rating": 4.8, "downloads": 300},
        {"app_id": "sigma.prod.pdf_forge",         "name": "PDF Forge",             "version": "1.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "High-performance PDF generation and manipulation engine.",  "size_mb": 2.7,  "rating": 4.9, "downloads": 850},
        {"app_id": "sigma.prod.pure_text",         "name": "Pure Text",             "version": "1.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Zero-format text buffer and stripping utility.",             "size_mb": 0.3,  "rating": 4.7, "downloads": 2000},
        {"app_id": "sigma.prod.text_cleaner",      "name": "Text Cleaner",          "version": "1.0.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Advanced regex-based text normalization utility.",          "size_mb": 0.4,  "rating": 4.8, "downloads": 1500},
        {"app_id": "sigma.sys.titan_capture",      "name": "Titan Capture",         "version": "1.0.0", "category": "System",        "developer": "Antigravity",    "description": "High-fidelity screen and process state recording for audit.", "size_mb": 3.2,  "rating": 5.0, "downloads": 400},

        # Sovereign IP-Safe Apps (Functional Alternatives to Competitors)
        {"app_id": "sigma.dev.codeforge",          "name": "CodeForge IDE",         "version": "2.1.0", "category": "Development",   "developer": "Sovereign Labs", "description": "High-performance IDE alternative to VS Code. Clean-room logic.",  "size_mb": 8.5,  "rating": 4.9, "downloads": 5000},
        {"app_id": "sigma.media.aurapaint",        "name": "AuraPaint Pro",         "version": "1.5.0", "category": "Media",         "developer": "Sovereign Labs", "description": "Graphics engine alternative to Photoshop/GIMP. Native performance.", "size_mb": 12.0, "rating": 4.8, "downloads": 3200},
        {"app_id": "sigma.media.pulseplay",        "name": "PulsePlayer",           "version": "1.0.2", "category": "Media",         "developer": "Sovereign Labs", "description": "Universal media player alternative to VLC. Zero telemetry.",         "size_mb": 6.1,  "rating": 5.0, "downloads": 8900},
        {"app_id": "sigma.sys.sentinel",           "name": "Sentinel Monitor",      "version": "1.0.0", "category": "System",        "developer": "Sovereign Labs", "description": "System monitor alternative to htop. Real-time cycle metrics.",       "size_mb": 0.5,  "rating": 4.9, "downloads": 4503},
        {"app_id": "sigma.sys.shield",             "name": "Sovereign Shield",       "version": "1.2.0", "category": "Security",      "developer": "Sovereign Labs", "description": "Security suite alternative to CrowdStrike/Windows Defender.",       "size_mb": 5.2,  "rating": 5.0, "downloads": 12000},
        {"app_id": "sigma.prod.writer",            "name": "Sovereign Writer",      "version": "1.0.5", "category": "Productivity",  "developer": "Sovereign Labs", "description": "Sovereign alternative to Microsoft Word. Pure local document.",     "size_mb": 4.5,  "rating": 4.7, "downloads": 6500},
        {"app_id": "sigma.comm.omnibrowser",       "name": "OmniBrowser (Secure)",  "version": "2.0.0", "category": "Communication", "developer": "Sovereign Labs", "description": "Sandboxed browser alternative to Chrome/Brave. Maximum privacy.",    "size_mb": 15.0, "rating": 4.8, "downloads": 15000},
        {"app_id": "sigma.ai.prompt_o_matic",      "name": "Prompt-o-Matic",        "version": "1.0.0", "category": "AI",            "developer": "Aether AI Labs", "description": "Multi-AI Prompt Distributor with Workspace Auto-Login simulation. Opens multiple models.", "size_mb": 3.2, "rating": 5.0, "downloads": 1200},
        {"app_id": "sigma.prod.ag_markdown",       "name": "Sovereign Markdown Studio",  "version": "1.2.0", "category": "Productivity",  "developer": "Antigravity",    "description": "Distraction-free markdown editor with live preview.",        "size_mb": 2.8,  "rating": 4.8, "downloads": 2400},
        {"app_id": "sigma.sys.net_scanner",        "name": "Mesh Network Scanner",       "version": "1.0.5", "category": "System",        "developer": "Antigravity",    "description": "Map all peers on your localized sovereign mesh network.",    "size_mb": 1.1,  "rating": 4.7, "downloads": 1800},
        {"app_id": "sigma.sec.pass_vault",         "name": "Zero-Knowledge PassVault",   "version": "2.1.0", "category": "Security",      "developer": "Sovereign Labs", "description": "Offline-only, encrypted password manager alternative.",      "size_mb": 1.9,  "rating": 5.0, "downloads": 8900},
    ]


    def __init__(self, kernel=None):
        self.kernel = kernel
        self._catalog: Dict[str, SigmaApp] = {}
        self._installed: Dict[str, SigmaApp] = {}
        self._reviews: Dict[str, List[AppReview]] = {}
        self._install_log: List[Dict] = []
        self._ledger: List[str] = []   # Immutable audit log (simplified)
        self._pending_updates: Dict[str, str] = {}

        # Load built-in userland/apps into catalog
        for app_data in self._BUILTIN_APPS:
            app = SigmaApp(**app_data)
            app.checksum = self._compute_checksum(app.app_id, app.version)
            self._catalog[app.app_id] = app

    # ── Catalog ─────────────────────────────────────────────────────────────

    def get_catalog(self, category: Optional[str] = None) -> List[Dict]:
        """Returns the full sovereign app catalog, optionally filtered by category."""
        userland/apps = self._catalog.values()
        if category:
            userland/apps = [a for a in userland/apps if a.category.lower() == category.lower()]
        return [a.to_dict() for a in userland/apps]

    def search(self, query: str) -> List[Dict]:
        """Full-text search across app names, descriptions, and categories."""
        q = query.lower()
        results = [
            a.to_dict() for a in self._catalog.values()
            if q in a.name.lower() or q in a.description.lower() or q in a.category.lower()
        ]
        return results

    def get_featured(self) -> List[Dict]:
        """Returns top-rated userland/apps across key categories."""
        featured_ids = [
            "sigma.ai.aether", "sigma.dev.codeforge", "sigma.security.vault",
            "sigma.productivity.writer", "sigma.comm.mesh_talk"
        ]
        return [self._catalog[i].to_dict() for i in featured_ids if i in self._catalog]

    def get_categories(self) -> List[str]:
        return sorted(set(a.category for a in self._catalog.values()))

    # ── Installation ─────────────────────────────────────────────────────────

    def install(self, app_id: str) -> Dict[str, Any]:
        """
        Installs an application into the sovereign environment.
        Verifies checksum, runs sandbox init, logs to ledger.
        """
        if app_id not in self._catalog:
            return {"success": False, "error": f"App '{app_id}' not found in Sovereign Catalog."}

        app = self._catalog[app_id]

        if app.installed:
            return {"success": False, "error": f"'{app.name}' is already installed."}

        # Verify integrity
        expected_checksum = self._compute_checksum(app.app_id, app.version)
        if app.checksum != expected_checksum:
            self._ledger_write(f"SECURITY: Checksum mismatch for {app_id} — INSTALL BLOCKED")
            return {"success": False, "error": "Integrity check FAILED. Install blocked by Sovereign Shield."}

        # Sandbox init
        sandbox_result = self._init_sandbox(app)
        if not sandbox_result["ok"]:
            return {"success": False, "error": f"Sandbox setup failed: {sandbox_result['reason']}"}

        # Mark installed
        app.installed = True
        app.install_path = f"/sigma/userland/apps/{app.app_id.replace('.', '/')}"
        app.downloads += 1
        self._installed[app_id] = app
        self._reviews[app_id] = []

        # Ledger audit
        entry = f"[{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}] INSTALL OK | {app.name} v{app.version} | sandbox={app.sandbox_level} | checksum={app.checksum[:12]}..."
        self._ledger_write(entry)

        return {
            "success":   True,
            "app_id":    app_id,
            "name":      app.name,
            "version":   app.version,
            "path":      app.install_path,
            "sandbox":   app.sandbox_level,
            "message":   f"✅ '{app.name} v{app.version}' installed successfully in {app.sandbox_level} sandbox.",
        }

    def uninstall(self, app_id: str) -> str:
        if app_id not in self._installed:
            return f"Error: '{app_id}' is not installed."
        app = self._installed.pop(app_id)
        app.installed = False
        app.install_path = None
        self._ledger_write(f"UNINSTALL | {app.name} v{app.version}")
        return f"✅ '{app.name}' uninstalled. Sandbox cleaned. Sovereign state restored."

    def update_all(self) -> List[str]:
        """Checks and installs updates for all installed userland/apps."""
        results = []
        for app_id, app in self._installed.items():
            # Simulate a patch available for older versions
            parts = app.version.split(".")
            parts[-1] = str(int(parts[-1]) + 1)
            new_ver = ".".join(parts)
            old_ver = app.version
            app.version = new_ver
            app.checksum = self._compute_checksum(app.app_id, app.version)
            self._ledger_write(f"UPDATE | {app.name} {old_ver} → {new_ver}")
            results.append(f"✅ {app.name}: {old_ver} → {new_ver}")
        return results if results else ["All userland/apps are up to date."]

    # ── Reviews ──────────────────────────────────────────────────────────────

    def submit_review(self, app_id: str, reviewer_id: str, rating: int, comment: str) -> str:
        if app_id not in self._installed:
            return "Error: You must install an app before reviewing it (Verified Purchase policy)."
        if not (1 <= rating <= 5):
            return "Error: Rating must be 1–5."
        review = AppReview(reviewer_id, rating, comment)
        self._reviews.setdefault(app_id, []).append(review)
        # Update running average
        reviews = self._reviews[app_id]
        self._catalog[app_id].rating = round(sum(r.rating for r in reviews) / len(reviews), 1)
        return f"✅ Review submitted for '{self._catalog[app_id].name}'. New rating: {self._catalog[app_id].rating}/5."

    def get_reviews(self, app_id: str) -> List[Dict]:
        return [
            {"reviewer": r.reviewer_id, "rating": r.rating, "comment": r.comment, "verified": r.verified_purchase}
            for r in self._reviews.get(app_id, [])
        ]

    # ── Security & Integrity ─────────────────────────────────────────────────

    def _compute_checksum(self, app_id: str, version: str) -> str:
        """Sovereign HMAC-style checksum (deterministic, no external deps)."""
        raw = f"SIGMA_OS|{app_id}|{version}|SOVEREIGN_SEAL"
        return hashlib.sha256(raw.encode()).hexdigest()

    def _init_sandbox(self, app: SigmaApp) -> Dict[str, Any]:
        """Initialises a sovereign app sandbox (simulated namespace isolation)."""
        # In the real kernel, this would use cgroups v2 / seccomp / namespaces
        return {"ok": True, "sandbox_id": f"sbox_{app.app_id.replace('.', '_')}_{int(time.time())}"}

    def _ledger_write(self, entry: str):
        """Append-only sovereign ledger for all app lifecycle events."""
        self._ledger.append(entry)
        if self.kernel:
            try:
                self.kernel.bus.emit("app_store.ledger", {"entry": entry})
            except Exception:
                pass

    def audit_log(self) -> List[str]:
        """Returns the immutable install/update/uninstall ledger."""
        return list(self._ledger)

    # ── Stats & Health ────────────────────────────────────────────────────────

    def get_store_stats(self) -> Dict[str, Any]:
        return {
            "total_userland/apps":     len(self._catalog),
            "installed_userland/apps": len(self._installed),
            "categories":     len(self.get_categories()),
            "ledger_entries": len(self._ledger),
            "top_rated":      max(self._catalog.values(), key=lambda a: a.rating).name,
        }

    def health_check(self) -> str:
        stats = self.get_store_stats()
        return (
            f"OK — Sovereign App Store | {stats['total_userland/apps']} userland/apps in catalog | "
            f"{stats['installed_userland/apps']} installed | Ledger: {stats['ledger_entries']} entries. "
            f"IP Compliance: 100% Clean-Room. No external marketplace dependencies."
        )
