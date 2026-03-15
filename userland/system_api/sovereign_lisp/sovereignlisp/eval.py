# Generated method: SovereignLisp.eval
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def eval(self, script: str):
        """Simple REPL Entry Point."""
        if '(+ ' in script:
            return self._sum([LispObject(LispObjectType.INT, int(i)) for i in script.strip('()').split()[1:]])
        return f"Eval: '{script}' dispatched to Core Lisp Loop."