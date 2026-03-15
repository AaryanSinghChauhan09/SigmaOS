"""
spot_it.py — backward-compat shim.
Real implementation lives in spot_it/ package.
"""

from spot_it.draw_circle import *  # noqa
from spot_it.draw_square import *  # noqa
from spot_it.draw_triangle import *  # noqa
from spot_it.draw_diamond import *  # noqa
from spot_it.draw_star import *  # noqa
from spot_it.draw_cross import *  # noqa
from spot_it.draw_hexagon import *  # noqa
from spot_it.draw_arrow import *  # noqa
from spot_it.SpotItGame import *  # noqa

__all__ = ['draw_circle', 'draw_square', 'draw_triangle', 'draw_diamond', 'draw_star', 'draw_cross', 'draw_hexagon', 'draw_arrow', 'SpotItGame']
