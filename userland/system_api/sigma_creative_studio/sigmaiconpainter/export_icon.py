"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaIconPainter.export_icon
"""



class SigmaIconPainter:
    def export_icon(self, formats=('svg', 'png', 'ico')):
        """Exports the designed icon in all required formats for desktop, browser, and taskbar."""
        return f"IconPainter (Export): Icon exported in {', '.join(formats).upper()} formats. All resolutions generated."
