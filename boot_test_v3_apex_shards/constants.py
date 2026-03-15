import sys, os, time
from sigma_core import SigmaKernel, SigmaConfig, EventBus, ModuleRegistry

_ROOT = os.path.abspath(os.path.dirname(__file__))
PASS = '\x1b[92m✔\x1b[0m'
FAIL = '\x1b[91m✖\x1b[0m'
INFO = '\x1b[96mℹ\x1b[0m'
HEAD = lambda s: print(f'\n\x1b[1m\x1b[95m━━━ {s} ━━━\x1b[0m')