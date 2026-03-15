# Generated method: SovereignTerminal.write


class SovereignTerminal:
    def write(self, text: str):
        """Standard Write to VTY Grid."""
        for char in text:
            if char == '\n':
                self._newline()
            elif char == '\r':
                self.cursor_x = 0
            else:
                self.grid[self.cursor_y][self.cursor_x] = char
                self.cursor_x += 1
                if self.cursor_x >= self.cols:
                    self._newline()