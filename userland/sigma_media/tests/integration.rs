use sigma_media::{SigmaTerminal, SigmaShell, SigmaMediaEngine};

#[test]
fn test_terminal_dimensions() {
    let mut term = SigmaTerminal::new();
    assert_eq!(term.rows, 24);
    assert_eq!(term.cols, 80);

    term.resize(120, 40);
    assert_eq!(term.rows, 40);
    assert_eq!(term.cols, 120);
    assert!(term.render_cell('A', 0, 0).is_ok());
}

#[test]
fn test_shell_autocompletion() {
    let shell = SigmaShell::new();
    let res = shell.complete_cmd("sigma-co");
    assert_eq!(res, vec!["sigma-container", "sigma-compose"]);
}

#[test]
fn test_media_pipeline() {
    let engine = SigmaMediaEngine::new();
    let audio = engine.open_audio_stream(2, 48000).unwrap();
    assert_eq!(audio.sample_rate, 48000);

    assert!(engine.open_audio_stream(0, 0).is_err());
    
    let video = engine.decode_video_frame(1920, 1080).unwrap();
    assert_eq!(video.width, 1920);
}
