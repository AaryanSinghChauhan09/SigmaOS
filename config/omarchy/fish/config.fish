# SPDX-License-Identifier: MIT
# SigmaOS Omarchy Fish Configuration (config.fish)

set -gx TERM xterm-256color
set -gx EDITOR nvim
set -gx VISUAL nvim

# Path
fish_add_path $HOME/.local/bin $HOME/SigmaOS/bin

# Prompt
if type -q starship
    starship init fish | source
end

# Aliases
alias om="omarchy"
alias pac="sudo pacman"
alias sig="sigpkg"
alias ll="ls -la"
alias gs="git status"
alias gd="git diff"

# Theme function
function theme
    if test (count $argv) -gt 0
        python3 $HOME/SigmaOS/config/omarchy/scripts/apply_theme.py set $argv[1]
    else
        python3 $HOME/SigmaOS/config/omarchy/scripts/apply_theme.py list
    end
end
