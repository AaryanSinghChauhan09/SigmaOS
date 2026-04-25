"""
SigmaOS SQLite Vector Memory Bindings
Provides persistent, queryable memory with zero third-party dependencies.
"""
import sqlite3
import os
import json
from typing import List, Dict, Any

class DbMemoryLayer:
    def __init__(self, db_path: str = "sigma_memory.db"):
        self.db_path = db_path
        self._init_db()

    def _init_db(self):
        with sqlite3.connect(self.db_path) as conn:
            conn.execute('''
                CREATE TABLE IF NOT EXISTS memory_vectors (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    intent TEXT NOT NULL,
                    vector_data TEXT NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            conn.commit()

    def store(self, intent: str, vector_data: List[float]):
        """Store a vectorized representation of an OS event."""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute(
                'INSERT INTO memory_vectors (intent, vector_data) VALUES (?, ?)',
                (intent, json.dumps(vector_data))
            )
            conn.commit()
        print(f"[VectorDB] Stored memory vector for intent: '{intent}'")

    def query(self, intent_filter: str) -> List[Dict[str, Any]]:
        """Query memory for past context."""
        with sqlite3.connect(self.db_path) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.execute(
                'SELECT * FROM memory_vectors WHERE intent LIKE ?', 
                (f"%{intent_filter}%",)
            )
            return [dict(row) for row in cursor.fetchall()]

    def prune_stale(self, days_old: int = 30):
        """Automated pruning to reduce dependency/storage bloat."""
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute(
                "DELETE FROM memory_vectors WHERE timestamp <= date('now', ?)",
                (f"-{days_old} days",)
            )
            conn.commit()
            print(f"[VectorDB] Pruned {cursor.rowcount} stale memory vectors.")
