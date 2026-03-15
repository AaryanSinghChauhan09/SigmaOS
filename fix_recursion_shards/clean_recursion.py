# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import os
import shutil

def clean_recursion():
    print('Detecting recursive modularization...')
    for root, dirs, files in os.walk(ROOT, topdown=False):
        parts = root.split(os.sep)
        counts = {}
        for p in parts:
            p_lower = p.lower()
            counts[p_lower] = counts.get(p_lower, 0) + 1
            if counts[p_lower] > 2:
                print(f'Recursion found: {root}')
                try:
                    for f in files:
                        if f.endswith('.py'):
                            pass
                    shutil.rmtree(root)
                except Exception as e:
                    print(f'  Failed to delete: {e}')
                break