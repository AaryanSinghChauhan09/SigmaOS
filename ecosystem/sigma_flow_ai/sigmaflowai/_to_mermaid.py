# Generated method: SigmaFlowAI._to_mermaid
from typing import Dict, List, Any

class SigmaFlowAI:
    def _to_mermaid(self, nodes: List[Dict], edges: List[Dict]) -> str:
        """Converts internal graph to Mermaid.js format."""
        m = 'graph TD\n'
        for node in nodes:
            label = node['label']
            nid = node['id']
            if node['type'] == 'Decision':
                m += f'    {nid}{{{label}}}\n'
            elif node['type'] == 'Start' or node['type'] == 'End':
                m += f'    {nid}(({label}))\n'
            else:
                m += f'    {nid}[{label}]\n'
        for edge in edges:
            cond = f"|{edge['condition']}|" if 'condition' in edge else ''
            m += f"    {edge['from']} -->{cond} {edge['to']}\n"
        return m