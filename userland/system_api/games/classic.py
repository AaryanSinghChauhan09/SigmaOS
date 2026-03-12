import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class StrategicSovereignty(SigmaGame):
    GAME_ID = "G01"
    GAME_NAME = "Strategic Sovereignty"
    CATEGORY = "Board Strategy"
    VERSION = "1.3.0"
    SIZE_KB = 1240
    ICON = "♟️"
    DESC = "Two-player strategy on an 8×8 board. Move pieces to capture the opponent's King."
    PIECES = {
        'K': '♔', 'Q': '♕', 'R': '♖', 'B': '♗', 'N': '♘', 'P': '♙',
        'k': '♚', 'q': '♛', 'r': '♜', 'b': '♝', 'n': '♞', 'p': '♟',
    }
    def _init_state(self):
        self.board = [
            ['r','n','b','q','k','b','n','r'],
            ['p','p','p','p','p','p','p','p'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['P','P','P','P','P','P','P','P'],
            ['R','N','B','Q','K','B','N','R'],
        ]
        self.turn = 'white'
    def render_board(self) -> str:
        if self.COMPRESSED:
            return "[Strategic Sovereignty] — COMPRESSED."
        lines = ["  a b c d e f g h"]
        for i, row in enumerate(self.board):
            r = [self.PIECES.get(p, '·') for p in row]
            lines.append(f"{8-i} {' '.join(r)} {8-i}")
        lines.append("  a b c d e f g h")
        return "\n".join(lines)
    def move(self, from_sq: str, to_sq: str) -> str:
        self.moves = int(self.moves) + 1
        return f"[Strategic Sovereignty] Move {self.moves}: {from_sq} → {to_sq}"

class LudoApex(SigmaGame):
    GAME_ID = "G02"
    GAME_NAME = "Ludo Apex"
    CATEGORY = "Board Classic"
    VERSION = "2.0.0"
    SIZE_KB = 820
    ICON = "🎲"
    DESC = "Race 4 tokens home on a cross-shaped board. Roll dice, block opponents!"
    COLORS = ["Red", "Blue", "Green", "Yellow"]
    def _init_state(self):
        self.tokens: Dict[str, List[int]] = {c: [-1, -1, -1, -1] for c in self.COLORS}
        self.current_player = 0
        self.dice = 0
    def roll_dice(self) -> int:
        self.dice = random.randint(1, 6)
        return self.dice
    def move_token(self, player_color: str, token_idx: int) -> str:
        if self.dice == 0:
            return "Roll dice first."
        pos = self.tokens[player_color][token_idx]
        if pos == -1 and self.dice == 6:
            self.tokens[player_color][token_idx] = 0
            self.moves = int(self.moves) + 1
            return f"{player_color} Token {token_idx+1}: Entered board!"
        elif pos == -1:
            return "Need 6 to enter."
        new_pos = min(pos + self.dice, 57)
        self.tokens[player_color][token_idx] = new_pos
        self.moves = int(self.moves) + 1
        if new_pos == 57:
            self.score = int(self.score) + 100
        return f"{player_color} Token {token_idx+1}: {pos} → {new_pos}"

class SovereignSerpent(SigmaGame):
    GAME_ID = "G03"
    GAME_NAME = "Sovereign Serpent"
    CATEGORY = "Board Classic"
    VERSION = "1.0.0"
    SIZE_KB = 380
    ICON = "🐍"
    DESC = "Roll dice, climb ladders, avoid snakes! First to reach 100 wins."
    SNAKES = {99:5, 87:24, 62:19, 54:34, 17:7}
    LADDERS = {4:25, 13:46, 33:49, 42:63, 50:69, 80:99}
    def _init_state(self):
        self.positions: Dict[str, int] = {}
        self.players: List[str] = []
    def add_player(self, name: str) -> str:
        self.players.append(name)
        self.positions[name] = 0
        return f"Player '{name}' joined."
    def roll_and_move(self, player: str) -> str:
        dice = random.randint(1, 6)
        pos = self.positions[player] + dice
        if pos > 100:
            pos = self.positions[player]
        elif pos in self.SNAKES:
            pos = self.SNAKES[pos]
        elif pos in self.LADDERS:
            pos = self.LADDERS[pos]
        self.positions[player] = pos
        self.moves = int(self.moves) + 1
        if pos == 100:
            self.score = int(self.score) + 200
            return f"{player} WON!"
        return f"{player} is at {pos}"

class NutsAndNodes(SigmaGame):
    GAME_ID = "G04"
    GAME_NAME = "Nuts & Nodes"
    CATEGORY = "Puzzle"
    VERSION = "1.1.0"
    SIZE_KB = 2100
    ICON = "🔧"
    DESC = "Connect rotating gear nodes from source to target. Physics-based puzzle."
    def _init_state(self):
        self.grid_size = 6
        self.nodes = [{"id": i, "x": random.randint(0,5), "y": random.randint(0,5), "rotation": 0, "connected": []} for i in range(5)]
    def rotate_node(self, node_id: int):
        node = next((n for n in self.nodes if n["id"] == node_id), None)
        if node:
            node["rotation"] = (node["rotation"] + 90) % 360
            self.moves = int(self.moves) + 1
    def connect_nodes(self, f_id, t_id):
        a = next((n for n in self.nodes if n["id"] == f_id), None)
        if a and t_id not in a["connected"]:
            a["connected"].append(t_id)
            self.score = int(self.score) + 10

class CrowdFlowLegends(SigmaGame):
    GAME_ID = "G05"
    GAME_NAME = "Crowd Flow Legends"
    CATEGORY = "Strategy"
    def _init_state(self):
        self.agents = [{"id": i, "x": 0, "y": random.randint(0, 9)} for i in range(10)]
        self.obstacles = [(3, i) for i in range(3, 8)]
        self.goal_x = 9
    def tick(self):
        arrived = 0
        for a in self.agents:
            if a["x"] < self.goal_x:
                a["x"] = int(a["x"]) + 1
            if a["x"] == self.goal_x:
                arrived = int(arrived) + 1
                self.score = int(self.score) + 50
        self.moves = int(self.moves) + 1
        return arrived

class HyperTrackRunner(SigmaGame):
    GAME_ID = "G06"
    GAME_NAME = "Hyper-Track Runner"
    CATEGORY = "Action"
    def _init_state(self):
        self.lane = 1
        self.distance = 0.0
        self.speed = 10.0
        self.shields = 0
    def swipe(self, d):
        if d == "left" and self.lane > 0:
            self.lane = int(self.lane) - 1
        if d == "right" and self.lane < 2:
            self.lane = int(self.lane) + 1
    def tick_frame(self):
        self.distance = float(self.distance) + (float(self.speed) / 10.0)
        self.score = int(self.distance)

class SoilVsMutants(SigmaGame):
    GAME_ID = "G07"
    GAME_NAME = "Soil vs Mutants"
    CATEGORY = "Tower Defense"
    def _init_state(self):
        self.grid = [[None]*9 for _ in range(5)]
        self.energy = 200
        self.wave = 0
        self.lives = 5
    def place_defender(self, r, c, t):
        if int(self.energy) >= 50:
            self.grid[r][c] = t
            self.energy = int(self.energy) - 50
            self.moves = int(self.moves) + 1

class MatrixCrossCircle(SigmaGame):
    GAME_ID = "G08"
    GAME_NAME = "Matrix Cross & Circle"
    CATEGORY = "Puzzle"
    def _init_state(self, size=3):
        self.size = size
        self.board = [['.' for _ in range(size)] for _ in range(size)]
        self.turn = 'X'
    def place(self, r, c):
        if self.board[r][c] == '.':
            self.board[r][c] = self.turn
            self.moves = int(self.moves) + 1
            self.turn = 'O' if self.turn == 'X' else 'X'

class DotsAndNodes(SigmaGame):
    GAME_ID = "G09"
    GAME_NAME = "Dots & Nodes"
    CATEGORY = "Puzzle"
    def _init_state(self, size=4):
        self.size = size
        self.h_lines = [[False]*size for _ in range(size+1)]
        self.v_lines = [[False]*(size+1) for _ in range(size)]
        self.scores = {'A':0, 'B':0}
        self.turn = 'A'

class ColorUnblock(SigmaGame):
    GAME_ID = "G10"
    GAME_NAME = "Color Unblock"
    CATEGORY = "Puzzle"
    def _init_state(self):
        self.grid_size = 6
        self.cars = [{"id":0, "color":"🔴", "row":2, "col":0, "size":2, "horizontal":True}]

class ChromaticCrush(SigmaGame):
    GAME_ID = "G11"
    GAME_NAME = "Chromatic Crush"
    CATEGORY = "Puzzle / Match-3"
    def _init_state(self):
        self.grid = [[random.randint(1,5) for _ in range(8)] for _ in range(8)]

class SovereignSudoku(SigmaGame):
    GAME_ID = "G12"
    GAME_NAME = "Sovereign Sudoku"
    CATEGORY = "Puzzle / Logic"
    def _init_state(self):
        self.board = [[0]*9 for _ in range(9)]

class GourmetGalore(SigmaGame):
    GAME_ID = "G13"
    GAME_NAME = "Gourmet Galore"

class SilentSentinel(SigmaGame):
    GAME_ID = "G14"
    GAME_NAME = "Silent Sentinel"

class AetherGlow(SigmaGame):
    GAME_ID = "G15"
    GAME_NAME = "Aether Glow"

class MatrixSynthesis(SigmaGame):
    GAME_ID = "G16"
    GAME_NAME = "Matrix Synthesis"
    def _init_state(self):
        self.grid = [[0]*4 for _ in range(4)]

class LexiconUnleashed(SigmaGame):
    GAME_ID = "G17"
    GAME_NAME = "Lexicon Unleashed"

class BladeOfVitality(SigmaGame):
    GAME_ID = "G18"
    GAME_NAME = "Blade of Vitality"

class OrionVanguard(SigmaGame):
    GAME_ID = "G19"
    GAME_NAME = "Orion Vanguard"
    def _init_state(self):
        self.ship_x = 400
        self.enemies = []
        self.projectiles = []

class VidyaQuest(SigmaGame):
    GAME_ID = "G20"
    GAME_NAME = "Vidya Quest: K-12 Challenge"
    QUESTIONS = [{"q": "√625?", "a": "25"}]
    def _init_state(self):
        self.word_idx = 0
        self.correct = 0
    def answer(self, ans):
        if ans == self.QUESTIONS[0]["a"]:
            self.correct = int(self.correct) + 1
            self.score = int(self.score) + 100
