// Integration Test Suite for Sovereign Audio and Video Advancements V94 PR
// Verifying all 11 sub-engines, master suite, and PR gateway

#[path = "../src/media/sovereign_audio_video_advancements_v94_pr.rs"]
mod sovereign_audio_video_advancements_v94_pr;

use sovereign_audio_video_advancements_v94_pr::*;

#[test]
fn test_alsa_advancements_integration_v94() {
    let mut alsa = SovereignAlsaUcm2EngineV94::new("hw:1");
    assert_eq!(alsa.soundcard_id, "hw:1");
    assert_eq!(alsa.active_verb, "HiFi");

    assert!(alsa.set_verb("ProAudio").is_ok());
    assert_eq!(alsa.active_verb, "ProAudio");

    alsa.set_softvol_gain_db(-6.0);
    assert_eq!(alsa.softvol_db_gain, -6.0);

    alsa.advance_ring_buffer(2048, 1024);
    assert_eq!(alsa.pcm_state, AlsaPcmStateV94::Running);

    alsa.advance_ring_buffer(8192, 0);
    assert_eq!(alsa.pcm_state, AlsaPcmStateV94::Xrun);
    assert!(alsa.recover_xrun());
    assert_eq!(alsa.pcm_state, AlsaPcmStateV94::Prepared);

    alsa.set_channel_map(&["FL", "FR", "FC", "LFE", "BL", "BR"]);
    assert_eq!(alsa.channel_map.len(), 6);

    assert_eq!(alsa.sync_ptp_clock(5_000_000_000), 5_000_000_000);
    assert!(alsa.toggle_snd_aloop(true));
}

#[test]
fn test_oss4_advancements_integration_v94() {
    let mut oss = SovereignOss4EngineV94::new("/dev/dsp1");
    let count = oss.parse_sndstat("Installed devices:\npcm0: <Realtek ALC892>\npcm1: <USB Audio DAC>\ndsp1: <OSS Direct>");
    assert_eq!(count, 3);

    oss.configure_dsp_format("AFMT_S32_LE", 192000, 8192);
    assert_eq!(oss.sample_rate_hz, 192000);
    assert_eq!(oss.fragment_size_bytes, 8192);

    assert_eq!(oss.allocate_virtual_channel("vchan2"), 3);

    let (left, right) = oss.calculate_peak_levels(&[500000, -1000000, 2000000]);
    assert!(left < 0.0);
    assert_eq!(left, right);

    oss.set_sysctl_latency(0);
    assert_eq!(oss.sysctl_latency_profile, "dev.pcm.0.latency=0");
}

#[test]
fn test_pulseaudio_advancements_integration_v94() {
    let mut pa = SovereignPulseAudioEngineV94::new();
    assert_eq!(pa.protocol_version, 35);

    pa.load_module("module-combine-sink", "slaves=sink1,sink2");
    assert!(pa.active_modules.contains_key("module-combine-sink"));

    let ducked = pa.compute_ducked_volume(80, "video");
    assert_eq!(ducked, 20);

    let unaltered = pa.compute_ducked_volume(80, "phone");
    assert_eq!(unaltered, 80);

    pa.toggle_webrtc_aec(false);
    assert!(!pa.is_webrtc_aec_active);
    pa.toggle_webrtc_aec(true);
    assert!(pa.is_webrtc_aec_active);

    assert_eq!(pa.synchronize_ntp_clock(9_999_999), 9_999_999);

    pa.set_rtp_multicast("239.255.0.1", 5004);
    assert_eq!(pa.rtp_multicast_group, "239.255.0.1");
    assert_eq!(pa.rtp_port, 5004);
}

