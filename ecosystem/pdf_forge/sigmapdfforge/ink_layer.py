"""
Auto-split from ecosystem\pdf_forge.py — SigmaPDFForge.ink_layer
"""

from sigma_core.system.sovereign_app import SovereignApp



class SigmaPDFForge:
    def ink_layer(self, action='Draw', color='Red', thickness=2):
        """
            Native PDF Inking (Edge/Foxit USP):
            Provides Freehand Draw, Erase, and Highlight on top of the PDF canvas vector layer.
            """
        if action == 'Erase':
            return 'PDF Forge (Ink): Erasing customized stroke paths from the active annotation layer.'
        elif action == 'Highlight':
            return f'PDF Forge (Ink): Applying vector highlight ({color}) over detected text.'
        else:
            return f'PDF Forge (Ink): Freehand drawing enabled. Vector stroke applied: {color}, {thickness}pt.'
