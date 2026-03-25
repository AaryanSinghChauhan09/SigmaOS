import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class SnakeGame(SigmaGame):
    GAME_ID = "G31"
    GAME_NAME = "Sovereign Snake"
    CATEGORY = "Arcade / Classic"
    def _init_state(self):
        self.W = 20
        self.H = 20
        self.snake = [(10,10),(10,9),(10,8)]
        self.dir = (0,1)
        self.food = (random.randint(0,19), random.randint(0,19))
        self.alive = True
    def tick(self):
        if not self.alive:
            return "Game over."
        h = self.snake[0]
        nr, nc = h[0] + self.dir[0], h[1] + self.dir[1]
        if not (0 <= nr < 20 and 0 <= nc < 20) or (nr, nc) in self.snake:
            self.alive = False
            return "💀 GAME OVER!"
        self.snake.insert(0, (nr, nc))
        self.moves = int(self.moves) + 1
        if (nr, nc) == self.food:
            self.score = int(self.score) + 10
            self.food = (random.randint(0, 19), random.randint(0, 19))
        else:
            self.snake.pop()

class MazeChasePacStyle(SigmaGame):
    GAME_ID = "G42"
    GAME_NAME = "Sigma Maze Chase"
    def _init_state(self):
        self.maze = ["#########", "#...#...#", "#########"]
        self.pos = (1, 1)
        self.ghosts = [(1, 7)]
    def move(self, d):
        dr, dc = {"up": (-1, 0), "down": (1, 0), "left": (0, -1), "right": (0, 1)}.get(d, (0, 0))
        h, w = len(self.maze), len(self.maze[0])
        nr, nc = self.pos[0] + dr, self.pos[1] + dc
        if 0 <= nr < h and 0 <= nc < w and self.maze[nr][nc] != '#':
            self.pos = (nr, nc)
            self.moves = int(self.moves) + 1

class BrickBreaker(SigmaGame):
    GAME_ID = "G43"
    GAME_NAME = "Brick Breaker"
    def _init_state(self):
        self.W = 20
        self.H = 15
        self.paddle_x = 8.0
        self.ball_x = 10.0
        self.ball_y = 10.0
        self.ball_dx = 1.0
        self.ball_dy = -1.0
        self.bricks = [[1]*20 for _ in range(4)]
        self.alive = True
    def tick(self):
        self.ball_x = float(self.ball_x) + float(self.ball_dx)
        self.ball_y = float(self.ball_y) + float(self.ball_dy)
        self.moves = int(self.moves) + 1
