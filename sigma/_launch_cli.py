# Generated file: _launch_cli
import sys
import os
import argparse
import time
import io
from sigma_core import SigmaKernel, SigmaConfig

def _launch_cli(kernel: SigmaKernel, extra_args: list, json_mode: bool=False, silent: bool=False):
    try:
        old_argv = sys.argv
        cli_argv = ['sigma_cli']
        if json_mode:
            cli_argv.append('--json')
        if silent:
            cli_argv.append('--silent')
        cleaned_extra = [a for a in extra_args if a not in ('--json', '--silent')]
        cli_argv.extend(cleaned_extra)
        sys.argv = cli_argv
        from sigma_cli import main as cli_main
        cli_main()
        sys.argv = old_argv
    except ImportError as ie:
        print(f'  CLI import error: {ie}')