# SigmaOS Apex Shard: create_module_file
import os
import ast
import textwrap
import re

def create_module_file(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write('# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)\n')
        f.write('# Principle: Single Responsibility per File\n\n')
        f.write(sanitize(content))