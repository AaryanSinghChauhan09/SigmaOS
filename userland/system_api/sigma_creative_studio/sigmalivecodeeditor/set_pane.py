"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaLiveCodeEditor.set_pane
"""



class SigmaLiveCodeEditor:
    def set_pane(self, pane, code):
        """Updates the content of a specific pane (html, css, or js)."""
        pane = pane.lower()
        if pane == 'html':
            self.html_content = code
        elif pane == 'css':
            self.css_content = code
        elif pane == 'js':
            self.js_content = code
        else:
            return f"Error: Unknown pane '{pane}'."
        return f'LiveEditor ({pane.upper()}): Pane updated. Preview will hot-reload.'
