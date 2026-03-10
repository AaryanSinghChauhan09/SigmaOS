"""
SigmaOS Visual Logic Engine (Scratch-Style Builder)
=====================================================
USP: Visual Block-Based System Control (No-Code).
Replaces legacy Apple Shortcuts and IFTTT.
"""
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

@dataclass
class VisualBlock:
    id: str
    type: str  # e.g., "trigger", "action", "control"
    name: str  # e.g., "On_Time", "Apply_Focus", "If_Battery_Low"
    inputs: Dict[str, Union[str, float]] = field(default_factory=dict)
    next_block_id: str = None

class SigmaVisualLogic:
    """The 'Scratch' engine for SigmaOS. Parses block chains into routines."""

    def __init__(self, kernel=None):
        self.kernel = kernel
        self.active_canvas: Dict[str, VisualBlock] = {}
        self._stats = {"blocks_placed": 0, "chains_compiled": 0}

    def place_block(self, block_type: str, name: str, **inputs) -> str:
        bid = f"block_{len(self.active_canvas)}"
        block = VisualBlock(bid, block_type, name, inputs)
        self.active_canvas[bid] = block
        self._stats["blocks_placed"] += 1
        return bid

    def connect_blocks(self, source_id: str, target_id: str):
        if source_id in self.active_canvas and target_id in self.active_canvas:
            self.active_canvas[source_id].next_block_id = target_id
            return f"Connected: {source_id} -> {target_id}"
        return "Error: Block(s) not found."

    def compile_chain(self, start_block_id: str) -> dict:
        """Translates a block-chain into a SigmaRoutine compatible JSON."""
        if start_block_id not in self.active_canvas:
            return {"status": "error", "msg": "Invalid start block."}
        
        chain = []
        current_id = start_block_id
        while current_id:
            block = self.active_canvas[current_id]
            chain.append({
                "name": block.name,
                "type": block.type,
                "inputs": block.inputs
            })
            current_id = block.next_block_id
        
        self._stats["chains_compiled"] += 1
        return {
            "status": "compiled",
            "routine_name": f"Visual_{start_block_id}",
            "execution_steps": chain,
            "complexity": len(chain)
        }

    def get_block_library(self):
        """Returns the 'Scratch Pallete' available for the user."""
        return {
            "Triggers": ["On_Timer", "On_Location", "On_Mesh_Event", "On_Battery"],
            "Actions":  ["Set_Theme", "Lock_Vault", "Broadcast_Mesh", "Start_Mission", "Apply_Routine"],
            "Control":  ["If/Else", "Wait", "Repeat", "Parallel_Wait"]
        }

    def health_check(self) -> str:
        s = self._stats
        return f"OK — Blocks: {s['blocks_placed']}, Compiled: {s['chains_compiled']}."
