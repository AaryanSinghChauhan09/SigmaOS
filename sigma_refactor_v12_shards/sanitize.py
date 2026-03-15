from functools import lru_cache
import os
import ast
import textwrap
import re
from ..constants import RELIGIOUS
from ..constants import PERSONAL
from ..constants import VULGAR

def sanitize(text):
    text = PERSONAL.sub('SigmaSovereign', text)
    text = RELIGIOUS.sub('Universal', text)
    text = VULGAR.sub('Substandard', text)
    return text