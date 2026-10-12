// SPDX-License-Identifier: MIT
// Test suite for Sovereign Localization and Misc Libraries PR Gateway Engine

#[path = "../src/tools/sovereign_localization_and_misc_libraries_pr_suite.rs"]
mod sovereign_localization_and_misc_libraries_pr_suite;

use sovereign_localization_and_misc_libraries_pr_suite::*;

#[test]
fn test_sovereign_localization_and_misc_libraries_full_suite() {
    let gateway = SovereignLocalizationAndMiscLibrariesParityPrGateway::new();

    // 1. Verify PR proposal collection
    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 6);
    assert!(gateway.evaluate_system_readiness());

    // 2. Verify Locales & Gettext Engine
    let gettext_pr = &gateway.gettext_engine;
    assert_eq!(gettext_pr.gettext("Hello, World!"), "Hello, World!");
    let mut fr_gettext = LocalesAndGettextInspirationsPrEngine::new();
    fr_gettext.active_locale = "fr_FR.UTF-8".to_string();
    assert_eq!(fr_gettext.gettext("Hello, World!"), "Bonjour le monde!");
    assert_eq!(fr_gettext.bsd_catgets(1, 1, "default"), "Success");
    assert_eq!(fr_gettext.resolve_effective_locale("LC_MESSAGES"), "en_US.UTF-8");

    // 3. Verify Iconv & Tzdata Engine
    let iconv_pr = &gateway.iconv_engine;
    let input_bytes = b"SovereignOS UTF-8";
    let converted = iconv_pr
        .iconv_convert(CharacterEncoding::Utf8, CharacterEncoding::Utf16Le, input_bytes)
        .expect("Transcoding to UTF-16LE failed");
    assert_eq!(converted.len(), input_bytes.len() * 2);

    let ascii_res = iconv_pr
        .iconv_convert_with_policy(
            CharacterEncoding::Utf8,
            CharacterEncoding::Ascii,
            b"Hello",
            IconvErrorPolicy::Exact,
        )
        .unwrap();
    assert_eq!(ascii_res, b"Hello");

    let tz_offset = iconv_pr
        .resolve_tz_offset("America/New_York", 1715000000)
        .expect("Time zone resolution failed");
    assert_eq!(tz_offset.abbrev, "EDT");

    // 4. Verify Zlib & Libffi Engine
    let zlib_pr = &gateway.zlib_engine;
    let adler = zlib_pr.adler32(b"checksum_data");
    let crc = zlib_pr.crc32(b"checksum_data");
    let crc32c = zlib_pr.crc32c(b"checksum_data");
    assert_ne!(adler, 0);
    assert_ne!(crc, 0);
    assert_ne!(crc32c, 0);

    let compressed = zlib_pr.compress_deflate(b"uncompressed stream", ZlibCompressionLevel::Best);
    assert!(compressed.starts_with(&[0x78, 0x9C]));

    let cif = zlib_pr.prep_ffi_cif(FfiCallingConvention::SysV, "uint64", &["uint64", "uint64"]);
    assert_eq!(cif.abi, FfiCallingConvention::SysV);
    assert_eq!(cif.return_type, "uint64");
    assert_eq!(cif.arg_types, vec!["uint64", "uint64"]);

    let ffi_res = zlib_pr.invoke_ffi_call(&cif, 0x2000, &[100, 200]).unwrap();
    assert_eq!(ffi_res, 300);

    // 5. Verify Ncurses & Readline Engine
    let mut ncurses_pr = NcursesAndReadlineInspirationsPrEngine::new(80, 24);
    assert_eq!(ncurses_pr.edit_mode, ReadlineEditMode::Emacs);
    ncurses_pr.mvaddch(0, 0, 'A');
    ncurses_pr.mvaddch_attr(1, 1, 'B', 1, 0, true);
    let diffs = ncurses_pr.refresh_screen_delta();
    assert_eq!(diffs.len(), 2);

    ncurses_pr.readline_add_history("systemctl status");
    assert_eq!(ncurses_pr.readline_history_search("status"), Some("systemctl status".to_string()));

    // 6. Verify SQLite & PCRE2 Engine
    let mut sqlite_pr = SqliteAndPcreInspirationsPrEngine::new();
    let initial_users = sqlite_pr.query_select_all("users").unwrap();
    assert_eq!(initial_users.len(), 2);

    sqlite_pr
        .insert_row(
            "users",
            3,
            vec![
                SqliteValue::Integer(3),
                SqliteValue::Text("developer".to_string()),
                SqliteValue::Real(3.14),
                SqliteValue::Blob(vec![1, 2, 3]),
                SqliteValue::Null,
            ],
        )
        .unwrap();
    assert_eq!(sqlite_pr.query_select_all("users").unwrap().len(), 3);

    let regex_matches = sqlite_pr
        .pcre_match_named("(?P<user>[a-z]+)", "request user=bob status=ok")
        .unwrap();
    assert_eq!(regex_matches.get("user"), Some(&"bob"));

    // 7. Verify Libxml2, ICU & Codecs Engine
    let codecs_pr = &gateway.codecs_engine;
    let xml_sample = "<config><setting>enabled</setting></config>";
    assert!(codecs_pr.parse_xml_well_formed(xml_sample));
    assert_eq!(codecs_pr.evaluate_xpath(xml_sample, "/config/setting"), vec!["enabled"]);

    let graphemes = codecs_pr.icu_grapheme_segment("ABC");
    assert_eq!(graphemes, vec!["A", "B", "C"]);
    assert_eq!(codecs_pr.icu_str_collator("apple", "banana", "en_US"), std::cmp::Ordering::Less);

    let png_header = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 1, 0, 8,
    ];
    let img_meta = codecs_pr.parse_image_header(&png_header).unwrap();
    assert_eq!(img_meta.format, ImageFormat::Png);
    assert_eq!(img_meta.width, 192);
    assert_eq!(img_meta.height, 256);
    assert_eq!(img_meta.color_depth, 8);
    assert_eq!(img_meta.color_space, "RGBA");

    // 8. Verify PR Markdown Description Generation
    let md = gateway.generate_pr_markdown_description(101).unwrap();
    assert!(md.contains("PR-LOCALIZATION-01"));
    assert!(md.contains("GNU gettext"));
}
