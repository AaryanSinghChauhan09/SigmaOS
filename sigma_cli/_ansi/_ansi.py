# Generated file: _ansi
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def _ansi(code, text):
    return f'{code}{text}{C.RESET}'