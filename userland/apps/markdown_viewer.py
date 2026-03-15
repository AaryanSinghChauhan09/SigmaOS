"""
markdown_viewer.py — backward-compat shim.
Real implementation lives in markdown_viewer/ package.
"""

from markdown_viewer.MarkdownViewer import *  # noqa
from markdown_viewer.launch import *  # noqa

__all__ = ['MarkdownViewer', 'launch']
