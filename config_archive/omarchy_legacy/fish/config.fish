# SPDX-License-Identifier: MIT
# SigmaOS Sovereign Desktop — Fish Shell Configuration
# Managed by: sigmaos-update --dotfiles-only
# Source: config/omarchy/fish/config.fish

# ── Environment ───────────────────────────────────────────────────────────────
set -gx TERM xterm-256color
set -gx EDITOR nvim
set -gx VISUAL nvim
set -gx PAGER less
set -gx BROWSER firefox

# Wayland / Hyprland environment
set -gx XDG_CURRENT_DESKTOP Hyprland
set -gx XDG_SESSION_TYPE wayland
set -gx MOZ_ENABLE_WAYLAND 1
set -gx QT_QPA_PLATFORM wayland
set -gx GDK_BACKEND wayland

# ── PATH ──────────────────────────────────────────────────────────────────────
fish_add_path $HOME/.local/bin
fish_add_path $HOME/SigmaOS/bin
fish_add_path $HOME/.cargo/bin

# ── Prompt: Starship ──────────────────────────────────────────────────────────
if type -q starship
    starship init fish | source
end

# ── Abbreviations ─────────────────────────────────────────────────────────────
# System
abbr --add pac  'sudo pacman'
abbr --add pacs 'sudo pacman -S'
abbr --add pacu 'sudo pacman -Syu'
abbr --add paci 'pacman -Qi'
abbr --add pacr 'sudo pacman -Rs'

# SigmaOS
abbr --add sig    'sigmaos-update'
abbr --add sigd   'sigmaos-desktop'
abbr --add sigdr  'sigmaos-doctor'
abbr --add sigu   'sigmaos-update'
abbr --add sigudt 'sigmaos-update --dotfiles-only'
abbr --add sigupk 'sigmaos-update --packages-only'
abbr --add sigumt 'sigmaos-update --theme-only'

# Git
abbr --add gs  'git status'
abbr --add gd  'git diff'
abbr --add ga  'git add -A'
abbr --add gc  'git commit -m'
abbr --add gp  'git push'
abbr --add gl  'git log --oneline -10'

# Misc
abbr --add ll  'ls -la'
abbr --add la  'ls -A'
abbr --add ..  'cd ..'
abbr --add ... 'cd ../..'

# ── SigmaOS convenience functions ─────────────────────────────────────────────

# theme: list or apply a Wallust theme
function theme
    if test (count $argv) -gt 0
        sigmaos-desktop theme $argv[1]
    else
        sigmaos-desktop theme list
    end
end

# update: full system update with confirmation
function update
    echo "⚡ SigmaOS System Update"
    sigmaos-update $argv
end

# doctor: health check alias
function doctor
    sigmaos-doctor
end

# mkcd: mkdir + cd
function mkcd
    mkdir -p $argv[1] && cd $argv[1]
end

# extract: universal archive extractor
function extract
    switch $argv[1]
        case '*.tar.gz'  ; tar xzf $argv[1]
        case '*.tar.bz2' ; tar xjf $argv[1]
        case '*.tar.xz'  ; tar xJf $argv[1]
        case '*.tar.zst' ; tar --zstd -xf $argv[1]
        case '*.zip'     ; unzip $argv[1]
        case '*.7z'      ; 7z x $argv[1]
        case '*.gz'      ; gunzip $argv[1]
        case '*.bz2'     ; bunzip2 $argv[1]
        case '*'
            echo "extract: unknown archive format: $argv[1]"
            return 1
    end
end

# ── SSH agent ────────────────────────────────────────────────────────────────
if status is-interactive
    if not set -q SSH_AUTH_SOCK
        eval (ssh-agent -c) > /dev/null 2>&1
    end
end

# ── Greeting ─────────────────────────────────────────────────────────────────
if status is-interactive
    if type -q sigmaos
        echo ""
        echo "⚡ SigmaOS Sovereign Desktop — type 'doctor' to check system health"
        echo ""
    end
end
