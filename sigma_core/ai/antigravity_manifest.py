"""
SigmaOS Antigravity AI Platform Manifest (v1.0 Apex)
=====================================================
Centralized registry for AI platforms, icons, colors, and quota defaults.
"""
from typing import List, Dict, Any

PLATFORMS = [
    {"name": "ChatGPT",        "url": "https://chatgpt.com",              "color": "#10A37F", "tier": 1, "icon": "🤖"},
    {"name": "Claude",         "url": "https://claude.ai",                "color": "#FF6B35", "tier": 1, "icon": "🔶"},
    {"name": "Gemini",         "url": "https://gemini.google.com",        "color": "#4285F4", "tier": 1, "icon": "♊"},
    {"name": "Perplexity",     "url": "https://perplexity.ai",            "color": "#1C1C1C", "tier": 1, "icon": "🔍"},
    {"name": "Copilot",        "url": "https://copilot.microsoft.com",    "color": "#0078D4", "tier": 1, "icon": "🪟"},
    {"name": "Grok",           "url": "https://grok.x.ai",                "color": "#1DA1F2", "tier": 1, "icon": "𝕏"},
    {"name": "Mistral",        "url": "https://chat.mistral.ai",          "color": "#7480FF", "tier": 2, "icon": "🌪️"},
    {"name": "Mistral Large 2","url": "https://chat.mistral.ai",          "color": "#7480FF", "tier": 1, "icon": "🌋"},
    {"name": "DeepSeek",       "url": "https://chat.deepseek.com",        "color": "#007BFF", "tier": 1, "icon": "🐋"},
    {"name": "LMArena",        "url": "https://lmarena.ai",               "color": "#E91E63", "tier": 2, "icon": "⚔️"},
    {"name": "Meta AI",        "url": "https://meta.ai",                  "color": "#0668E1", "tier": 2, "icon": "🌐"},
    {"name": "Ollama",         "url": "http://localhost:11434",           "color": "#FFFFFF", "tier": 2, "icon": "🦙"},
]

QUOTA_DEFAULTS = {
    "ChatGPT":         {"limit": 40,   "used": 0, "unit": "msgs/3h",  "pro": True},
    "Claude":          {"limit": 45,   "used": 0, "unit": "msgs/5h",  "pro": False},
    "Gemini":          {"limit": 60,   "used": 0, "unit": "msgs/day", "pro": False},
    "DeepSeek":        {"limit": 100,  "used": 0, "unit": "msgs/day", "pro": True},
    "Mistral":         {"limit": 50,   "used": 0, "unit": "msgs/day", "pro": False},
}
