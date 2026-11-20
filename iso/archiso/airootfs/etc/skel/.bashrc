# SigmaOS bash configuration — colors, completion, history, starship prompt.

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# History
HISTSIZE=10000
HISTFILESIZE=10000
HISTCONTROL=ignoreboth
shopt -s histappend
shopt -s checkwinsize

# Completion
if [[ -f /usr/share/bash-completion/bash_completion ]]; then
    . /usr/share/bash-completion/bash_completion
elif [[ -f /etc/bash_completion ]]; then
    . /etc/bash_completion
fi

# Colored output
alias ls='ls --color=auto'
alias grep='grep --color=auto'
alias ll='ls -alF'
alias la='ls -A'
alias ..='cd ..'
alias ...='cd ../..'

# SigmaOS prompt (falls back to default PS1 if starship is absent)
if command -v starship >/dev/null 2>&1; then
    eval "$(starship init bash)"
else
    PS1='[\u@\h \W]\$ '
fi
