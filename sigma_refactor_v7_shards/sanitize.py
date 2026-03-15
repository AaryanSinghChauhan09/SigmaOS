# SigmaOS Apex Shard: sanitize
import os
import ast
import textwrap
import re

def sanitize(text):
    for p in PERSONAL:
        text = p.sub('SigmaUser', text)
    for r in RELIGIOUS:
        text = r.sub('Universal', text)
    return text