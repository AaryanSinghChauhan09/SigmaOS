# Shell Word Splitting & Field Expansion Study

## Overview
This study evaluates the mechanisms for field splitting, quote removal, and globbing in modern shell interpreters compared to IEEE Std 1003.1-2017.

## Key Findings
1. Unquoted parameter expansion (`$VAR`) must split on characters in `$IFS` (default: space, tab, newline).
2. Quoted parameter expansion (`"$VAR"`) suppresses word splitting and treats the value as a single field.
3. Pathname expansion (globbing) occurs after field splitting on unquoted words containing `*`, `?`, or `[...]`.
