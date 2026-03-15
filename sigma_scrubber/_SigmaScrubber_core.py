# Generated class core: SigmaScrubber
import os
import re
from pathlib import Path

class SigmaScrubber:
    """
    Forensic Identity Scrubbing Engine for SigmaOS.
    Ensures no personal paths (C:/Users/SigmaUser) or dev-keys leak to GitHub.
    """