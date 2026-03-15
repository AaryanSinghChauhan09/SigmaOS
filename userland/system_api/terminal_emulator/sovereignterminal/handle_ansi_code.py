# Generated method: SovereignTerminal.handle_ansi_code


class SovereignTerminal:
    def handle_ansi_code(self, code: str):
        """USP: ANSI Escape Code State Machine (e.g. \x1b[31m)."""
        if '[31m' in code:
            self.fg_color = (255, 0, 0)
        if '[32m' in code:
            self.fg_color = (0, 255, 0)
        if '[0m' in code:
            self.fg_color = (255, 255, 255)