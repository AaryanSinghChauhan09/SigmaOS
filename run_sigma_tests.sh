#!/usr/bin/env bash
set -e
echo "=== Running SigmaOS Subsystem Tests ==="
rustc --test src/shell/zsh_bash_parity.rs -o /tmp/test_zsh_bash_parity --cfg 'feature="default"' && /tmp/test_zsh_bash_parity
rustc --test src/media/browser.rs -o /tmp/test_browser && /tmp/test_browser
rustc --test src/compatibility/fedora.rs -o /tmp/test_fedora --cfg 'feature="default"' && /tmp/test_fedora
echo "=== All SigmaOS Subsystem Tests Passed ==="
