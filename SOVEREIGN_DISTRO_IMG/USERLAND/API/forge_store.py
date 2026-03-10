"""
SigmaOS Forge App Store
=======================
A high-performance, compressed, IP-Safe integrated application store.
Offers one-click downloads of open-source logic games and productivity tools.
Focuses on low footprint and instant execution.
"""

import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaForgeStore:
    def __init__(self, kernel):
        self.kernel = kernel
        self.installed_apps_file = Path(r'SIGMA_VIRTUAL_ROOT\.gemini\antigravity\scratch\SigmaOS\config\installed_apps.json')
        self.installed_apps = self._load_installed()
        
        # IP-Safe Open Source Game Clones & Tools
        self.catalog = {
            "aether_orch": {
                "name": "Aether Orchestrator",
                "category": "AI",
                "size_mb": 4.5,
                "description": "Cross-model AI coordination and prompt engineering hub.",
                "exec": "self.kernel.bus.emit('app.launch.aether_orch')"
            },
            "ag_finder": {
                "name": "Antigravity Tools Finder",
                "category": "System",
                "size_mb": 0.8,
                "description": "Discover and map tools created by Antigravity.",
                "exec": "self.kernel.bus.emit('app.launch.ag_finder')"
            },
            "ag_physics": {
                "name": "Antigravity Engine",
                "category": "System",
                "size_mb": 2.2,
                "description": "Core physics-based UI and zero-G drift engine.",
                "exec": "self.kernel.bus.emit('app.launch.ag_physics')"
            },
            "email_discovery": {
                "name": "Email Discovery Agent",
                "category": "AI",
                "size_mb": 1.5,
                "description": "AI-powered email intent analysis and sorting.",
                "exec": "self.kernel.bus.emit('app.launch.email_discovery')"
            },
            "excel_ai": {
                "name": "Excel AI Filler",
                "category": "Productivity",
                "size_mb": 3.1,
                "description": "Automatically fill and predict spreadsheet data using LLMs.",
                "exec": "self.kernel.bus.emit('app.launch.excel_ai')"
            },
            "excel_preproc": {
                "name": "Excel Preprocessor",
                "category": "Productivity",
                "size_mb": 1.8,
                "description": "Clean and normalize heavy Excel datasets for AI processing.",
                "exec": "self.kernel.bus.emit('app.launch.excel_preproc')"
            },
            "ag_guide": {
                "name": "Guide for Antigravity Tools",
                "category": "Documentation",
                "size_mb": 0.2,
                "description": "Comprehensive manual for the Antigravity software suite.",
                "exec": "self.kernel.bus.emit('app.launch.ag_guide')"
            },
            "indent_flow": {
                "name": "IndentFlow",
                "category": "Coding",
                "size_mb": 0.6,
                "description": "Automatic code structure and indentation visualization.",
                "exec": "self.kernel.bus.emit('app.launch.indent_flow')"
            },
            "routine_dash": {
                "name": "OpenRoutines Dashboard",
                "category": "Automation",
                "size_mb": 1.4,
                "description": "Visual control for scheduled OS routines and automations.",
                "exec": "self.kernel.bus.emit('app.launch.routine_dash')"
            },
            "pdf_forge": {
                "name": "PDF Forge",
                "category": "Productivity",
                "size_mb": 2.7,
                "description": "High-performance PDF generation and manipulation.",
                "exec": "self.kernel.bus.emit('app.launch.pdf_forge')"
            },
            "pure_text": {
                "name": "Pure Text",
                "category": "Productivity",
                "size_mb": 0.3,
                "description": "Zero-format text buffer and stripping utility.",
                "exec": "self.kernel.bus.emit('app.launch.pure_text')"
            },
            "text_cleaner": {
                "name": "Text Cleaner",
                "category": "Productivity",
                "size_mb": 0.4,
                "description": "Advanced regex-based text normalization.",
                "exec": "self.kernel.bus.emit('app.launch.text_cleaner')"
            },
            "titan_capture": {
                "name": "Titan Capture",
                "category": "System",
                "size_mb": 3.2,
                "description": "High-fidelity screen and process state recording.",
                "exec": "self.kernel.bus.emit('app.launch.titan_capture')"
            },
            "logic_box": {
                "name": "Sigma Logic Box (Nuts & Bolts / X-O / Dots)",
                "category": "Games",
                "size_mb": 4.1,
                "description": "Collection of logic and strategy board games.",
                "exec": "self.kernel.bus.emit('app.launch.logic_box')"
            },
            "crowd_control": {
                "name": "Swarm Legends (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 12.0,
                "description": "Real-time crowd manipulation strategy game.",
                "exec": "self.kernel.bus.emit('app.launch.swarm')"
            },
            "transit_surf": {
                "name": "Subway Drifters (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 25.0,
                "description": "Infinite runner with zero-G drift mechanics.",
                "exec": "self.kernel.bus.emit('app.launch.surf')"
            },
            "botany_defense": {
                "name": "Bio-Defense (IP-Safe Clone)",
                "category": "Games",
                "size_mb": 18.0,
                "description": "Defend your core using modular biological nodes.",
                "exec": "self.kernel.bus.emit('app.launch.bio')"
            },
            "car_park_puzzle": {
                "name": "Parking Grid Control",
                "category": "Games",
                "size_mb": 3.5,
                "description": "Color-coded logical movement puzzle.",
                "exec": "self.kernel.bus.emit('app.launch.parking')"
            },
            "chess_pro": {
                "name": "Sovereign Chess Engine",
                "category": "Games",
                "size_mb": 1.5,
                "description": "Adaptive IP-safe chess game with zero-G rendering.",
                "exec": "self.kernel.bus.emit('app.launch.chess')"
            },
            "mesh_ludo": {
                "name": "Aether Ludo (Mesh P2P)",
                "category": "Games",
                "size_mb": 2.1,
                "description": "Classic Ludo reinvented for decentralized P2P networks.",
                "exec": "self.kernel.bus.emit('app.launch.ludo')"
            },
            "snake_ladder": {
                "name": "Quantum Snakes & Ladders",
                "category": "Games",
                "size_mb": 1.2,
                "description": "Classic board game with teleportation mechanics.",
                "exec": "self.kernel.bus.emit('app.launch.snakes')"
            },
            "tic_tac": {
                "name": "Quantum Tic-Tac-Toe",
                "category": "Games",
                "size_mb": 0.5,
                "description": "3D Tic-Tac-Toe with advanced AI solver.",
                "exec": "self.kernel.bus.emit('app.launch.tictac')"
            },
            "merge_2048": {
                "name": "Merge 2048 (Sigma Edition)",
                "category": "Games",
                "size_mb": 1.0,
                "description": "IP-safe 2048 alternative with dark mode aesthetics.",
                "exec": "self.kernel.bus.emit('app.launch.merge2048')"
            },
            "ag_markdown": {
                "name": "Sovereign Markdown Studio",
                "category": "Productivity",
                "size_mb": 2.8,
                "description": "Distraction-free markdown editor with live preview.",
                "exec": "self.kernel.bus.emit('app.launch.md_studio')"
            },
            "net_scanner": {
                "name": "Mesh Network Scanner",
                "category": "System",
                "size_mb": 1.1,
                "description": "Map all peers on your localized sovereign mesh.",
                "exec": "self.kernel.bus.emit('app.launch.net_scanner')"
            },
            "pass_vault": {
                "name": "Zero-Knowledge PassVault",
                "category": "Security",
                "size_mb": 1.9,
                "description": "Offline-only, encrypted password manager.",
                "exec": "self.kernel.bus.emit('app.launch.pass_vault')"
            }
        }

    def _load_installed(self) -> List[str]:
        if self.installed_apps_file.exists():
            try:
                with open(self.installed_apps_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except: pass
        return []

    def _save_installed(self):
        self.installed_apps_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self.installed_apps_file, 'w', encoding='utf-8') as f:
            json.dump(self.installed_apps, f)

    def get_catalog(self) -> Dict[str, Any]:
        return self.catalog

    def install_app(self, app_id: str) -> Dict[str, Any]:
        if app_id not in self.catalog:
            return {"status": "ERROR", "msg": "App not found."}
        if app_id in self.installed_apps:
            return {"status": "OK", "msg": "Already installed."}
            
        # Simulate quick download and extraction (compressed AppImage style)
        self.installed_apps.append(app_id)
        self._save_installed()
        return {"status": "SUCCESS", "msg": f"{self.catalog[app_id]['name']} installed successfully (Compressed)."}

    def launch_app(self, app_id: str) -> str:
        if app_id not in self.installed_apps:
            return f"Error: {app_id} is not installed."
        app = self.catalog.get(app_id)
        if not app:
            return "App definition missing."
        
        # In a generic environment, eval is risky, but this is an internal router map
        if "bus.emit" in app['exec']:
            try:
                eval(app['exec'])
                return f"Launched {app['name']}"
            except Exception as e:
                return f"Execution error: {e}"
        return f"Simulating launch: {app['name']}"
