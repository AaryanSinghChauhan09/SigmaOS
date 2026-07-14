/// Sovereign Inode — absorbs XFS, ext4, and Fuchsia object storage concepts.
#[derive(Debug, Clone)]
pub enum InodeKind {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub id: u64,
    pub kind: InodeKind,
    pub size: u64,
}
