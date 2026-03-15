"""
antigravity_core.py — backward-compat shim.
Real implementation lives in antigravity_core/ package.
"""

from antigravity_core.AntigravityForensicCore import *  # noqa
from antigravity_core.JurisprudenceEngine import *  # noqa
from antigravity_core.AntigravityLayer import *  # noqa
from antigravity_core.AntigravityGhostMode import *  # noqa
from antigravity_core.AntigravityDeveloperTools import *  # noqa
from antigravity_core.AntigravityDataScience import *  # noqa
from antigravity_core.AntigravityMachineLearning import *  # noqa

__all__ = ['AntigravityForensicCore', 'JurisprudenceEngine', 'AntigravityLayer', 'AntigravityGhostMode', 'AntigravityDeveloperTools', 'AntigravityDataScience', 'AntigravityMachineLearning']
