# Generated method: SigmaApp.to_dict
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaApp:
    def to_dict(self) -> Dict[str, Any]:
        return {'app_id': self.app_id, 'name': self.name, 'version': self.version, 'category': self.category, 'developer': self.developer, 'description': self.description, 'size_mb': self.size_mb, 'rating': self.rating, 'downloads': self.downloads, 'verified': self.verified, 'sandbox': self.sandbox_level, 'installed': self.installed}