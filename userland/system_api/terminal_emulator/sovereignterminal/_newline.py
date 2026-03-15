# Generated method: SovereignTerminal._newline


class SovereignTerminal:
    def _newline(self):
        self.cursor_x = 0
        if self.cursor_y < self.rows - 1:
            self.cursor_y += 1
        else:
            self.grid.pop(0)
            self.grid.append([' ' for _ in range(self.cols)])