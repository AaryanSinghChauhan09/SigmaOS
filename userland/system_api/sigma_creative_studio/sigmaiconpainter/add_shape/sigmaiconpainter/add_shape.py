# Generated method: SigmaIconPainter.add_shape


class SigmaIconPainter:
    def add_shape(self, shape, color, x, y, size):
        """Adds a vector shape (circle, rect, star, polygon) to the canvas."""
        layer = {'shape': shape, 'color': color, 'x': x, 'y': y, 'size': size}
        self.layers.append(layer)
        return f'IconPainter: Added {shape} [{color}] at ({x},{y}).'