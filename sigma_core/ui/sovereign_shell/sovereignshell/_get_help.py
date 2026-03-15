# Generated method: SovereignShell._get_help
import sys
import os
import time
from typing import List, Optional, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .interfaces import SigmaModuleBase, ISigmaService

class SovereignShell:
    def _get_help(self) -> str:
        return '\nSovereign Shell Help\n--------------------\nsystem [cmd]       - Hardware & Kernel telemetry\nswarm [cmd] [args] - Deploy or manage AI swarms\nfs [cmd] [args]    - SigmaFS temporal operations\nvibe [preset]      - Shift Morphological Aesthetic\nauto [cmd] [args]  - OmniAutomator / Proactive Sentinel\nturbo              - Engage Max Throughput mode\nclear              - Clear the terminal screen\nhelp               - This help menu\nexit               - Terminate session\n        '