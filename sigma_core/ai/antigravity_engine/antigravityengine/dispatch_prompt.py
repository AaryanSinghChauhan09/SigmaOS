# Generated method: AntigravityEngine.dispatch_prompt
import urllib.parse
import webbrowser
import threading
import time
import json
import os
from typing import List, Dict, Any, Optional
from sigma_core.ai.antigravity_manifest import PLATFORMS, QUOTA_DEFAULTS

class AntigravityEngine:
    def dispatch_prompt(self, prompt: str, selected_names: List[str]):
        """USP: Atomic multi-node prompt dispatch with browser-staggering."""
        ts = time.strftime('%Y-%m-%d %H:%M:%S')
        record = {'time': ts, 'prompt': prompt, 'platforms': selected_names, 'status': 'DISPATCHED'}
        self.history.append(record)
        q = urllib.parse.quote_plus(prompt)
        url_templates = {'ChatGPT': f'https://chatgpt.com/?q={q}', 'Claude': f'https://claude.ai/new?q={q}', 'Gemini': f'https://gemini.google.com/app?q={q}', 'Perplexity': f'https://perplexity.ai/search?q={q}', 'DeepSeek': f'https://chat.deepseek.com/?q={q}', 'Mistral': f'https://chat.mistral.ai/chat?q={q}'}

        def _batch_open():
            for name in selected_names:
                plat = next((p for p in self.platforms if p['name'] == name), None)
                if plat:
                    url = url_templates.get(name, plat['url'])
                    webbrowser.open(url)
                    time.sleep(0.3)
        threading.Thread(target=_batch_open, daemon=True).start()
        return record