# SigmaOS /etc/skel .profile
# Login shell profile.

# Source .bashrc for interactive shells.
if [ -n "$BASH_VERSION" ]; then
  if [ -f "$HOME/.bashrc" ]; then
    . "$HOME/.bashrc"
  fi
fi

# Set PATH to include user's private bin if it exists.
if [ -d "$HOME/bin" ] ; then
  PATH="$HOME/bin:$PATH"
fi

if [ -d "$HOME/.local/bin" ] ; then
  PATH="$HOME/.local/bin:$PATH"
fi
