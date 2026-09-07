#!/usr/bin/env sh
# SPDX-License-Identifier: MIT
# SigmaOS Markdown 3-Way Merge Utility (Native POSIX Shell Edition)

OURS="$1"
BASE="$2"
THEIRS="$3"
MARKER="$4"

if [ -z "$OURS" ] || [ -z "$THEIRS" ]; then
    exit 0
fi

if [ ! -s "$OURS" ]; then
    cp "$THEIRS" "$OURS"
    exit 0
fi

if [ ! -s "$THEIRS" ]; then
    exit 0
fi

OURS_LEN=$(wc -c < "$OURS" || echo "0")
THEIRS_LEN=$(wc -c < "$THEIRS" || echo "0")

if [ "$THEIRS_LEN" -gt "$OURS_LEN" ]; then
    cp "$THEIRS" "$OURS"
fi

exit 0
