"""
SigmaOS Sovereign Experience Shard
===================================
Visual interaction, shell environments, and data dashboards.
"""
from .sovereign_shell import SovereignShell
from .window_manager import SigmaWindowManager
from .data_visualizer import SigmaDataVisualizer
from .ghostchat import SigmaGhostChat

__all__ = ["SovereignShell", "SigmaWindowManager", "SigmaDataVisualizer", "SigmaGhostChat"]
