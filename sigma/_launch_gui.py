# Generated file: _launch_gui
import sys
import os
import argparse
import time
import io
from sigma_core import SigmaKernel, SigmaConfig

def _launch_gui(kernel: SigmaKernel, intent: str=None):
    try:
        from sigma_gui import launch_gui
        print(f"  Launching SigmaOS GUI Dashboard{(' with intent: ' + intent if intent else '')}…\n")
        if not launch_gui(kernel, intent=intent):
            _launch_cli(kernel, [])
    except ImportError as ie:
        print(f'  GUI import error: {ie}')
        _launch_cli(kernel, [])