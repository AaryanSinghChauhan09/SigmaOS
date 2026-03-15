"""
package_weaver.py — backward-compat shim.
Real implementation lives in package_weaver/ package.
"""

from package_weaver.PackageWeaver import *  # noqa

__all__ = ['PackageWeaver']
