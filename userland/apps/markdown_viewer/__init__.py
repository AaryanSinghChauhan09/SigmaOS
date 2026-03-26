"""
markdown_viewer.py — backward-compat shim.
Real implementation lives in markdown_viewer/ package.
"""

from .markdown_viewer.MarkdownViewer import *  # noqa
from .markdown_viewer.launch import *  # noqa

__all__ = ['MarkdownViewer', 'launch']

"""Auto-generated package __init__.py"""
from .markdownviewer import *  # noqa: F401, F403
from .launch import *  # noqa: F401, F403
