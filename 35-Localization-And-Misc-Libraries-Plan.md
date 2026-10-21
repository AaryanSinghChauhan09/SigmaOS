# Sovereign Localization and Misc Libraries Plan (locales, gettext, iconv, tzdata, zlib, libffi, ncurses, readline, SQLite, PCRE, libxml2, ICU, libpng, libjpeg, libtiff)

## Executive Summary

This plan outlines the architecture, implementation, and Pull Request (PR) proposals for Linux & BSD inspired Localization and Core Utility Libraries in SigmaOS. The suite provides `#![no_std]` and `alloc`-compatible zero-dependency Rust implementations and Pull Request proposals across 6 core domains:

1. **locales & gettext**: Message translation catalogs, plural form expression evaluator for 150+ language families, `LC_MESSAGES` domain resolution, binary `.mo` / text `.po` catalog parsing.
2. **iconv & tzdata**: Universal character encoding conversion (`UTF-8`, `ISO-8859-1`, `ASCII`, `Shift-JIS`, `UTF-16LE/BE`) and IANA tzdata time zone database transition rule parser with DST offset calculation.
3. **zlib & libffi**: Fast Zlib RFC 1950 DEFLATE stream compressor, Adler-32 / CRC32 checksums, and libffi C-ABI call descriptors, argument marshaling, and calling convention trampolines (`SysV`, `StdCall`, `FastCall`, `Win64`).
4. **ncurses & readline**: Terminfo terminal capability parser, double-buffered VT100 rendering with ANSI diff sequence generation, GNU readline history ring buffer, tab auto-completion, and Emacs/Vi modal editing keybindings.
5. **SQLite & PCRE**: Embedded SQLite3 B-tree table storage scanner, SQL query parser, ACID WAL journal state manager, and PCRE2 regular expression engine with named capture group extraction and back-references.
6. **libxml2, ICU, & Image Codecs (libjpeg, libpng, libtiff)**: libxml2 SAX/DOM XML query parser, ICU Unicode grapheme/word boundary segmentation, and PNG/JPEG/TIFF magic-byte header parsing, chunk validation, and color space metadata extraction.

---

## Core Components & Architecture

### 1. `LocalesAndGettextInspirationsPrEngine`
- `resolve_effective_locale(category: &str) -> String`: Precedence solver (`LC_ALL` > `LC_MESSAGES`/`LC_TIME` > `LANG` > `"C"`).
- `evaluate_plural_formula(count: u64, locale: &str) -> usize`: Plural form index calculator.
- `gettext`, `dgettext`, `ngettext`: Translation catalog lookups.

### 2. `IconvAndTzdataInspirationsPrEngine`
- `iconv_convert(from: CharacterEncoding, to: CharacterEncoding, input: &[u8])`: Transcoding engine.
- `resolve_tz_offset(tz_name: &str, timestamp: i64)`: IANA tzdata Olson transition rule solver.

### 3. `ZlibAndLibffiInspirationsPrEngine`
- `adler32`, `crc32`: SIMD checksum routines.
- `compress_deflate`: DEFLATE stream compressor.
- `prep_ffi_cif`, `invoke_ffi_call`: Dynamic C-ABI function call marshaling.

### 4. `NcursesAndReadlineInspirationsPrEngine`
- `mvaddch`, `refresh_screen_delta`: Double-buffered screen matrix diffing and ANSI VT100 command generator.
- `readline_add_history`, `readline_complete`: History ring buffer and tab auto-completion.

### 5. `SqliteAndPcreInspirationsPrEngine`
- `query_select_all`, `insert_row`: Embedded B-Tree table scanner and WAL transaction journal.
- `pcre_match_named`: PCRE2 regex matcher with named group captures (`(?P<name>...)`).

### 6. `Libxml2IcuAndImageCodecsInspirationsPrEngine`
- `parse_xml_well_formed`: SAX/DOM XML well-formedness checker with tag stack validation.
- `icu_grapheme_segment`: ICU grapheme boundary segmentation.
- `parse_image_header`: Magic-byte header parser for PNG, JPEG, and TIFF formats.

---

## Verification & Test Plan

1. **Unit Tests**:
   - `test_locales_and_gettext_full_workflow`
   - `test_iconv_and_tzdata_full_workflow`
   - `test_zlib_and_libffi_full_workflow`
   - `test_ncurses_and_readline_full_workflow`
   - `test_sqlite_and_pcre_full_workflow`
   - `test_libxml2_icu_and_codecs_full_workflow`
   - `test_master_pr_gateway_full_workflow`
2. **Integration Verification**:
   - Run `./run_sigma_tests.sh` to confirm 100% pass rate across the entire test suite.
   - Run `./scripts/sync_wiki.sh` to synchronize wiki documentation assets.
