# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def node_header_comment(filename: str, name: str) -> str:
    return f'"""\nAuto-split from {filename} — {name}\n"""\n\n'