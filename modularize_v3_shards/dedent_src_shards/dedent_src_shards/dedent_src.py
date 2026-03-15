# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def dedent_src(src: str) -> str:
    return textwrap.dedent(src)