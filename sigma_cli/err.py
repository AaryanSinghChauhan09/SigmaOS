"""
Auto-split from sigma_cli.py — err
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def err(msg):
    print(f"  {_ansi(C.RED, '✖')}  {msg}")
