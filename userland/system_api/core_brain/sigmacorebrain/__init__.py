# Generated method: SigmaCoreBrain.__init__
from typing import Dict, List, Any
import json

class SigmaCoreBrain:
    def __init__(self, kernel):
        self.kernel = kernel
        self._rules = {'Sovereignty': 'Always prefer local compute and open standards.', 'Independence': 'Avoid vendor-specific lock-in; use abstract adapters.', 'Zero_Trust': 'Verify all external data before ingestion into Sigma-FS.'}
        self._prompt_templates = {'Meta_OS': 'You are the core OS brain for SigmaOS. Current mode: {mode}. Mode config: {config}. Goal: {goal}. Steps: 1. Interpret. 2. Route to Adapters. 3. Synthesize.'}