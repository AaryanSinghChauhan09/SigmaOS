# SigmaOS zsh configuration — sane defaults + starship prompt.

# History
HISTSIZE=10000
SAVEHIST=10000
setopt SHARE_HISTORY
setopt HIST_IGNORE_DUPS
setopt HIST_IGNORE_SPACE

# Completion
autoload -Uz compinit && compinit -u
zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'

# Editing
bindkey -e

# Directory navigation
setopt AUTO_CD
setopt CORRECT
setopt INTERACTIVE_COMMENTS

# Globbing
setopt EXTENDED_GLOB
setopt GLOB_DOTS

# Starship prompt
if command -v starship >/dev/null 2>&1; then
  eval "$(starship init zsh)"
fi

# SigmaOS commands — thin aliases over the sigmaos-* toolchain.
alias sig-update='sigmaos-update'
alias sig-snap='sigmaos-snapshot'
alias sig-secureboot='sigmaos-secureboot'
alias sig-aur='sigma-aur'
alias sig-rollbacks='sigmaos-snapshot list'
sig-backup() { sigmaos-snapshot create "manual-$(date +%Y%m%d-%H%M%S)"; }

# SigmaOS environment
export EDITOR="${EDITOR:-nvim}"
export VISUAL="${VISUAL:-nvim}"
export TERMINAL="${TERMINAL:-foot}"

# Hyprland session helpers (live desktop)
if [ -n "${WAYLAND_DISPLAY:-}" ]; then
  export XDG_CURRENT_DESKTOP=Hyprland
fi