#[test]
fn test_pipewire_advancements_integration_v94() {
    let mut pw = SovereignPipeWireEngineV94::new(100);
    assert_eq!(pw.node_id, 100);

    pw.set_quantum_rate(44100, 128);
    assert_eq!(pw.sample_rate_hz, 44100);
    assert_eq!(pw.quantum_frames, 128);

    pw.add_wireplumber_rule("50-alsa-config.lua");
    assert!(pw.wireplumber_rules.contains(&"50-alsa-config.lua".to_string()));

    assert!(pw.set_eq_band_gain(5, 6.0).is_ok());
    assert_eq!(pw.filter_chain_eq_bands_db[5], 6.0);
    assert!(pw.set_eq_band_gain(99, 1.0).is_err());

    pw.attach_dmabuf_handle(55);
    assert_eq!(pw.dmabuf_handles, vec![55]);

    let pod = pw.serialize_spa_pod_bytes();
    assert_eq!(&pod[0..5], b"SPA94");

    pw.configure_hrtf_profile("CIPIC_Subject_003");
    assert_eq!(pw.hrtf_profile, "CIPIC_Subject_003");
}

#[test]
fn test_jack_advancements_integration_v94() {
    let mut jack = SovereignJackEngineV94::new("SigmaJackV94Client");
    assert_eq!(jack.client_name, "SigmaJackV94Client");

    jack.connect_ports("system:playback_1", "SigmaJack:in_1");
    assert_eq!(jack.connected_port_pairs.len(), 1);

    let sysex_valid = vec![0xF0, 0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7];
    assert!(jack.push_sysex_msg(&sysex_valid).is_ok());
    assert_eq!(jack.sysex_queue.len(), 1);

    let sysex_invalid = vec![0x00, 0x41, 0xF7];
    assert!(jack.push_sysex_msg(&sysex_invalid).is_err());

    assert!(jack.save_nsm_session("/var/run/nsm/session.nsm"));
    assert_eq!(jack.nsm_session_file, "/var/run/nsm/session.nsm");

    let latency = jack.calculate_latency_us();
    assert!(latency > 0.0);

    jack.toggle_rolling(true);
    assert!(jack.is_rolling);
}

#[test]
fn test_sndio_advancements_integration_v94() {
    let mut sndio = SovereignSndioEngineV94::new("snd/1");
    assert_eq!(sndio.device_spec, "snd/1");

    sndio.set_sioctl_level("output.level", 125);
    assert_eq!(*sndio.sioctl_controls.get("output.level").unwrap(), 125);

    sndio.set_sioctl_level("input.level", 200); // clamped to 127
    assert_eq!(*sndio.sioctl_controls.get("input.level").unwrap(), 127);

    assert_eq!(sndio.advance_mtc_clock(120), 120);
    assert_eq!(sndio.advance_mtc_clock(60), 180);

    sndio.configure_env_devices("snd/2", "rmidi/1");
    assert_eq!(sndio.audio_device_env, "snd/2");
    assert_eq!(sndio.midi_device_env, "rmidi/1");
}

#[test]
fn test_gstreamer_advancements_integration_v94() {
    let mut gst = SovereignGStreamerEngineV94::new();
    assert_eq!(gst.attach_pad_probe(), 1);
    assert_eq!(gst.attach_pad_probe(), 2);

    gst.configure_splitmux(120);
    assert_eq!(gst.split_mux_segment_sec, 120);

    gst.start_rtsp_server(8555);
    assert!(gst.is_rtsp_server_active);
    assert_eq!(gst.rtsp_port, 8555);

    let cmd = gst.build_gst_launch_command();
    assert!(cmd.contains("v4l2src"));
    assert!(cmd.contains("splitmuxsink"));
    assert!(cmd.contains("max-size-time=120000000000"));
}

#[test]
fn test_ffmpeg_advancements_integration_v94() {
    let mut ff = SovereignFfmpegEngineV94::new("QSV");
    assert_eq!(ff.hwaccel_backend, "QSV");

    assert_eq!(ff.allocate_drm_prime_pool(4), 4);
    assert_eq!(ff.drm_prime_surface_handles.len(), 4);

    ff.add_filter("eq=contrast=1.2:brightness=0.05");
    assert_eq!(ff.filter_chain_specs.len(), 3);

    let bytes = ff.read_custom_avio_bytes(4000);
    assert_eq!(bytes, 4000);
    assert_eq!(ff.total_bytes_processed, 4000);
    assert!(ff.hdr10_plus_metadata_enabled);

    ff.toggle_dolby_vision(true);
    assert!(ff.dolby_vision_profile_rpu);
}

