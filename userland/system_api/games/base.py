from typing import List, Tuple, Dict, Optional, Any, Type, Set

class SigmaGame:
    """Base class for all SigmaOS games."""
    GAME_ID   : str = "G00"
    GAME_NAME : str = "Unknown"
    CATEGORY  : str = "Misc"
    VERSION   : str = "1.0.0"
    SIZE_KB   : int = 0
    ICON      : str = "🎮"
    DESC      : str = ""
    COMPRESSED: bool = True

    def __init__(self):
        # Core Engine Attributes
        self.score: int = 0
        self.moves: int = 0
        self.started: bool = False
        self.level: int = 1
        self.game_over: bool = False
        self.won: bool = False
        self.won_flag: bool = False
        self.solved: bool = False
        self.alive: bool = True

        # Grid & Board Attributes
        self.grid: Any = None
        self.board: Any = None
        self.boards: Any = None
        self.W: Any = 0
        self.H: Any = 0
        self.rows: int = 0
        self.cols: int = 0
        self.active_board: Any = None

        # Logic & Data Attributes
        self.q: Any = ""
        self.ans: Any = ""
        self.cards: Any = []
        self.ships: Any = []
        self.hits: Any = []
        self.misses: Any = []
        self.matched: Any = set()
        self.revealed: Any = None
        self.flagged: Any = None
        self.cards_matched: Any = set()
        self.sunk: Any = []
        self.heaps: Any = []
        self.deck: List[Any] = []
        self.hand: List[Any] = []
        self.discard: List[Any] = []
        self.rack: Any = None
        self.items: Any = None
        self.inventory: List[Any] = []
        
        # Word & Text Attributes
        self.words: Any = []
        self.word_idx: int = 0
        self.correct: int = 0
        self.target_word: str = ""
        self.current_word: str = ""
        self.end: Any = ""
        self.chain: Any = []
        self.target: Any = None
        self.current: Any = None
        self.row_clues: Any = None
        self.col_clues: Any = None
        self.clues: Any = None
        self.answers: Any = None
        self.solution: Any = None

        # Physics & Entity Attributes
        self.velocity: float = 0.0
        self.pos_x: float = 0.0
        self.pos_y: float = 0.0
        self.speed: float = 0.0
        self.lane: int = 0
        self.distance: float = 0.0
        self.health: int = 100
        self.energy: int = 0
        self.entities: List[Any] = []
        self.agents: Any = []
        self.obstacles: Any = None
        self.goal_x: Any = 0
        self.start: Any = 0.0
        self._last: float = 0.0

        # Clicker & Idle Attributes
        self.points: float = 0.0
        self.cps: float = 0.0
        self.click_val: int = 1
        self.upgrades: Dict[str, Any] = {}
        self.prices: Dict[str, Any] = {}
        self.cps_add: Dict[str, Any] = {}
        self.streak: int = 0
        self.idx: int = 0
        self.hints_used: int = 0
        self.turn: Any = ""
        self.game_stats: Dict[str, Any] = {}

    def hydrate(self):
        """Unpack the full playable state (simulated)."""
        self.COMPRESSED = False
        return f"{self.GAME_NAME} logic hydrated."

    def get_info(self) -> Dict[str, Any]:
        return {
            "id": self.GAME_ID,
            "name": self.GAME_NAME,
            "category": self.CATEGORY,
            "version": self.VERSION,
            "size": f"{self.SIZE_KB}KB",
            "icon": self.ICON,
            "desc": self.DESC
        }

    def _init_state(self):
        """Initialise the internal game state."""
        pass

    def health_check(self) -> str:
        """Self-diagnostic for the game logic."""
        return f"OK — {self.GAME_NAME} Logic Interface Active."
