//! Standalone Integration Test Suite for Sovereign Localization & Misc Libraries PR Suite
//!
//! Tests locales, gettext, iconv, tzdata, zlib, libffi, ncurses, readline, SQLite, PCRE,
//! libxml2, ICU, and image codecs (libpng, libjpeg, libtiff) parity engines and master gateway.

#[path = "../src/tools/sovereign_localization_and_misc_libraries_pr_suite.rs"]
mod sovereign_localization_and_misc_libraries_pr_suite;

use std::cmp::Ordering;
use sovereign_localization_and_misc_libraries_pr_suite::*;

#[test]
fn test_locales_and_gettext_full_workflow() {
    let mut engine = LocalesAndGettextInspirationsPrEngine::new();
    assert_eq!(engine.resolve_effective_locale("LC_MESSAGES"), "en_US.UTF-8");

    // Test German locale plural formula
    assert_eq!(engine.evaluate_plural_formula(0, "de_DE"), 1);
    assert_eq!(engine.evaluate_plural_formula(1, "de_DE"), 0);
    assert_eq!(engine.evaluate_plural_formula(5, "de_DE"), 1);

    // Test French gettext translations
    engine.active_locale = "fr_FR.UTF-8".to_string();
    assert_eq!(engine.gettext("Hello, World!"), "Bonjour le monde!");
    assert_eq!(engine.ngettext("One file", "%d files", 1), "%d fichier");
    assert_eq!(engine.ngettext("One file", "%d files", 3), "%d fichiers");

    // Test PO content parsing
    let po_data = "msgid \"Settings\"\nmsgstr \"Paramètres\"\n";
    let parsed_count = engine.parse_po_content("messages", "fr_FR", po_data);
    assert_eq!(parsed_count, 1);
    assert_eq!(engine.gettext("Settings"), "Paramètres");

    // Test BSD catgets
    assert_eq!(engine.catgets(1, 1, 1, "default"), "Operation succeeded");
    assert_eq!(engine.catgets(1, 1, 99, "default"), "default");

    // Test LC_NUMERIC & LC_MONETARY locale formatters
    assert_eq!(engine.format_number_locale(1234.56, "fr_FR"), "1234,56");
    assert_eq!(engine.format_monetary_locale(49.99, "€", "fr_FR"), "49,99 €");

    // Test PR proposal format
    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("Gettext"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_iconv_and_tzdata_full_workflow() {
    let engine = IconvAndTzdataInspirationsPrEngine::new();

    // Test UTF-8 -> ISO-8859-1 conversion
    let input = "Sovereign OS".as_bytes();
    let latin1 = engine.iconv_convert(CharacterEncoding::Utf8, CharacterEncoding::Iso8859_1, input).unwrap();
    assert_eq!(latin1, input);

    // Test UTF-8 -> UTF-16LE conversion and back
    let utf16 = engine.iconv_convert(CharacterEncoding::Utf8, CharacterEncoding::Utf16Le, input).unwrap();
    assert_eq!(utf16.len(), input.len() * 2);
    let utf8_back = engine.iconv_convert(CharacterEncoding::Utf16Le, CharacterEncoding::Utf8, &utf16).unwrap();
    assert_eq!(String::from_utf8(utf8_back).unwrap(), "Sovereign OS");

    // Test UTF-32LE conversion
    let utf32 = engine.iconv_convert(CharacterEncoding::Utf8, CharacterEncoding::Utf32Le, input).unwrap();
    assert_eq!(utf32.len(), input.len() * 4);

    // Test tzdata DST offset lookup
    let transition = engine.resolve_tz_offset("America/New_York", 1715000000).unwrap();
    assert_eq!(transition.abbrev, "EDT");
    assert_eq!(transition.utc_offset_seconds, -14400);

    // Test POSIX TZ parser and zdump
    let posix_tz = engine.parse_posix_tz_string("EST5EDT,M3.2.0,M11.1.0").unwrap();
    assert_eq!(posix_tz.abbrev, "EDT");

    let dump = engine.zdump("America/New_York");
    assert_eq!(dump.len(), 2);

    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("libiconv"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_zlib_and_libffi_full_workflow() {
    let engine = ZlibAndLibffiInspirationsPrEngine::new();
    let data = b"SigmaOS Zlib Deflate Compression Stream";

    let adler = engine.adler32(data);
    let crc = engine.crc32(data);
    assert_ne!(adler, 0);
    assert_ne!(crc, 0);

    let compressed = engine.compress_deflate(data, ZlibCompressionLevel::Best);
    assert_eq!(compressed[0], 0x78);
    assert_eq!(compressed[1], 0x9C);

    // Test Gzip RFC 1952 stream encoder and header validator
    let gzip_stream = engine.compress_gzip(data);
    assert_eq!(gzip_stream[0], 0x1F);
    assert_eq!(gzip_stream[1], 0x8B);

    let (method, crc_val, isize_val) = engine.decompress_gzip_header(&gzip_stream).unwrap();
    assert_eq!(method, 0x08);
    assert_eq!(crc_val, crc);
    assert_eq!(isize_val, data.len() as u32);

    // Test libffi dynamic function descriptor
    let cif = engine.prep_ffi_cif(
        FfiCallingConvention::SysV,
        "uint64",
        &["uint64", "uint64", "uint64"],
    );
    assert_eq!(cif.num_args, 3);
    assert_eq!(cif.integer_register_count, 3);
    assert_eq!(cif.stack_spill_count, 0);

    let result = engine.invoke_ffi_call(&cif, 0x4000, &[100, 200, 300]).unwrap();
    assert_eq!(result, 600);

    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("DEFLATE"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_ncurses_and_readline_full_workflow() {
    let mut engine = NcursesAndReadlineInspirationsPrEngine::new(100, 30);

    engine.mvaddch(5, 10, 'A');
    engine.mvaddch(5, 11, 'B');
    let delta = engine.refresh_screen_delta();
    assert_eq!(delta.len(), 2);
    assert!(delta[0].contains('A'));
    assert!(delta[1].contains('B'));

    // Second refresh with no changes produces zero diff commands
    let delta2 = engine.refresh_screen_delta();
    assert_eq!(delta2.len(), 0);

    // Terminfo capability lookup & mouse tracking
    assert_eq!(engine.lookup_terminfo_cap("clear"), Some(&"\x1B[2J\x1B[H".to_string()));
    engine.set_mouse_mask(0x01);
    engine.simulate_mouse_click(12, 8, 0x01);
    assert_eq!(engine.last_mouse_pos, Some((12, 8, 0x01)));

    // Readline history, tab completion & Vi editing
    engine.readline_add_history("systemctl status");
    engine.readline_add_history("systemctl restart");
    assert_eq!(engine.readline_history.len(), 2);

    let completions = engine.readline_complete("sys", &["systemctl", "syslog", "journalctl"]);
    assert_eq!(completions, vec!["systemctl", "syslog"]);

    engine.vi_mode = true;
    let mut line = "echo test".to_string();
    let res = engine.readline_handle_vi_command('D', &mut line);
    assert_eq!(res, Some("".to_string()));

    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("Ncurses"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_sqlite_and_pcre_full_workflow() {
    let mut engine = SqliteAndPcreInspirationsPrEngine::new();

    let initial_users = engine.query_select_all("users").unwrap();
    assert_eq!(initial_users.len(), 2);

    // PRAGMA and SQL statement parser
    assert_eq!(engine.execute_pragma("journal_mode", "").unwrap(), "WAL");
    assert_eq!(engine.parse_sql_query("SELECT * FROM users").unwrap(), "SELECT_QUERY");

    // Insert new user row
    engine
        .insert_row(
            "users",
            3,
            vec![SqliteValue::Integer(3), SqliteValue::Text("developer".to_string())],
        )
        .unwrap();

    let updated_users = engine.query_select_all("users").unwrap();
    assert_eq!(updated_users.len(), 3);

    // PCRE regex named capture extraction & substitution
    let pattern = "(?P<user>[a-z]+)";
    let log_line = "2026-04-12 10:00:00 user=jules level=info";
    let captures = engine.pcre_match_named(pattern, log_line).unwrap();
    assert_eq!(captures.get("user"), Some(&"jules"));

    let substituted = engine.pcre2_substitute("jules", log_line, "admin");
    assert_eq!(substituted, "2026-04-12 10:00:00 user=admin level=info");

    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("SQLite3"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_libxml2_icu_and_codecs_full_workflow() {
    let engine = Libxml2IcuAndImageCodecsInspirationsPrEngine::new();

    // Test XML well-formedness and XPath evaluation
    assert!(engine.parse_xml_well_formed("<config><setting key=\"a\">val</setting></config>"));
    assert!(!engine.parse_xml_well_formed("<config><setting>val</config></setting>"));

    let xpath_res = engine.eval_xpath_query("<root><setting>val1</setting><setting>val2</setting></root>", "setting");
    assert_eq!(xpath_res, vec!["val1", "val2"]);

    // Test ICU grapheme segmentation & string collation
    let graphemes = engine.icu_grapheme_segment("SigmaOS");
    assert_eq!(graphemes.len(), 7);
    assert_eq!(engine.icu_str_collator_cmp("b", "a", "en"), Ordering::Greater);

    // Test PNG header parsing
    let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 64, 8];
    let png_meta = engine.parse_image_header(&png_header).unwrap();
    assert_eq!(png_meta.format, ImageFormat::Png);
    assert_eq!(png_meta.width, 128);
    assert_eq!(png_meta.height, 64);
    assert_eq!(png_meta.chunk_summary.len(), 3);

    // Test JPEG header parsing
    let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
    let jpeg_meta = engine.parse_image_header(&jpeg_header).unwrap();
    assert_eq!(jpeg_meta.format, ImageFormat::Jpeg);
    assert!(!jpeg_meta.chunk_summary.is_empty());

    // Test TIFF header parsing
    let tiff_header = [0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
    let tiff_meta = engine.parse_image_header(&tiff_header).unwrap();
    assert_eq!(tiff_meta.format, ImageFormat::Tiff);
    assert!(!tiff_meta.chunk_summary.is_empty());

    let pr = engine.generate_pr_proposal();
    assert!(pr.pr_title.contains("Libxml2"));
    assert_eq!(pr.status, "Ready for Merge");
}

#[test]
fn test_master_pr_gateway_full_workflow() {
    let gateway = SovereignLocalizationAndMiscLibrariesParityPrGateway::new();

    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 6);
    assert!(gateway.evaluate_system_readiness());

    for manifest in &proposals {
        assert!(!manifest.pr_title.is_empty());
        assert!(!manifest.target_subsystem.is_empty());
        assert!(!manifest.original_libraries.is_empty());
        assert!(!manifest.features_implemented.is_empty());
        assert_eq!(manifest.status, "Ready for Merge");
    }
}
