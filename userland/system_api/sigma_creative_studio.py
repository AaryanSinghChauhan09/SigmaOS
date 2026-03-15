"""
sigma_creative_studio.py — backward-compat shim.
Real implementation lives in sigma_creative_studio/ package.
"""

from sigma_creative_studio.SigmaBlockCoder import *  # noqa
from sigma_creative_studio.SigmaLiveCodeEditor import *  # noqa
from sigma_creative_studio.SigmaIconPainter import *  # noqa
from sigma_creative_studio.SigmaSoundStudio import *  # noqa
from sigma_creative_studio.SigmaAnimationStudio import *  # noqa

__all__ = ['SigmaBlockCoder', 'SigmaLiveCodeEditor', 'SigmaIconPainter', 'SigmaSoundStudio', 'SigmaAnimationStudio']
