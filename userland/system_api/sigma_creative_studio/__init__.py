"""
sigma_creative_studio.py — backward-compat shim.
Real implementation lives in sigma_creative_studio/ package.
"""

from .sigma_creative_studio.SigmaBlockCoder import *  # noqa
from .sigma_creative_studio.SigmaLiveCodeEditor import *  # noqa
from .sigma_creative_studio.SigmaIconPainter import *  # noqa
from .sigma_creative_studio.SigmaSoundStudio import *  # noqa
from .sigma_creative_studio.SigmaAnimationStudio import *  # noqa

__all__ = ['SigmaBlockCoder', 'SigmaLiveCodeEditor', 'SigmaIconPainter', 'SigmaSoundStudio', 'SigmaAnimationStudio']

"""Auto-generated package __init__.py"""
from .sigmablockcoder import *  # noqa: F401, F403
from .sigmalivecodeeditor import *  # noqa: F401, F403
from .sigmaiconpainter import *  # noqa: F401, F403
from .sigmasoundstudio import *  # noqa: F401, F403
from .sigmaanimationstudio import *  # noqa: F401, F403
