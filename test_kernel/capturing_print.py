# Generated file: capturing_print
import sys, traceback
import builtins
from sigma_core.kernel import SigmaKernel

def capturing_print(*args, **kwargs):
    line = ' '.join((str(a) for a in args))
    captured.append(line)
    orig_print(*args, **kwargs)