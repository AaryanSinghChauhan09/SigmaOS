# Generated file: handle_kernel_fault
import tkinter as tk
from tkinter import scrolledtext
from gui_pkg.styles import PAL, FONT_MONO, FONT_SMALL, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_LOGO

def handle_kernel_fault(parent, exc_type, exc_val, exc_tb, is_child_mode=True):
    """Standard Kernel Fault Interceptor Module."""
    import traceback
    err_msg = ''.join(traceback.format_exception(exc_type, exc_val, exc_tb))
    print(err_msg)
    fault_win = tk.Toplevel(parent)
    fault_win.attributes('-topmost', True)
    fault_win.attributes('-fullscreen', True)
    fault_win.configure(bg='#0D1117')
    face_text = '^_^' if is_child_mode else ':('
    tk.Label(fault_win, text=face_text, font=('Inter', 120), fg='white', bg='#0D1117').place(relx=0.1, rely=0.2)
    fault_title = 'SIGMA HAS A LITTLE BOO-BOO' if is_child_mode else 'SIGMA_KERNEL_SERVICE_FAULT'
    tk.Label(fault_win, text=fault_title, font=('Consolas', 24), fg='white', bg='#0D1117').place(relx=0.1, rely=0.45)
    desc_text = 'Oops! Sigma needs a tiny rest to feel better.' if is_child_mode else 'Your Sovereign instance ran into a problem and needs to reconstruct.'
    tk.Label(fault_win, text=desc_text, font=('Inter', 14), fg=PAL['dim'], bg='#0D1117').place(relx=0.1, rely=0.55)
    if not is_child_mode:
        scroll = scrolledtext.ScrolledText(fault_win, bg='#000000', fg=PAL['red'], font=FONT_MONO, height=15)
        scroll.place(relx=0.1, rely=0.62, relwidth=0.8)
        scroll.insert('1.0', err_msg)
        scroll.config(state='disabled')

    def _reboot():
        fault_win.destroy()
        if hasattr(parent, '_morphic_island'):
            final_msg = 'SIGMA FEELS BETTER!' if is_child_mode else 'KERNEL RECONSTRUCTED'
            parent._morphic_island(final_msg, PAL['green'], 5000)
    btn_text = 'FIX BOO-BOO' if is_child_mode else 'RECONSTRUCT KERNEL (SOFT REBOOT)'
    tk.Button(fault_win, text=btn_text, font=('Inter Bold', 12), bg=PAL['accent'], fg='white', padx=20, pady=10, relief='flat', command=_reboot).place(relx=0.1, rely=0.9)