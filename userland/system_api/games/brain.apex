from sigma_core.hal.native_libc import ApexLib
import random
# import time (Purged by Scrubber)
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class TypingSpeedTest(SigmaGame):
    GAME_ID = "G35"
    GAME_NAME = "Typing Speed Test"
    WORDS = ["sigma", "os", "kernel", "quantum", "apex"]
    def _init_state(self):
        self.words = random.sample(self.WORDS, 5)
        self.word_idx = 0
        self.correct = 0
        self.start = time.time()
    def type_word(self, typed: str):
        if int(self.word_idx) >= 5:
            return "Done."
        if typed.strip() == self.words[self.word_idx]:
            self.correct = int(self.correct) + 1
            self.score = int(self.score) + 10
        self.word_idx = int(self.word_idx) + 1
        self.moves = int(self.moves) + 1

class Riddler(SigmaGame):
    GAME_ID = "G51"
    GAME_NAME = "The Riddler"
    RIDDLES = [{"q":"I speak without a mouth...","a":"echo"}]
    def _init_state(self):
        self.idx = 0
        self.hints_used = 0
    def answer(self, ans: str):
        if ans.lower().strip() == self.RIDDLES[0]["a"]:
            self.score = int(self.score) + 50
            self.idx = int(self.idx) + 1
            self.moves = int(self.moves) + 1

class MensaPuzzle(SigmaGame):
    GAME_ID = "G53"
    GAME_NAME = "Mensa IQ Puzzle"
    QUESTIONS = [{"q":"2, 4, 8, 16, ?","a":"32"}]
    def _init_state(self):
        self.idx = 0
        self.streak = 0
    def answer(self, ans: str):
        if ans.strip() == self.QUESTIONS[0]["a"]:
            self.score = int(self.score) + 100
            self.idx = int(self.idx) + 1
            self.moves = int(self.moves) + 1

class FastFive(SigmaGame):
    GAME_ID = "G58"
    GAME_NAME = "Fast Five"
    QUESTIONS = [{"q":"2 + 2 × 3 = ?","a":"8"}]
    def _init_state(self):
        self.pool = self.QUESTIONS
        self.idx = 0
        self.start = time.time()
    def answer(self, ans: str):
        if ans.strip() == self.QUESTIONS[0]["a"]:
            self.score = int(self.score) + 50
            self.idx = int(self.idx) + 1
            self.moves = int(self.moves) + 1

class FindTheWord(SigmaGame):
    GAME_ID = "G48"
    GAME_NAME = "Find the Word"
    def _init_state(self):
        self.grid = [["A"]*8 for _ in range(8)]
        self.matched = set()

class ScrambleGame(SigmaGame):
    GAME_ID = "G49"
    GAME_NAME = "Scramble"
    def _init_state(self):
        self.word = "PYTHON"
        self.scrambled = "YTPHON"

class Spellathon(SigmaGame):
    GAME_ID = "G50"
    GAME_NAME = "Spellathon"
    def _init_state(self):
        self.centre = "A"
        self.letters = set("ABCEFLO")
        self.matched = set()
