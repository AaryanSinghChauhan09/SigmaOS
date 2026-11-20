# SPDX-License-Identifier: MIT
# SigmaOS Omarchy Zsh Configuration (.zshrc)

export TERM="xterm-256color"
export EDITOR="nvim"
export VISUAL="nvim"

# Path configuration
export PATH="$HOME/.local/bin:$HOME/SigmaOS/bin:$PATH"

# Prompt
if command -v starship >/dev/null 2>&1; then
    eval "$(starship init zsh)"
fi

# Omarchy & SigmaOS Aliases
alias om="omarchy"
alias pac="sudo pacman"
alias sig="sigpkg"
alias ll="ls -la --color=auto"
alias gs="git status"
alias gd="git diff"
alias gcm="git commit -m"
alias vim="nvim"
alias v="nvim"
alias c="clear"

# Wallust Theme Switcher
theme() {
    if [ -n "$1" ]; then
        python3 "$HOME/SigmaOS/config/omarchy/scripts/apply_theme.py" set "$1"
    else
        python3 "$HOME/SigmaOS/config/omarchy/scripts/apply_theme.py" list
    fi
}

# Autocompletion & History
HISTFILE="$HOME/.zsh_history"
HISTSIZE=10000
SAVEHIST=10000
setopt APPEND_HISTORY
setopt SHARE_HISTORY
setopt HIST_IGNORE_DUPS
