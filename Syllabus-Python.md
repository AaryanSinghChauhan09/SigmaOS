# Python Programming → SigmaPy Runtime

> Maps the Python Programming syllabus to `SigmaPy` — the highly isolated, sovereign Python runtime embedded inside SigmaOS.

---

## Overview & Core Concepts

SigmaPy embeds CPython 3.12 directly into the Ring-3 userland lattice, providing an elegant scripting layer for:

- `sigma-cli` shell automation scripts

- `SigmaSheets` macro calculation engine

- `SigmaAI` Machine Learning pipelines E.g., PyTorch, TensorFlow, Scikit-learn

- System telemetry automation (`sigma-automate`)

**Unique Selling Point (USP):** Turning raw data into actionable insights with unmatched developer productivity, readability, and rich data science ecosystem integration (NumPy, Pandas, Matplotlib).

---

## Unit I: Introduction to Python & Basic Syntax

```python

# History: Created by Guido van Rossum, 1991

# Features: interpreted, dynamically typed, garbage-collected,

#           readable syntax, huge stdlib, cross-platform

# Basic Syntax

print("SigmaOS Zenith v15.2")       # Output statement

# This is a comment                 # Single-line comment

"""
Multi-line
docstring / comment
"""

# Indentation is MANDATORY (4 spaces standard)

if True:
    print("Indented block executed successfully")

# Running Python in SigmaOS

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

print(type(x))             # <class 'int'>

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
print(a + b, a - b, a * b, a / b)    # 13, 7, 30, 3.3333...

print(a // b, a % b, a ** b)         # 3, 1, 1000  (floor, mod, power)

# Bitwise Operators

print(0xFF & 0x0F)   # 15

print(1 << 4)        # 16

# Comparison & Logical

print(a > b and b > 0)    # True

print(a == 10 or b == 0)  # True

print(not (a < 5))        # True

# Control Structures

role = "admin"
if role == "admin":
    print("Full administrative access granted")
elif role == "user":
    print("Limited userland access granted")
else:
    print("Access denied")

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

# while Loop

count = 0
while count < 5:
    print(count)
    count += 1

# Loop control statements

for n in range(10):
    if n == 3: continue     # skip 3

    if n == 7: break        # stop at 7

    print(n)

# pass: placeholder statement (no operation)

def placeholder_fn():
    pass
```

---

## Unit IV: Functions, Modules & Data Science Tools

```python

# Built-in functions

print(abs(-5))            # 5

print(max(1, 5, 3))       # 5

print(min([4, 2, 7]))     # 2

print(len("SigmaOS"))     # 7

print(round(3.14159, 2))  # 3.14

print(sorted([3,1,2]))    # [1, 2, 3]

print(sum([1,2,3,4,5]))   # 15

print(list(enumerate(["a","b"])))  # [(0,'a'), (1,'b')]

print(list(zip([1,2], ["a","b"]))) # [(1,'a'), (2,'b')]

# User-defined functions

def factorial(n: int) -> int:
    """Calculate n! recursively."""
    if n <= 1: return 1
    return n * factorial(n - 1)

def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

# *args and **kwargs

def log(*args, level="INFO", **kwargs):
    print(f"[{level}]", *args, kwargs)

# Lambda functions

square = lambda x: x ** 2
add = lambda a, b: a + b

# Modules & Data Science Tools

import os, sys, math, json
from datetime import datetime

# Core Data Science Toolkits (SigmaPy Native Bridging)

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.ensemble import RandomForestClassifier

# SigmaOS built-in kernel modules

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
print(s.replace("OS", "OS v15.2"))
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

## Debugging & Problem-Solving in Python

### Common Issues & Fix Strategies

- **Issue - Memory Leaks in Long-Running Data Pipelines:** Unreferenced Pandas DataFrames or large NumPy arrays remain trapped in memory due to cyclic garbage collection delays.
- *Fix Strategy:* Use explicit `del df` statements, invoke `gc.collect()` manually after heavy batch transformations, and utilize memory-mapped files (`np.memmap`) for out-of-core processing.

- **Issue - Algorithmic Complexity in Nested Loops:** Using naive `for` loops to search or cross-reference large lists yields $O(N^2)$ complexity.
- *Fix Strategy:* Convert lists to Python sets or dictionaries (`O(1)` hash lookup) or vectorize operations entirely using NumPy broadcasting ($O(N)$ C-speed execution).

- **Issue - Thread Deadlocks & GIL Contention:** Multi-threaded CPython scripts stall due to Global Interpreter Lock (GIL) contention or circular lock acquisition.
- *Fix Strategy:* Migrate from `threading` to `multiprocessing` or `concurrent.futures.ProcessPoolExecutor` to bypass the GIL entirely and distribute workloads across physical CPU cores.

- **Issue - Silent Data Truncation & Missing Values:** Unsanitized CSV ingestion injects `NaN` values into mathematical tensors.
- *Fix Strategy:* Integrate robust unit testing (`pytest`) and data validation pipelines (`pydantic` / `great_expectations`) to enforce strict schema contracts before execution.

---

## SigmaPy Integration Points

| Python Feature | SigmaOS Usage |
| :--- | :--- |
| `subprocess` | Spawn sigma shard processes |
| `socket` | SovereignNetStack Python API |
| `os.path` | SovereignFS path helpers |
| `json` | Config file parsing |
| `threading` | Parallel automation scripts |
| `asyncio` | Async SigmaDB queries |
| `numpy` / `pandas` | SigmaAI data pipelines |
| `matplotlib` | SigmaViz chart rendering |
| `sklearn` | SigmaAI ML training |

### Last updated: 2026-05-19 | SigmaOS Zenith v15.2
