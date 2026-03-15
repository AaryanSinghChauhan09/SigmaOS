"""
logic_simulator.py — backward-compat shim.
Real implementation lives in logic_simulator/ package.
"""

from logic_simulator.full_adder import *  # noqa
from logic_simulator.half_adder import *  # noqa
from logic_simulator.LogicSimulator import *  # noqa
from logic_simulator.launch import *  # noqa

__all__ = ['full_adder', 'half_adder', 'LogicSimulator', 'launch']
