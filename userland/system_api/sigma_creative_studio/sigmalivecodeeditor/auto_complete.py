"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaLiveCodeEditor.auto_complete
"""



class SigmaLiveCodeEditor:
    def auto_complete(self, partial_tag):
        """Emmet-style auto-completion for HTML/CSS abbreviations."""
        completions = {'div': '<div></div>', 'ul>li*3': '<ul>\n  <li></li>\n  <li></li>\n  <li></li>\n</ul>', 'link': '<link rel="stylesheet" href="styles.css">', '!': '<!DOCTYPE html><html><head></head><body></body></html>'}
        return completions.get(partial_tag, f"Emmet: No shorthand for '{partial_tag}'.")
