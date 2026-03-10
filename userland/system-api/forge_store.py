"""
SigmaOS Forge App Store
=======================
A high-performance, compressed, IP-Safe integrated application store.
Offers one-click downloads of open-source logic games and productivity tools.
Focuses on low footprint and instant execution.
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Any
from conversion_engine import SigmaConversionEngine
from sovereign_utility_suite import SovereignUtilitySuite
from ad_shield import SigmaAdShield
from youtube_downloader import SigmaYouTubeSovereignFetcher
from sovereign_clipboard import SigmaSovereignClipboard

class SigmaForgeStore:
    def __init__(self, kernel):
        self.kernel = kernel
        self.conversion = SigmaConversionEngine(kernel) if SigmaConversionEngine else None
        self.utils = SovereignUtilitySuite(kernel) if SovereignUtilitySuite else None
        self.shield = SigmaAdShield(kernel) if SigmaAdShield else None
        self.yt_fetcher = SigmaYouTubeSovereignFetcher(kernel) if SigmaYouTubeSovereignFetcher else None
        self.clipboard = SigmaSovereignClipboard(kernel) if SigmaSovereignClipboard else None
        # Use USERPROFILE env var so it works on any Windows user account
        _config_dir = Path(os.environ.get("USERPROFILE", os.path.expanduser("~"))) / ".sigmaos" / "config"
        self._installed_apps_file = _config_dir / "installed_apps.json"
        self._installed_apps: List[str] = self._load_installed()
        
        # IP-Safe Open Source Game Clones & Tools
        self.catalog = {
            "aether_orch": {
                "name": "Aether Orchestrator",
                "category": "AI",
                "size_mb": 4.5,
                "description": "Cross-model AI coordination and prompt engineering hub.",
                "exec": "bus.emit('app.launch.aether_orch')"
            },
            "ag_finder": {
                "name": "Antigravity Tools Finder",
                "category": "System",
                "size_mb": 0.8,
                "description": "Discover and map tools created by Antigravity.",
                "exec": "bus.emit('app.launch.ag_finder')"
            },
            "ag_physics": {
                "name": "Antigravity Engine",
                "category": "System",
                "size_mb": 2.2,
                "description": "Core physics-based UI and zero-G drift engine.",
                "exec": "bus.emit('app.launch.ag_physics')"
            },
            "email_discovery": {
                "name": "Email Discovery Agent",
                "category": "AI",
                "size_mb": 1.5,
                "description": "AI-powered email intent analysis and sorting.",
                "exec": "bus.emit('app.launch.email_discovery')"
            },
            "excel_ai": {
                "name": "Excel AI Filler",
                "category": "Productivity",
                "size_mb": 3.1,
                "description": "Automatically fill and predict spreadsheet data using LLMs.",
                "exec": "bus.emit('app.launch.excel_ai')"
            },
            "excel_preproc": {
                "name": "Excel Preprocessor",
                "category": "Productivity",
                "size_mb": 1.8,
                "description": "Clean and normalize heavy Excel datasets for AI processing.",
                "exec": "bus.emit('app.launch.excel_preproc')"
            },
            "ag_guide": {
                "name": "Guide for Antigravity Tools",
                "category": "Documentation",
                "size_mb": 0.2,
                "description": "Comprehensive manual for the Antigravity software suite.",
                "exec": "bus.emit('app.launch.ag_guide')"
            },
            "indent_flow": {
                "name": "IndentFlow",
                "category": "Coding",
                "size_mb": 0.6,
                "description": "Automatic code structure and indentation visualization.",
                "exec": "bus.emit('app.launch.indent_flow')"
            },
            "routine_dash": {
                "name": "OpenRoutines Dashboard",
                "category": "Automation",
                "size_mb": 1.4,
                "description": "Visual control for scheduled OS routines and automations.",
                "exec": "bus.emit('app.launch.routine_dash')"
            },
            "pdf_forge": {
                "name": "PDF Forge",
                "category": "Productivity",
                "size_mb": 2.7,
                "description": "High-performance PDF generation and manipulation.",
                "exec": "bus.emit('app.launch.pdf_forge')"
            },
            "pure_text": {
                "name": "Pure Text",
                "category": "Productivity",
                "size_mb": 0.3,
                "description": "Zero-format text buffer and stripping utility.",
                "exec": "bus.emit('app.launch.pure_text')"
            },
            "text_cleaner": {
                "name": "Text Cleaner",
                "category": "Productivity",
                "size_mb": 0.4,
                "description": "Advanced regex-based text normalization.",
                "exec": "bus.emit('app.launch.text_cleaner')"
            },
            "titan_capture": {
                "name": "Titan Capture",
                "category": "System",
                "size_mb": 3.2,
                "description": "High-fidelity screen and process state recording.",
                "exec": "bus.emit('app.launch.titan_capture')"
            },
            "logic_box": {
                "name": "Sigma Logic Box (Nuts & Bolts / X-O / Dots)",
                "category": "Games",
                "size_mb": 4.1,
                "description": "Collection of logic and strategy board games.",
                "exec": "bus.emit('app.launch.logic_box')"
            },
            "crowd_control": {
                "name": "Swarm Legends (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 12.0,
                "description": "Real-time crowd manipulation strategy game.",
                "exec": "bus.emit('app.launch.swarm')"
            },
            "transit_surf": {
                "name": "Subway Drifters (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 25.0,
                "description": "Infinite runner with zero-G drift mechanics.",
                "exec": "bus.emit('app.launch.surf')"
            },
            "botany_defense": {
                "name": "Bio-Defense (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 18.0,
                "description": "Defend your core using modular biological nodes.",
                "exec": "bus.emit('app.launch.bio')"
            },
            "car_park_puzzle": {
                "name": "Parking Grid Control",
                "category": "Games",
                "size_mb": 3.5,
                "description": "Color-coded logical movement puzzle.",
                "exec": "bus.emit('app.launch.parking')"
            },
            "chess_pro": {
                "name": "Sovereign Chess Engine",
                "category": "Games",
                "size_mb": 1.5,
                "description": "Adaptive IP-safe chess game with zero-G rendering.",
                "exec": "bus.emit('app.launch.chess')"
            },
            "mesh_ludo": {
                "name": "Aether Ludo (Mesh P2P)",
                "category": "Games",
                "size_mb": 2.1,
                "description": "Classic Ludo reinvented for decentralized P2P networks.",
                "exec": "bus.emit('app.launch.ludo')"
            },
            "snake_ladder": {
                "name": "Quantum Snakes & Ladders",
                "category": "Games",
                "size_mb": 1.2,
                "description": "Classic board game with teleportation mechanics.",
                "exec": "bus.emit('app.launch.snakes')"
            },
            "tic_tac": {
                "name": "Quantum Tic-Tac-Toe",
                "category": "Games",
                "size_mb": 0.5,
                "description": "3D Tic-Tac-Toe with advanced AI solver.",
                "exec": "bus.emit('app.launch.tictac')"
            },
            "merge_2048": {
                "name": "Merge 2048 (Sigma Edition)",
                "category": "Games",
                "size_mb": 1.0,
                "description": "IP-safe 2048 alternative with dark mode aesthetics.",
                "exec": "bus.emit('app.launch.merge2048')"
            },
            "ag_markdown": {
                "name": "Sovereign Markdown Studio",
                "category": "Productivity",
                "size_mb": 2.8,
                "description": "Distraction-free markdown editor with live preview.",
                "exec": "bus.emit('app.launch.md_studio')"
            },
            "net_scanner": {
                "name": "Mesh Network Scanner",
                "category": "System",
                "size_mb": 1.1,
                "description": "Map all peers on your localized sovereign mesh.",
                "exec": "bus.emit('app.launch.net_scanner')"
            },
            "pass_vault": {
                "name": "Zero-Knowledge PassVault",
                "category": "Security",
                "size_mb": 1.9,
                "description": "Offline-only, encrypted password manager.",
                "exec": "bus.emit('app.launch.pass_vault')"
            },
            "image_to_text": {
                "name": "Sovereign OCR (Image to Text)",
                "category": "Conversion",
                "size_mb": 5.2,
                "description": "Extract text from hand-drawn or digital documents locally.",
                "exec": "bus.emit('app.launch.ocr_tool')"
            },
            "text_to_html": {
                "name": "Sovereign Morph (Text to HTML)",
                "category": "Conversion",
                "size_mb": 0.8,
                "description": "Convert markdown/txt to premium glassmorphic HTML.",
                "exec": "bus.emit('app.launch.html_morph')"
            },
            "video_transcript": {
                "name": "Sovereign Transcript (Video to Text)",
                "category": "Conversion",
                "size_mb": 25.0,
                "description": "Generate on-device transcripts from video files.",
                "exec": "bus.emit('app.launch.transcribe')"
            },
            "grammar_checker": {
                "name": "Sovereign Grammarly",
                "category": "Productivity",
                "size_mb": 4.2,
                "description": "Local writing and grammar assistant.",
                "exec": "bus.emit('app.launch.grammar')"
            },
            "carbon_code": {
                "name": "Carbon: Code to Image",
                "category": "DevTools",
                "size_mb": 1.1,
                "description": "Generate beautiful code snippets locally.",
                "exec": "bus.emit('app.launch.carbon')"
            },
            "pdf_buddy": {
                "name": "iLovePDF: PDF Toolkit",
                "category": "Utilities",
                "size_mb": 8.5,
                "description": "Merge, Split, and Morph PDFs offline.",
                "exec": "bus.emit('app.launch.pdf_buddy')"
            },
            "speed_test": {
                "name": "Ookla SpeedTest Pro",
                "category": "Performance",
                "size_mb": 0.5,
                "description": "Measure raw network throughput through the Mesh.",
                "exec": "bus.emit('app.launch.speedtest')"
            },
            "rufus_flash": {
                "name": "Rufus ISO Creator",
                "category": "Utilities",
                "size_mb": 3.0,
                "description": "Create bootable USB drives with Sovereign signatures.",
                "exec": "bus.emit('app.launch.rufus')"
            },
            "brave_shield": {
                "name": "Brave-Grade AdShield",
                "category": "Security",
                "size_mb": 12.0,
                "description": "OS-wide, regional-aware blocking. No trackers allowed.",
                "exec": "bus.emit('app.launch.brave_shield')"
            },
            "yt_fetcher": {
                "name": "Sovereign YouTube Downloader",
                "category": "Media",
                "size_mb": 15.6,
                "description": "Download YT videos/audio locally without ads.",
                "exec": "bus.emit('app.launch.yt_fetcher')"
            },
            "universal_morpher": {
                "name": "Universal Format Morpher",
                "category": "Conversion",
                "size_mb": 55.0,
                "description": "Local CloudConvert: DOC to PDF, MP4 to MKV, and more.",
                "exec": "bus.emit('app.launch.universal_morpher')"
            },
            "mesh_clipboard": {
                "name": "Sovereign Clipboard (Mesh-Sync)",
                "category": "Productivity",
                "size_mb": 1.2,
                "description": "Universal Copy-Paste across your private mesh.",
                "exec": "bus.emit('app.launch.clipboard')"
            }
        }

    def _load_installed(self) -> List[str]:
        if self._installed_apps_file.exists():
            try:
                with open(self._installed_apps_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception:
                pass
        return []

    def _save_installed(self):
        self._installed_apps_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self._installed_apps_file, 'w', encoding='utf-8') as f:
            json.dump(self._installed_apps, f)

    def get_catalog(self, category: str = None) -> Dict[str, Any]:
        """Return full catalog or filter by category."""
        if category:
            return {k: v for k, v in self.catalog.items()
                    if v.get("category", "").lower() == category.lower()}
        return self.catalog

    def get_catalog_list(self, category: str = None) -> List[dict]:
        """Return catalog as a list suitable for the app store grid view."""
        filtered = self.get_catalog(category)
        result = []
        for app_id, meta in filtered.items():
            entry = dict(meta)
            entry["app_id"] = app_id
            entry["installed"] = app_id in self._installed_apps
            result.append(entry)
        return result

    def install_app(self, app_id: str) -> Dict[str, Any]:
        if app_id not in self.catalog:
            return {"status": "ERROR", "msg": "App not found."}
        if app_id in self._installed_apps:
            return {"status": "OK", "msg": "Already installed."}
        # Simulate quick download and extraction (compressed AppImage style)
        self._installed_apps.append(app_id)
        self._save_installed()
        return {"status": "SUCCESS", "msg": f"{self.catalog[app_id]['name']} installed successfully (Compressed)."}

    def launch_app(self, app_id: str) -> str:
        if app_id not in self._installed_apps:
            return f"Error: {app_id} is not installed."
        app = self.catalog.get(app_id)
        if not app:
            return "App definition missing."
        # Route through the kernel event bus safely (no eval)
        if "bus.emit" in app["exec"]:
            try:
                event_str = app["exec"].replace("bus.emit(", "").rstrip(")")
                event_name = event_str.strip("'\"")
                self.kernel.bus.emit(event_name, {"app_id": app_id})
                return f"Launched {app['name']}"
            except Exception as e:
                return f"Execution error: {e}"
        return f"Simulating launch: {app['name']}"

    def health_check(self) -> str:
        return f"OK — ForgeStore: {len(self.catalog)} apps available, {len(self._installed_apps)} installed."
