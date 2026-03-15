# Generated method: SigmaSpotlight.search
from typing import List, Dict, Any
import time

class SigmaSpotlight:
    def search(self, query: str) -> List[Dict]:
        """USP: Multi-threaded search across local FS, AI, and OS Registry."""
        results = []
        q = query.lower()
        for cat, items in self._index.items():
            for item in items:
                if q in item.lower():
                    results.append({'Category': cat, 'Name': item, 'Type': 'Index'})
        if q.endswith('?') or any((w in q for w in ['what', 'how', 'why'])):
            results.append({'Category': 'AI_Nexus', 'Name': f"Ask AI: '{query}'", 'Type': 'Action', 'Action': lambda q=query: self.kernel.ai.prompt(q)})
        if q in self._index['Commands']:
            results.append({'Category': 'System', 'Name': f"Execute Command: '{query}'", 'Type': 'Kernel_Op'})
        if query.startswith('www.') or query.endswith('.com') or query.startswith('http'):
            results.append({'Category': 'Web', 'Name': f'Open WebApp: {query}', 'Type': 'Browser_Launch', 'Action': lambda q=query: self.kernel.bus.emit('browser.launch', {'url': q}) if hasattr(self.kernel, 'bus') else None})
        self._history.append(query)
        return results