# SigmaOS Interactive Shell

Interactive mode owns prompt rendering, terminal-state restoration, signal
handling, and job-control transitions. Every error path must restore terminal
state before returning to the prompt.
