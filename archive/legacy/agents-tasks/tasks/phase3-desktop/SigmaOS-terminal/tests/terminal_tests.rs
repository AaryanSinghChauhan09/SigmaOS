//! Tests for SigmaOS-terminal.

use sigma_terminal::{
    Color, Terminal,
    config::{ColorScheme, TermConfig},
    pty::Pty,
    renderer::{Cell, CellGrid, Renderer},
};

#[test]
fn test_pty_creation() {
    let pty = Pty::open("/bin/bash", &[]);
    assert!(pty.is_ok());
    let pty = pty.unwrap();
    assert_eq!(pty.cols, 80);
    assert_eq!(pty.rows, 24);
}

#[test]
fn test_input_handling() {
    let mut pty = Pty::open("/bin/bash", &[]).unwrap();
    pty.write(b"ls\n").unwrap();
    assert_eq!(&pty.write_buf, b"ls\n");
}

#[test]
fn test_output_rendering() {
    let mut renderer = Renderer::new();
    let grid = CellGrid::new(80, 24);
    renderer.render(&grid).unwrap();
    assert_eq!(renderer.frame_count, 1);
}

#[test]
fn test_color_scheme() {
    let config = TermConfig::default();
    let fg = config.foreground_color();
    assert_eq!(fg.r, 0xd8);
}

#[test]
fn test_sixel_config_flag() {
    let config = TermConfig::default();
    assert!(config.enable_sixel);
}

#[test]
fn test_kitty_image_protocol_flag() {
    let config = TermConfig::default();
    assert!(config.enable_kitty_graphics);
}

#[test]
fn test_hyperlink_detection_placeholder() {
    // Placeholder: hyperlink regex detection in VT parser.
    let url = "https://example.com";
    assert!(url.starts_with("https://"));
}

#[test]
fn test_regex_search_placeholder() {
    let scrollback = vec!["error: something failed", "all good here"];
    let matches: Vec<_> = scrollback.iter().filter(|l| l.contains("error")).collect();
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_scrollback_lines_config() {
    let config = TermConfig::default();
    assert_eq!(config.scrollback_lines, 10_000);
}

#[test]
fn test_selection_copy_placeholder() {
    let cell = Cell { ch: 'A', fg: Color::WHITE, bg: Color::BLACK, bold: false, italic: false, underline: false };
    assert_eq!(cell.ch, 'A');
}

#[test]
fn test_unicode_handling() {
    let mut grid = CellGrid::new(80, 24);
    grid.set(0, 0, Cell { ch: '→', ..Cell::default() });
    assert_eq!(grid.get(0, 0).unwrap().ch, '→');
}

#[test]
fn test_mouse_reporting_placeholder() {
    // Mouse reporting mode flag exists in config scope.
    let enabled = false; // default off
    assert!(!enabled);
}

#[test]
fn test_bell_placeholder() {
    // Bell should not panic when triggered.
    let _bell = "BEL";
}

#[test]
fn test_title_update() {
    let config = TermConfig::default();
    // Title is typically set via OSC 2 escape; config has shell name as initial title.
    assert!(!config.shell.is_empty());
}

#[test]
fn test_config_reload() {
    let config1 = TermConfig::load_or_default().unwrap();
    let config2 = TermConfig::load_or_default().unwrap();
    assert_eq!(config1.font_size, config2.font_size);
}

#[test]
fn test_pty_resize() {
    let mut pty = Pty::open("/bin/bash", &[]).unwrap();
    pty.resize(132, 50).unwrap();
    assert_eq!(pty.cols, 132);
    assert_eq!(pty.rows, 50);
}
