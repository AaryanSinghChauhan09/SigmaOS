# Generated file: scrub_all
import os
import re
from pathlib import Path

def scrub_all():
    """Global entry point for direct module execution."""
    scrubber = SigmaScrubber()
    scrubber.run()