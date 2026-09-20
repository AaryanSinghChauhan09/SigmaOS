# SigmaOS Application Translation & Message Catalog Guide

## 1. Executive Summary

SigmaOS provides a high-performance, zero-allocation translation framework (`gettext` equivalent) designed for system services, CLI utilities, and desktop GUI applications. It relies on message catalog files (`.po` portable objects and compiled `.sigmsg` binary objects) with support for plural forms, variable interpolation, and domain isolation.

## 2. Translation File Workflow

```
+-----------------------------------------------------------+
| 1. Source Code with tr!() / gettext() macros              |
+-----------------------------+-----------------------------+
                              |
               sigmsg-extract / xgettext
                              |
+-----------------------------v-----------------------------+
| 2. Master Template (.pot / messages.pot)                 |
+-----------------------------+-----------------------------+
                              |
                     sigmsg-merge / msgmerge
                              |
+-----------------------------v-----------------------------+
| 3. Translator PO Files (po/de.po, po/ja.po, po/fr.po)    |
+-----------------------------+-----------------------------+
                              |
                    sigmsg-fmt / msgfmt
                              |
+-----------------------------v-----------------------------+
| 4. Binary Message Catalogs (/usr/share/locale/*/LC_MESSAGES)
+-----------------------------------------------------------+
```

## 3. Message Catalog Format

### 3.1 PO Translation File Example (`po/de.po`)
```po
msgid ""
msgstr ""
"Project-Id-Version: SigmaOS Desktop 1.0\n"
"Language: de\n"
"MIME-Version: 1.0\n"
"Content-Type: text/plain; charset=UTF-8\n"
"Plural-Forms: nplurals=2; plural=(n != 1);\n"

msgid "Settings"
msgstr "Einstellungen"

msgid "Welcome, {username}!"
msgstr "Willkommen, {username}!"

msgid "Found {count} file."
msgid_plural "Found {count} files."
msgstr[0] "{count} Datei gefunden."
msgstr[1] "{count} Dateien gefunden."
```

### 3.2 Binary Message Catalog Format (`.sigmsg`)
The `.sigmsg` binary catalog format uses a zero-copy memory-mapped hash index allowing O(1) string lookup times without startup parsing overhead.

## 4. Code Developer Integration

### 4.1 Rust Application Integration
```rust
use sigmaos_i18n::{tr, tr_plural, set_domain};

fn main() {
    set_domain("zenith-control-center");

    // Simple translation lookup
    println!("{}", tr!("Settings"));

    // String interpolation with variables
    let username = "Alice";
    println!("{}", tr!("Welcome, {username}!", username = username));

    // Plural form translation
    let count = 5;
    println!("{}", tr_plural!(
        "Found {count} file.",
        "Found {count} files.",
        count,
        count = count
    ));
}
```

### 4.2 C / C++ Developer Integration
```c
#include <sigmaos/i18n.h>
#include <stdio.h>

int main(void) {
    bindtextdomain("my_c_app", "/usr/share/locale");
    textdomain("my_c_app");

    printf("%s\n", _("Settings"));
    return 0;
}
```
