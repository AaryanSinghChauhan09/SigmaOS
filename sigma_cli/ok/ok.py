# Generated file: ok
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def ok(msg):
    print(f"  {_ansi(C.GREEN, '✔')}  {msg}")