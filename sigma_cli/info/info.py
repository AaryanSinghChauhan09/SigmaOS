# Generated file: info
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def info(msg):
    print(f"  {_ansi(C.CYAN, 'ℹ')}  {msg}")