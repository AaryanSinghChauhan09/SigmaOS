// Integration Test Suite for Sovereign Audio & Video Advancements (V90 PR Suite)

#[path = "../src/media/sovereign_audio_video_advancements_v90_pr.rs"]
mod sovereign_audio_video_advancements_v90_pr;

use sovereign_audio_video_advancements_v90_pr::*;

#[test]
fn test_integration_audio_video_v90_suite() {
    let suite = SovereignAudioVideoAdvancementsSuiteV90::new();
    assert_eq!(suite.alsa.soundcard_id, "hw:0");
    assert_eq!(suite.oss4.dsp_device_path, "/dev/dsp0");
    assert_eq!(suite.pulseaudio.protocol_version, 35);
    assert_eq!(suite.pipewire.node_id, 1);
    assert_eq!(suite.jack.client_name, "SigmaJackV90");
    assert_eq!(suite.sndio.device_spec, "snd/0");
    assert_eq!(suite.gstreamer.rtsp_port, 8554);
    assert_eq!(suite.ffmpeg.hwaccel_backend, "VAAPI");
    assert_eq!(suite.v4l2.video_dev_path, "/dev/video0");
    assert_eq!(suite.vlc.ass_subtitle_font, "DejaVu Sans");
    assert_eq!(suite.mpv.vo_backend, "gpu-next");
}

#[test]
fn test_integration_v90_gateway_and_submissions() {
    let mut gateway = SovereignAudioVideoV90PrGatewayEngine::new();
    assert_eq!(gateway.submissions.len(), 11);
    assert_eq!(gateway.gateway_name, "SigmaOS-AudioVideo-V90-Gateway");

    let audit_score = gateway.run_20_point_audit();
    assert_eq!(audit_score, 100);

    // Verify all subsystems have valid PQC signatures and merged status
    for pr in gateway.submissions.values() {
        assert_eq!(pr.status, SovereignAvPrStatusV90::Merged);
        assert_eq!(pr.pqc_signature, "pqc_dilithium5_valid_v90");
        assert!(pr.pr_id >= 9001);
    }

    // Submit an extra custom proposal
    let new_pr_id = gateway.submit_pr(
        "Vulkan Video Next-Gen Accelerated Decode V90",
        "FFmpeg",
        "pqc_dilithium5_ext_v90"
    );
    assert_eq!(new_pr_id, 9012);
    let custom_pr = gateway.submissions.get(&new_pr_id).expect("Custom PR must exist");
    assert_eq!(custom_pr.status, SovereignAvPrStatusV90::Proposed);
    assert_eq!(custom_pr.subsystem, "FFmpeg");
}

#[test]
fn test_integration_sub_engines_functionality() {
    // ALSA UCM2 & PTP
    let mut alsa = SovereignAlsaUcm2EngineV90::new("hw:1");
    assert!(alsa.set_verb("HdmiOutput").is_ok());
    assert_eq!(alsa.active_verb, "HdmiOutput");
    alsa.set_softvol_gain_db(-6.0);
    assert_eq!(alsa.softvol_db_gain, -6.0);
    assert_eq!(alsa.sync_ptp_clock(5_000_000_000), 5_000_000_000);

    // OSSv4
    let mut oss = SovereignOss4EngineV90::new("/dev/dsp1");
    let count = oss.parse_sndstat("pcm0: <Realtek ALC892> (play/rec)\npcm1: <Intel Display Audio> (play)");
    assert_eq!(count, 2);
    oss.configure_dsp_format("AFMT_S32_LE", 96000, 2048);
    assert_eq!(oss.sample_format, "AFMT_S32_LE");

    // PulseAudio
    let mut pa = SovereignPulseAudioEngineV90::new();
    pa.load_module("module-null-sink", "sink_name=Dummy");
    assert!(pa.active_modules.contains_key("module-null-sink"));
    assert_eq!(pa.compute_ducked_volume(80, "music"), 20);
    assert_eq!(pa.compute_ducked_volume(80, "phone"), 80);

    // PipeWire
    let mut pw = SovereignPipeWireEngineV90::new(10);
    pw.set_quantum_rate(192000, 1024);
    assert_eq!(pw.sample_rate_hz, 192000);
    assert!(pw.set_eq_band_gain(5, -3.5).is_ok());
    pw.attach_dmabuf_handle(99);
    assert_eq!(pw.dmabuf_handles, vec![99]);
    let pod_bytes = pw.serialize_spa_pod_bytes();
    assert_eq!(&pod_bytes[0..5], b"SPA90");

    // JACK
    let mut jack = SovereignJackEngineV90::new("SigmaDAW");
    jack.connect_ports("SigmaDAW:out_1", "system:playback_1");
    assert_eq!(jack.connected_port_pairs.len(), 1);
    assert!(jack.push_sysex_msg(&[0xF0, 0x7E, 0x7F, 0x09, 0x01, 0xF7]).is_ok());
    assert!(jack.save_nsm_session("/var/run/nsm/session1"));

    // sndio
    let mut sndio = SovereignSndioEngineV90::new("snd/1");
    sndio.configure_env_devices("snd/1", "rmidi/1");
    assert_eq!(sndio.audio_device_env, "snd/1");
    sndio.set_sioctl_level("output.level", 125);
    assert_eq!(sndio.sioctl_controls["output.level"], 125);

    // GStreamer
    let mut gst = SovereignGStreamerEngineV90::new();
    assert_eq!(gst.attach_pad_probe(), 1);
    gst.configure_splitmux(300);
    gst.start_rtsp_server(8555);
    assert!(gst.build_gst_launch_command().contains("splitmuxsink"));

    // FFmpeg
    let mut ff = SovereignFfmpegEngineV90::new("QSV");
    assert_eq!(ff.allocate_drm_prime_pool(4), 4);
    ff.add_filter("hwupload");
    assert_eq!(ff.filter_chain_specs.len(), 3);
    assert_eq!(ff.read_custom_avio_bytes(1024), 1024);

    // V4L2
    let mut v4l2 = SovereignV4l2EngineV90::new("/dev/video2");
    v4l2.configure_multi_planar(&[1920 * 1080, 1920 * 1080 / 2]);
    assert_eq!(v4l2.planes_count, 2);
    v4l2.set_ptz_controls(20, -10, 200);
    assert_eq!(v4l2.ptz_pan_tilt_zoom, (20, -10, 200));

    // VLC
    let mut vlc = SovereignVlcEngineV90::new();
    vlc.set_ass_subtitle_font("Noto Sans");
    vlc.adjust_subtitle_delay(250);
    assert_eq!(vlc.subtitle_sync_offset_ms, 250);
    vlc.add_discovered_upnp_mrl("upnp://192.168.1.50/media.mkv");
    vlc.enqueue_playlist_item("file:///movies/stream.mp4");
    assert_eq!(vlc.playlist_urls.len(), 1);

    // mpv
    let mut mpv = SovereignMpvEngineV90::new("/tmp/sigma_mpv.sock");
    mpv.append_gpu_shader("ravu-r4.glsl");
    assert_eq!(mpv.active_glsl_shaders.len(), 3);
    mpv.register_property_observer(1, "time-pos");
    mpv.show_osd_message("Buffering...");
    assert_eq!(mpv.osd_notification_text, "Buffering...");
    assert!(mpv.format_jsonrpc_command("speed", "1.25").contains("speed"));
}
