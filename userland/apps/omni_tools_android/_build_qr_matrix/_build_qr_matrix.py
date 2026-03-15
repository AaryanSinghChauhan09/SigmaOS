# Generated file: _build_qr_matrix
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

def _build_qr_matrix(data: str, modules: int=21) -> list:
    """Generate a pseudo-QR boolean grid from data (deterministic hash)."""
    import hashlib
    seed = int(hashlib.sha256(data.encode()).hexdigest(), 16)
    rng = random.Random(seed)
    matrix = [[False] * modules for _ in range(modules)]

    def finder(r, c):
        for dr in range(7):
            for dc in range(7):
                edge = dr in (0, 6) or dc in (0, 6) or (2 <= dr <= 4 and 2 <= dc <= 4)
                matrix[r + dr][c + dc] = edge
    finder(0, 0)
    finder(0, modules - 7)
    finder(modules - 7, 0)
    for i in range(8, modules - 8):
        matrix[6][i] = matrix[i][6] = i % 2 == 0
    for r in range(modules):
        for c in range(modules):
            if not matrix[r][c]:
                matrix[r][c] = rng.random() > 0.5
    return matrix