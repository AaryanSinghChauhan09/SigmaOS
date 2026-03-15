# Generated method: Spellathon._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Spellathon:
    def _init_state(self):
        self.centre = 'A'
        self.letters = set('ABCEFLO')
        self.matched = set()