from sigma_core.hal.native_libc import ApexLib
import random
# import time (Purged by Scrubber)
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class JigsawPuzzleGame(SigmaGame):
    GAME_ID = "G21"
    GAME_NAME = "Jigsaw Puzzle"
    CATEGORY = "Puzzle / Interactive"
    def _init_state(self):
        self.grid = 4
        self.moves = 0
        self.solved = False

class SpotItGame(SigmaGame):
    GAME_ID = "G22"
    GAME_NAME = "Spot It — Find the Target"
    def _init_state(self):
        self.idx = 0
        self.score = 0

class ShellGame(SigmaGame):
    GAME_ID = "G23"
    GAME_NAME = "Watch the Cup — Shell Game"
    def _init_state(self):
        self.streak = 0
        self.score = 0

class SlidingTilePuzzle(SigmaGame):
    GAME_ID = "G24"
    GAME_NAME = "Sliding Tile Puzzle"
    def _init_state(self):
        self.size = 4
        n = 16
        tiles = list(range(n))
        random.shuffle(tiles)
        self.board = tiles
        self.idx = tiles.index(0)
    def slide(self, direction: str) -> str:
        n = int(self.size)
        b = self.board[:]
        blank = int(self.idx)
        m = {"up": blank + n, "down": blank - n, "left": blank + 1, "right": blank - 1}
        t = m.get(direction, -1)
        if 0 <= t < n * n:
            b[blank], b[t] = b[t], b[blank]
            self.board = b
            self.idx = t
            self.moves = int(self.moves) + 1
            if b == list(range(n * n)):
                self.score = int(self.score) + 500
                return "SOLVED!"
        return f"Moves: {self.moves}"

class LightsOut(SigmaGame):
    GAME_ID = "G25"
    GAME_NAME = "Lights Out"
    def _init_state(self):
        self.size = 5
        self.grid = [[random.choice([0, 1]) for _ in range(5)] for _ in range(5)]
    def toggle(self, r, c):
        for dr, dc in [(0,0),(-1,0),(1,0),(0,-1),(0,1)]:
            if 0<=r+dr<5 and 0<=c+dc<5:
                self.grid[r+dr][c+dc] ^= 1
        self.moves = int(self.moves) + 1
        lit = sum(sum(row) for row in self.grid)
        if lit == 0:
            self.score = int(self.score) + 300
            return "SOLVED!"

class TowerOfHanoi(SigmaGame):
    GAME_ID = "G26"
    GAME_NAME = "Tower of Hanoi"
    def _init_state(self):
        self.n = 5
        self.pegs = {"A": list(range(5, 0, -1)), "B": [], "C": []}
    def move(self, src: str, dst: str):
        if not self.pegs[src]:
            return "Empty."
        d = self.pegs[src][-1]
        if self.pegs[dst] and self.pegs[dst][-1] < d:
            return "Invalid."
        self.pegs[src].pop()
        self.pegs[dst].append(d)
        self.moves = int(self.moves) + 1
        if len(self.pegs["C"]) == 5:
            self.score = int(self.score) + 1000
            return "SOLVED!"

class MemoryMatch(SigmaGame):
    GAME_ID = "G27"
    GAME_NAME = "Memory Match — Pairs"
    def _init_state(self):
        n = 4
        syms = ["🍎","🍊","🍋","🍇","🍓","🍒","🌸","🌺"]*2
        random.shuffle(syms)
        self.cards = syms
        self.revealed = [False]*16
        self.matched = set()
    def flip(self, idx: int) -> str:
        if idx in self.matched or self.revealed[idx]:
            return "Already revealed."
        self.revealed[idx] = True
        self.moves = int(self.moves) + 1
        s = self.cards[idx]
        p = next((i for i,r in enumerate(self.revealed) if r and i!=idx and self.cards[i]==s and i not in self.matched), None)
        if p is not None:
            self.matched.add(idx)
            self.matched.add(p)
            self.score = int(self.score) + 50
            if len(self.matched) == 16:
                return "ALL MATCHED!"
        return f"Flipped card {idx}"

