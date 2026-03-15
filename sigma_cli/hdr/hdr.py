# Generated file: hdr
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def hdr(msg):
    print(f"\n{_ansi(C.BOLD + C.MAGENTA, '━━━ ' + msg + ' ━━━')}")