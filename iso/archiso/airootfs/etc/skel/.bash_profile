# SigmaOS live session — boot straight to the Hyprland desktop on tty1.
# Falls back to a normal shell on SSH, other VTs, or if Hyprland exits.
[[ -f ~/.bashrc ]] && . ~/.bashrc

if [ -z "${SSH_TTY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ] && [ "${XDG_VTNR:-}" = "1" ]; then
    command -v Hyprland >/dev/null 2>&1 && Hyprland
fi