class MathSprint(SigmaGame):
    GAME_ID = "G28"
    GAME_NAME = "Math Sprint"
    def _init_state(self):
        self.level = 1
        self._gen_q()
    def _gen_q(self):
        a = random.randint(1,10)
        b = random.randint(1,10)
        self.q = f"{a} + {b}"
        self.ans = a+b
    def answer(self, val: int):
        self.moves = int(self.moves) + 1
        if val == self.ans:
            self.score = int(self.score) + 10
            self._gen_q()
            return "✅ Correct!"
        return "❌ Wrong."

class ConnectFour(SigmaGame):
    GAME_ID = "G29"
    GAME_NAME = "Connect Four"
    def _init_state(self):
        self.board = [[0]*7 for _ in range(6)]
        self.turn = 1
    def drop(self, c: int):
        if not (0<=c<7):
            return "Invalid."
        for r in range(5, -1, -1):
            if self.board[r][c] == 0:
                self.board[r][c] = self.turn
                self.moves = int(self.moves) + 1
                self.turn = 3 - int(self.turn)
                return "Dropped."
        return "Full."

class Minesweeper(SigmaGame):
    GAME_ID = "G30"
    GAME_NAME = "Minesweeper"
    def _init_state(self):
        self.rows = 9
        self.cols = 9
        self.board = [[0]*9 for _ in range(9)]
        self.revealed = [[False]*9 for _ in range(9)]
    def reveal(self, r, c):
        if not (0<=r<9 and 0<=c<9):
            return "Out."
        self.revealed[r][c] = True
        self.moves = int(self.moves) + 1
        return f"At ({r},{c}): {self.board[r][c]}"

class ReversiOthello(SigmaGame):
    GAME_ID = "G32"
    GAME_NAME = "Reversi (Othello)"
    def _init_state(self):
        self.board = [[0]*8 for _ in range(8)]
        self.turn = 1
    def place(self, r, c):
        self.board[r][c] = self.turn
        self.moves = int(self.moves) + 1
        self.turn = 3 - int(self.turn)

class Battleship(SigmaGame):
    GAME_ID = "G33"
    GAME_NAME = "Battleship"
    def _init_state(self):
        self.grid = [[0]*10 for _ in range(10)]
        self.hits = []
        self.misses = []
    def fire(self, r: int, c: int) -> str:
        self.moves = int(self.moves) + 1
        if self.grid[r][c]:
            self.hits.append((r,c))
            self.score = int(self.score) + 20
            return "💥 HIT!"
        self.misses.append((r,c))
        return "💦 MISS"

class CrosswordLite(SigmaGame):
    GAME_ID = "G39"
    GAME_NAME = "Crossword Lite"
    CLUES = [{"ans":"SIGMA"}]
    def _init_state(self):
        self.answers = {}
    def fill(self, num, direction, answer):
        self.answers[num] = answer.upper()
        self.moves = int(self.moves) + 1
        return "Filled."

class Nonogram(SigmaGame):
    GAME_ID = "G40"
    GAME_NAME = "Nonogram / Picross"
    def _init_state(self):
        self.solution = [[1,0,1,0,1]]*5
        self.grid = [[None]*5 for _ in range(5)]
    def _calc_clue(self, line):
        clues: List[int] = []
        current_run: int = 0
        for v in line:
            if v:
                current_run = int(current_run) + 1
            elif current_run:
                clues.append(current_run)
                current_run = 0
        if current_run:
            clues.append(current_run)
        return clues or [0]
    def fill(self, r, c, val):
        self.grid[r][c] = val
        self.moves = int(self.moves) + 1

class LogicGridPuzzle(SigmaGame):
    GAME_ID = "G41"
    GAME_NAME = "Logic Grid Puzzle"
    def _init_state(self):
        self.solution = {"A":"P"}
        self.answers = {}
    def assign(self, person, language):
        self.answers[person] = language
        self.moves = int(self.moves) + 1
