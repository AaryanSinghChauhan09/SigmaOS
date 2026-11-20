# SigmaOS /etc/skel .bashrc
# User shell configuration for new accounts.
# Inspired by Omarchy 4.0.5 default/bash/

# If not running interactively, don't do anything.
[[ $- != *i* ]] && return

# Source environment bootstrap.
SIGMAOS_PATH="${SIGMAOS_PATH:-/usr/share/sigmaos}"
if [ -r "${SIGMAOS_PATH}/default/bash/env-bootstrap" ]; then
  . "${SIGMAOS_PATH}/default/bash/env-bootstrap"
fi

# History settings.
HISTCONTROL=ignoreboth
HISTSIZE=1000
HISTFILESIZE=2000
shopt -s histappend

# Check window size after each command.
shopt -s checkwinsize

# Enable color support.
if [ -x /usr/bin/dircolors ]; then
  test -r ~/.dircolors && eval "$(dircolors -b ~/.dircolors)" || eval "$(dircolors -b)"
  alias ls='ls --color=auto'
  alias grep='grep --color=auto'
  alias fgrep='fgrep --color=auto'
  alias egrep='egrep --color=auto'
fi

# Useful aliases (LEAST ASTONISHMENT).
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'

# SigmaOS shortcuts.
alias sigmaos-theme='sigmaos-theme-system sigmaos'
alias sigmaos-capture='sigmaos-capture-screenshot'

# Set a fancy prompt (non-color, unless we know we "want" color).
case "$TERM" in
  xterm-color|*-256color) color_prompt=yes;;
esac

if [ "$color_prompt" = yes ]; then
  PS1='\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
else
  PS1='\u@\h:\w\$ '
fi
unset color_prompt

# Enable programmable completion features.
if ! shopt -oq posix; then
  if [ -f /usr/share/bash-completion/bash_completion ]; then
    . /usr/share/bash-completion/bash_completion
  elif [ -f /etc/bash_completion ]; then
    . /etc/bash_completion
  fi
fi
