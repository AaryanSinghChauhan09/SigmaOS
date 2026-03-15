# Generated method: UIMixin._console
import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    def _console(self, parent, height=8) -> scrolledtext.ScrolledText:
        st = scrolledtext.ScrolledText(parent, bg='#0A0A14', fg=PAL['green'], insertbackground=PAL['cyan'], font=FONT_MONO, height=height, relief='flat', selectbackground=PAL['accent'])
        st.tag_configure('OK', foreground=PAL['green'])
        st.tag_configure('WARN', foreground=PAL['gold'])
        st.tag_configure('ERR', foreground=PAL['red'])
        st.tag_configure('INFO', foreground=PAL['cyan'])
        st.tag_configure('HEAD', foreground=PAL['accent2'], font=('Consolas', 10, 'bold'))
        return st