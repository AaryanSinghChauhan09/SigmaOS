# Generated method: SigmaBlockCoder.add_sprite


class SigmaBlockCoder:
    def add_sprite(self, name, asset_path):
        """Registers a visual sprite (character/object) on the stage."""
        self.sprites[name] = {'asset': asset_path, 'x': 0, 'y': 0, 'size': 100}
        return f"BlockCoder: Sprite '{name}' added from '{asset_path}'. Ready on stage."