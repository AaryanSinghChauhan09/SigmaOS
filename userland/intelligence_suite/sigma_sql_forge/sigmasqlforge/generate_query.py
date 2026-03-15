# Generated method: SigmaSQLForge.generate_query
import time
import re
from typing import List, Dict, Any, Optional

class SigmaSQLForge:
    def generate_query(self, request: str, table_schema: Dict[str, str]) -> str:
        """Simulates NL2SQL conversion."""
        req = request.lower()
        table_name = list(table_schema.keys())[0] if table_schema else 'generic_table'
        columns = ', '.join(table_schema.get(table_name, 'id, val').split(','))
        query = f'SELECT {columns} FROM {table_name}'
        if 'where' in req or 'filter' in req:
            query += ' WHERE val > 100'
        if 'sort' in req or 'order' in req:
            query += ' ORDER BY id DESC'
        return query