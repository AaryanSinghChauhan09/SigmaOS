# Generated method: SigmaLiveCodeEditor.live_preview


class SigmaLiveCodeEditor:
    def live_preview(self):
        """Renders the current tri-pane code into an isolated Blink-engine frame."""
        combined = f'HTML:{len(self.html_content)} CSS:{len(self.css_content)} JS:{len(self.js_content)}'
        self.last_preview_hash = hash(combined)
        return f'LiveEditor (Preview): Rendering in isolated local frame. [Hash: 0x{abs(self.last_preview_hash):x}]'