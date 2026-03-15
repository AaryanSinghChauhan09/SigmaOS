"""
package_weaver.py — backward-compat shim.
Real implementation lives in package_weaver/ package.
"""

from .package_weaver.PackageWeaver import *  # noqa

__all__ = ['PackageWeaver']

"""Auto-generated package __init__.py"""
from .packageweaver import *  # noqa: F401, F403
