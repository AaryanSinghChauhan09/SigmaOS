#!/bin/bash
# Add extern crate alloc; after #![ attributes in all .rs files

for file in src/**/*.rs; do
    if grep -q "^use alloc::" "$file" && ! grep -q "^extern crate alloc;" "$file"; then
        # Find the last #![ line and add extern crate alloc; after it
        awk '
        BEGIN { in_attrs = 1 }
        /^#![]/ { print; next }
        in_attrs { print "extern crate alloc;"; in_attrs = 0 }
        { print }
        ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
    fi
done
