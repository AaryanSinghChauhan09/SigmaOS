"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaIconPainter.new_icon
"""



class SigmaIconPainter:
    def new_icon(self, size=(64, 64), background='#1a1a2e'):
        self.canvas_size = size
        return f'IconPainter: New {size[0]}x{size[1]} canvas created. Background: {background}.'
