"""
SigmaAppStore: The Sovereign Application & Game Ecosystem.
=========================================================
USP: One-click, sandboxed, and decentralized application distribution.
Competes with: Mac App Store, Microsoft Store, Google Play, Steam.

Now dynamically linked to SigmaGames Engine and Linux Parity packages.
"""

import time
from typing import List, Dict
from app_sandbox import SigmaAppSandbox

class SigmaAppStore:
    def __init__(self, kernel):
        self.kernel = kernel
        self.installed_userland_apps = {} # Changed to dict: app_id -> silo_id
        self.sandbox = SigmaAppSandbox(kernel)
        self._load_catalog()

    def _load_catalog(self):
        """Unified catalog from games engine and tools registry."""
        self.catalog = {"Games": [], "Tools": []}
        
        # 1. Fetch Games from Internal Engine
        games_engine = self.kernel.games
        if games_engine:
            for g in games_engine.get_catalog_metadata():
                self.catalog["Games"].append({
                    "id": g["id"],
                    "name": g["name"],
                    "category": g["category"],
                    "size": f"{g['size_kb']/1024:.1f}MB",
                    "icon": g["icon"],
                    "description": g["desc"]
                })

        # 2. Advanced Tools & Cloud Bridges
        tools = [
            # Security & Core
            {"id": "t_kali", "name": "Sigma Pentest Lab", "category": "Security", "size": "12MB", "icon": "💀", "description": "Kali-parity network auditing toolkit."},
            {"id": "t_ubuntu", "name": "Debian-Bridge", "category": "System", "size": "4MB", "icon": "📦", "description": "Package management (apt/dnf parity)."},
            
            # Presentation & Design AI
            {"id": "t_beautiful_ai", "name": "Beautiful.ai Bridge", "category": "AI Design", "size": "8MB", "icon": "🎨", "description": "Smart slide deck automation for professional pitches."},
            {"id": "t_gamma", "name": "Gamma OS Hub", "category": "AI Design", "size": "15MB", "icon": "🪄", "description": "AI-generated documents, decks, and web pages."},
            {"id": "t_tome", "name": "Tome AI Bridge", "category": "AI Design", "size": "12MB", "icon": "📖", "description": "Generative storytelling and AI presentation builder."},
            {"id": "t_visme", "name": "Visme Pro Sync", "category": "AI Design", "size": "20MB", "icon": "🖼️", "description": "Unified platform for visual content and data viz."},
            {"id": "t_slidesgo", "name": "Slidesgo Vault", "category": "AI Design", "size": "5MB", "icon": "📑", "description": "Direct access to AI-powered presentation templates."},
            
            # Workflow & Automation Engines
            {"id": "t_zapier", "name": "Zapier Mesh", "category": "Automation", "size": "6MB", "icon": "⚡", "description": "Automate workflows across 6000+ cloud apps locally."},
            {"id": "t_make", "name": "Make (formerly Integromat)", "category": "Automation", "size": "10MB", "icon": "🌀", "description": "Visual visual workflow orchestrator for cloud apps."},
            {"id": "t_n8n", "name": "n8n Self-Host", "category": "Automation", "size": "45MB", "icon": "🐙", "description": "Next-gen self-hosted workflow automation."},
            {"id": "t_integrately", "name": "Integrately Bridge", "category": "Automation", "size": "4MB", "icon": "🔗", "description": "1-click automation sync with external SaaS apps."},
            
            # Project & Task AI
            {"id": "t_monday", "name": "Monday.com OS Sync", "category": "Management", "size": "30MB", "icon": "📅", "description": "Work OS integration for team and task management."},
            {"id": "t_notion", "name": "Notion Sovereign Lab", "category": "Management", "size": "25MB", "icon": "📓", "description": "Unified workspace for notes, docs, and projects."},
            {"id": "t_taskade", "name": "Taskade AI Agent", "category": "Management", "size": "14MB", "icon": "✅", "description": "AI productivity workspace with built-in agents."},
            {"id": "t_wrike", "name": "Wrike Hub", "category": "Management", "size": "18MB", "icon": "📈", "description": "Collaborative work management platform sync."},
            
            # Scheduling & Productivity AI
            {"id": "t_calendly", "name": "Calendly Sync", "category": "Productivity", "size": "5MB", "icon": "📅", "description": "Automated meeting scheduling integration."},
            {"id": "t_reclaim", "name": "Reclaim.ai Pilot", "category": "Productivity", "size": "7MB", "icon": "⏳", "description": "AI-powered calendar for habit and task protection."},
            {"id": "t_motion", "name": "Motion AI Sync", "category": "Productivity", "size": "11MB", "icon": "🚀", "description": "AI-automated schedule optimizer and task tracker."},
            {"id": "t_clockwise", "name": "Clockwise Catalyst", "category": "Productivity", "size": "9MB", "icon": "🕒", "description": "AI calendar assistant for focus time optimization."},
            
            # Knowledge & Data AI
            {"id": "t_mem", "name": "Mem.ai Portal", "category": "Knowledge", "size": "22MB", "icon": "🧠", "description": "Self-organizing workspace powered by AI memory."},
            {"id": "t_julius", "name": "Julius AI Data Lab", "category": "Knowledge", "size": "16MB", "icon": "📊", "description": "AI data analyst for spreadsheets and sql data."},
            {"id": "t_zing", "name": "Zing Data Sync", "category": "Knowledge", "size": "8MB", "icon": "⚡", "description": "Instant mobile-first data visualization and BI."},
            {"id": "t_tettra", "name": "Tettra Knowledge Base", "category": "Knowledge", "size": "12MB", "icon": "📂", "description": "AI-powered internal company knowledge base."},
            
            # Presentation.ai, SlideGo, DeckPilot
            {"id": "t_deckpilot", "name": "DeckPilot AI", "category": "AI Design", "size": "9MB", "icon": "🛩️", "description": "Copilot for generating high-conversion decks."},
            {"id": "t_presentation_ai", "name": "Presentation.ai", "category": "AI Design", "size": "11MB", "icon": "✨", "description": "The 'Grammarly' of presentations - AI-powered designs."},
            {"id": "t_pitch", "name": "Pitch AI Sync", "category": "AI Design", "size": "14MB", "icon": "🎤", "description": "AI-powered presentation software for modern teams."},
            {"id": "t_flourish", "name": "Flourish Data Viz", "category": "AI Design", "size": "12MB", "icon": "📊", "description": "Dynamic storytelling with data visualization."},
            
            # AI Powerhouses (Meta, Perplexity, Grok)
            {"id": "t_grok", "name": "Grok 2.0 (xAI) Hub", "category": "AI Nexus", "size": "0.1MB", "icon": "👁️", "description": "Real-time, spicy, and rebellious AI from xAI."},
            {"id": "t_meta", "name": "Meta AI (Llama 3.1)", "category": "AI Nexus", "size": "0.1MB", "icon": "♾️", "description": "Meta's most advanced AI for research and creative tasks."},
            {"id": "t_perplexity", "name": "Perplexity Pro Bridge", "category": "AI Nexus", "size": "0.1MB", "icon": "🔍", "description": "Answer engine that provides real-time citations and web data."},
            {"id": "t_popai", "name": "PopAI Personal Assistant", "category": "AI Nexus", "size": "0.1MB", "icon": "🎈", "description": "Your 24/7 AI document analyzer and creative partner."},
            {"id": "t_plus", "name": "Plus AI Design", "category": "AI Design", "size": "0.1MB", "icon": "➕", "description": "AI-powered market research and slide creation."}
        ]
        self.catalog["Tools"] = tools

    def list_userland_apps(self, category: str = None) -> Dict:
        """Returns the current catalog (can filter by category)."""
        self._load_catalog() # Refresh
        if category:
            return {category: self.catalog.get(category, [])}
        return self.catalog

    def install_app(self, app_id: str) -> str:
        """One-click 'hydration' install for apps and games."""
        app_name = "Unknown App"
        found = False
        
        # Populate catalog first
        self._load_catalog()
        
        # Search catalog
        for cat in self.catalog.values():
            for a in cat:
                if a["id"] == app_id:
                    app_name = a["name"]
                    found = True
                    break
        
        if not found:
            return f"Error: App '{app_id}' not found."

        if app_id in self.installed_userland_apps:
            return f"'{app_name}' is already installed."

        # Logic-specific installation
        if app_id.startswith("G"):
            # It's a game from games engine
            if hasattr(self.kernel, "games"):
                res = self.kernel.games.install_game(app_id)
                if res["status"] == "success":
                    self.installed_userland_apps[app_id] = "GAME_SILO"
                    return res["message"]
        
        # Create a dedicated silo for the tool
        silo_id = self.sandbox.create_silo(app_id)
        self.installed_userland_apps[app_id] = silo_id
        
        # 3. Create FileSystem Snapshot for recovery (if fs available)
        if hasattr(self.kernel, "fs"):
            self.kernel.fs.create_snapshot(f"app-install-{app_id}")

        return f"Successfully hydrated '{app_name}' and siloted in {silo_id}."

    def launch_app(self, app_id: str) -> str:
        silo_id = self.installed_userland_apps.get(app_id)
        if not silo_id:
            return f"Error: '{app_id}' is not installed."
        
        # Execute via Sandbox Silo
        msg = self.sandbox.execute_in_silo(silo_id, "start")
        return f"{app_id} Silo [ACTIVE]: {msg}"

    def health_check(self) -> str:
        return f"OK — Store: {len(self.catalog['Games'])} Games, {len(self.catalog['Tools'])} Tools | {len(self.installed_userland_apps)} Installed."
