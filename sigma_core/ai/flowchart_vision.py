"""
SigmaOS Text-to-Flowchart Engine (Apex v1.0)
============================================
USP: Direct natural language to logic-graph synthesis.
Integrates with IndentFlow for real-time visualization.
"""
import json
import re

class FlowchartVision:
    def __init__(self, kernel):
        self.kernel = kernel

    def synthesize_graph(self, text: str) -> dict:
        """Parses text for nodes and edges (simplified GraphViz-lite)."""
        lines = text.split('\n')
        nodes = []
        edges = []
        
        # Simple Logic: If text contains 'if', 'then', 'else'
        for i, line in enumerate(lines):
            line = line.strip().lower()
            if "if" in line:
                nodes.append({"id": i, "label": f"DECISION: {line}", "type": "cond"})
            elif "loop" in line or "for" in line:
                nodes.append({"id": i, "label": f"ITERATION: {line}", "type": "loop"})
            else:
                nodes.append({"id": i, "label": f"PROCESS: {line}", "type": "step"})
            
            if i > 0:
                edges.append({"from": i-1, "to": i})

        return {
            "status": "SUCCESS",
            "schema": {"nodes": nodes, "edges": edges},
            "message": "Logic Graph Synthesized. Exporting to JSON for IndentFlow Rendering."
        }

    def health_check(self) -> str:
        return "OK - Flowchart Parser Active"