#[test]
fn test_v4l2_advancements_integration_v94() {
    let mut v4l2 = SovereignV4l2EngineV94::new("/dev/video2");
    assert_eq!(v4l2.video_dev_path, "/dev/video2");

    v4l2.configure_multi_planar(&[1920 * 1080, 1920 * 540]);
    assert_eq!(v4l2.planes_count, 2);

    v4l2.set_ptz_controls(20, -10, 200);
    assert_eq!(v4l2.ptz_pan_tilt_zoom, (20, -10, 200));

    v4l2.setup_media_link("sensor0:0", "csi2:0");
    assert_eq!(v4l2.subdev_pad_links.len(), 1);
    assert_eq!(v4l2.auto_exposure_kelvin, 4600);

    v4l2.set_framerate(144);
    assert_eq!(v4l2.dynamic_framerate_fps, 144);
}

#[test]
fn test_vlc_advancements_integration_v94() {
    let mut vlc = SovereignVlcEngineV94::new();
    assert!(vlc.libvlc_version.contains("v94"));

    vlc.set_ass_subtitle_font("Noto Sans");
    assert_eq!(vlc.ass_subtitle_font, "Noto Sans");

    vlc.adjust_subtitle_delay(-250);
    assert_eq!(vlc.subtitle_sync_offset_ms, -250);

    vlc.add_discovered_upnp_mrl("upnp://192.168.1.100:5000/media/1");
    assert_eq!(vlc.discovered_upnp_mrls.len(), 1);

    vlc.enqueue_playlist_item("file:///home/user/video.mkv");
    assert_eq!(vlc.playlist_urls.len(), 1);

    vlc.set_chapter(5);
    assert_eq!(vlc.current_chapter, 5);
}

#[test]
fn test_mpv_advancements_integration_v94() {
    let mut mpv = SovereignMpvEngineV94::new("/tmp/mpv_test_socket");
    assert_eq!(mpv.vo_backend, "gpu-next");

    mpv.append_gpu_shader("CAS-scaled.glsl");
    assert_eq!(mpv.active_glsl_shaders.len(), 3);

    mpv.register_property_observer(1, "time-pos");
    assert_eq!(mpv.property_observers.get(&1).unwrap(), "time-pos");

    mpv.show_osd_message("Buffering 99%...");
    assert_eq!(mpv.osd_notification_text, "Buffering 99%...");

    let json = mpv.format_jsonrpc_command("speed", "1.25");
    assert!(json.contains("speed"));
    assert!(json.contains("1.25"));

    mpv.add_lua_hook("event_end_file");
    assert_eq!(mpv.lua_hooks.len(), 1);
}

#[test]
fn test_master_suite_and_pr_gateway_integration_v94() {
    let suite = SovereignAudioVideoAdvancementsSuiteV94::new();
    assert_eq!(suite.alsa.soundcard_id, "hw:0");
    assert_eq!(suite.oss4.dsp_device_path, "/dev/dsp0");
    assert_eq!(suite.pulseaudio.protocol_version, 35);
    assert_eq!(suite.pipewire.node_id, 1);
    assert_eq!(suite.jack.client_name, "SigmaJackV94");
    assert_eq!(suite.sndio.device_spec, "snd/0");
    assert_eq!(suite.gstreamer.rtsp_port, 8554);
    assert_eq!(suite.ffmpeg.hwaccel_backend, "VAAPI");
    assert_eq!(suite.v4l2.video_dev_path, "/dev/video0");
    assert!(suite.vlc.libvlc_version.contains("v94"));
    assert_eq!(suite.mpv.vo_backend, "gpu-next");

    let mut gateway = SovereignAudioVideoV94PrGatewayEngine::new();
    assert_eq!(gateway.submissions.len(), 11);

    let custom_pr = gateway.submit_pr(
        "Low-Latency Opus RTP Packetizer with PTP Timestamping",
        "PulseAudio",
        "pqc_dilithium5_custom_sig"
    );
    assert_eq!(custom_pr, 12);
    assert_eq!(gateway.submissions.len(), 12);

    assert_eq!(gateway.run_20_point_audit(), 100);
}
