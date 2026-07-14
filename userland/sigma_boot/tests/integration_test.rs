use std::path::PathBuf;
use sigma_boot::InitramfsBuilder;

#[test]
fn test_sigma_boot_initramfs_generation() {
    let mut builder = InitramfsBuilder::new();
    builder.add_file(PathBuf::from("/bin/sigma_init"));
    builder.add_file(PathBuf::from("/etc/sigma/config.toml"));

    // In a real scenario, this would write a CPIO archive and we'd assert its format.
    let result = builder.build("/tmp/sigma_initramfs.cpio.gz");
    assert!(result.is_ok());
}
