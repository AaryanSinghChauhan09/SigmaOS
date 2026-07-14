use std::path::PathBuf;

/// Packages binaries and static files into a cpio archive for the Linux kernel
/// to mount as the initial root filesystem during boot.
pub struct InitramfsBuilder {
    files: Vec<PathBuf>,
}

impl Default for InitramfsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl InitramfsBuilder {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    /// Add a binary or file to the initramfs payload.
    pub fn add_file(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    /// Build the `cpio.gz` archive.
    pub fn build(&self, output_path: &str) -> Result<(), String> {
        // In a real implementation, this would format the added files as a CPIO archive,
        // gzip compress it, and write it to `output_path`.
        println!("Building initramfs to {} containing {} files.", output_path, self.files.len());
        Ok(())
    }
}
