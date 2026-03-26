# Generated method: SovereignTerminal.__init__


class SovereignTerminal:
    def __init__(self, kernel, win_id: str):
        self.kernel = kernel
        self.win_id = win_id
        self.cols = 80
        self.rows = 25
        self.grid = [[' ' for _ in range(self.cols)] for _ in range(self.rows)]
        self.cursor_x = 0
        self.cursor_y = 0
        self.fg_color = (255, 255, 255)
        self.bg_color = (0, 0, 0, 150)