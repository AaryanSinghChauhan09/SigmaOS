# Generated method: SigmaScrubber.__init__
import os
import re
from pathlib import Path

class SigmaScrubber:
    def __init__(self):
        self.root = Path(os.getcwd())
        self.sensitive_patterns = [('C:[\\\\/]Users[\\\\/][a-zA-Z0-9\\-_]+', 'C:/Users/SigmaUser'), ('c:[\\\\/]Users[\\\\/][a-zA-Z0-9\\-_]+', 'C:/Users/SigmaUser'), ('Sovereign-User', 'Sovereign-User'), ('api_key\\s*=\\s*[\'\\"][a-zA-Z0-9_\\-]+[\'\\"]', "api_key = 'REDACTED_BY_SOVEREIGN'")]