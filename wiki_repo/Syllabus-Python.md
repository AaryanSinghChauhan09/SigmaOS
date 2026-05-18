# Python Programming → SigmaPy Runtime

> Maps the Python Programming syllabus to `SigmaPy` — the embedded Python runtime inside SigmaOS.

---

## Overview

SigmaPy embeds CPython 3.12 as the scripting engine for:
- `sigma-cli` shell scripts
- `SigmaSheets` macro engine
- `SigmaAI` ML pipelines
- System automation (`sigma-automate`)

---

## Unit I: Introduction to Python

```python
# History: Created by Guido van Rossum, 1991
# Features: interpreted, dynamically typed, garbage-collected,
#            readable syntax, huge stdlib, cross-platform

# Basic Syntax
print("SigmaOS Zenith v15.1")      # Output
# This is a comment                 # Single-line comment
"""
Multi-line
docstring / comment
"""

# Indentation is MANDATORY (4 spaces standard)
if True:
    print("Indented block")

# Running Python in SigmaOS:
# sigma run script.py
# sigma-py interactive shell: sigma-py --repl
```

---

## Unit II: Data Types, Variables & I/O

```python
# Data Types
x = 42           # int
y = 3.14         # float
z = 2 + 3j       # complex
s = "SigmaOS"    # str
b = True         # bool
n = None         # NoneType

# Type checking
print(type(x))     # <class 'int'>
print(isinstance(x, int))  # True

# Input / Output
name = input("Enter your name: ")
print(f"Hello, {name}!")
print("Value:", x, "Type:", type(x))

# Formatted output
print(f"Pi = {y:.4f}")          # 3.1400
print("Hex: {:X}".format(255))  # FF
print(f"Binary: {42:08b}")      # 00101010

# Type Conversion (Casting)
int_val = int("42")          # str → int
float_val = float(42)        # int → float
str_val = str(3.14)          # float → str
bool_val = bool(0)           # int → bool (0 = False)
list_val = list((1, 2, 3))   # tuple → list
```

---

## Unit III: Control Structures & Loops

```python
# Operators
a, b = 10, 3
print(a + b, a - b, a * b, a / b)   # 13, 7, 30, 3.333
print(a // b, a % b, a ** b)         # 3, 1, 1000  (floor, mod, power)

# Bitwise
print(0xFF & 0x0F)   # 15
print(1 << 4)        # 16

# Comparison & Logical
print(a > b and b > 0)   # True
print(a == 10 or b == 0) # True
print(not (a < 5))       # True

# Control Structures
role = "admin"
if role == "admin":
    print("Full access")
elif role == "user":
    print("Limited access")
else:
    print("No access")

# Nested if
x = 85
if x >= 0:
    if x >= 90:
        grade = "A+"
    elif x >= 80:
        grade = "A"
    else:
        grade = "B"

# Loops
for i in range(1, 6):       # 1 to 5
    print(i, end=" ")

for char in "SigmaOS":
    print(char)

# while
count = 0
while count < 5:
    print(count)
    count += 1

# Loop control
for n in range(10):
    if n == 3: continue     # skip 3
    if n == 7: break        # stop at 7
    print(n)

# pass: placeholder (no operation)
def placeholder_fn():
    pass
```

---

## Unit IV: Functions & Modules

```python
# Built-in functions
print(abs(-5))            # 5
print(max(1, 5, 3))       # 5
print(min([4, 2, 7]))     # 2
print(len("SigmaOS"))     # 7
print(round(3.14159, 2))  # 3.14
print(sorted([3,1,2]))    # [1, 2, 3]
print(sum([1,2,3,4,5]))   # 15
print(enumerate(["a","b"]))  # (0,'a'), (1,'b')
print(zip([1,2], ["a","b"])) # (1,'a'), (2,'b')
print(map(str, [1,2,3]))     # ['1','2','3']
print(filter(lambda x: x>2, [1,2,3,4]))  # [3, 4]

# User-defined functions
def factorial(n: int) -> int:
    """Calculate n! recursively."""
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

# *args and **kwargs
def log(*args, level="INFO", **kwargs):
    print(f"[{level}]", *args, kwargs)

# Lambda functions
square = lambda x: x ** 2
add = lambda a, b: a + b
sorter = sorted(items, key=lambda x: x['priority'])

# Modules
import os
import sys
import math
from datetime import datetime
import json

# Custom module (sigma_utils.py)
# import sigma_utils
# sigma_utils.format_bytes(1024)

# SigmaOS built-in modules
import sigma            # Core OS API
sigma.proc.list()       # List running processes
sigma.fs.read('/sigma/log/latest.log')
sigma.net.ping('8.8.8.8')
```

---

## Unit V: Strings, Collections & Comprehensions

```python
# Strings
s = "SigmaOS Zenith"
print(s.upper(), s.lower(), s.title())
print(s.replace("OS", "OS v15"))
print(s.split(" "))          # ['SigmaOS', 'Zenith']
print(s.strip())             # trim whitespace
print(s.startswith("Sigma")) # True
print(s[0:5])                # 'Sigma' (slicing)
print(s[::-1])               # reverse string
print(f"Length: {len(s)}")

# Lists
procs = ["init", "sigma-ui", "sigma-net"]
procs.append("sigma-ai")
procs.insert(0, "kernel")
procs.remove("sigma-net")
procs.sort()
first = procs[0]
last  = procs[-1]
sub   = procs[1:3]   # slice

# Tuples (immutable)
coords = (1920, 1080)
width, height = coords    # unpacking
# coords[0] = 4K  → TypeError (immutable)

# Sets (unique, unordered)
drivers = {"NVMe", "USB", "Audio", "NVMe"}  # NVMe appears once
drivers.add("GPU")
drivers.discard("USB")
print("NVMe" in drivers)   # True

# Dictionaries
process = {
    "pid":    42,
    "name":   "sigma-ui",
    "state":  "running",
    "cpu":    0.4
}
print(process["name"])
process["memory"] = "128MB"
for key, val in process.items():
    print(f"  {key}: {val}")

# Comprehensions
squares   = [x**2 for x in range(10)]                    # List
even_sq   = [x**2 for x in range(10) if x % 2 == 0]     # With filter
sq_dict   = {x: x**2 for x in range(5)}                  # Dict
unique_sq = {x**2 for x in range(-5, 6)}                  # Set
gen       = (x**2 for x in range(1000000))               # Generator (memory-efficient)
```

---

## SigmaPy Integration Points

| Python Feature | SigmaOS Usage |
|---|---|
| `subprocess` | Spawn sigma shard processes |
| `socket` | SovereignNetStack Python API |
| `os.path` | SovereignFS path helpers |
| `json` | Config file parsing |
| `threading` | Parallel automation scripts |
| `asyncio` | Async SigmaDB queries |
| `numpy` / `pandas` | SigmaAI data pipelines |
| `matplotlib` | SigmaViz chart rendering |
| `sklearn` | SigmaAI ML training |

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
