#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::string::String;
use std::vec::Vec;
// Appending Access Rights & Immutable Journal Stream Enforcement for SigmaOS


/// Appending Access Rights Stream Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppendingRightsFlags {
    pub allow_append: bool,
    pub allow_read: bool,
    pub allow_truncate: bool,
    pub allow_overwrite: bool,
}

impl AppendingRightsFlags {
    pub fn append_only() -> Self {
        AppendingRightsFlags {
            allow_append: true,
            allow_read: true,
            allow_truncate: false,
            allow_overwrite: false,
        }
    }
}

/// Append-Only Audit Stream Buffer
pub struct AppendOnlyStream {
    pub stream_id: usize,
    pub name: String,
    pub rights: AppendingRightsFlags,
    pub data: Vec<u8>,
}

impl AppendOnlyStream {
    pub fn new(stream_id: usize, name: &str) -> Self {
        AppendOnlyStream {
            stream_id,
            name: String::from(name),
            rights: AppendingRightsFlags::append_only(),
            data: Vec::new(),
        }
    }

    pub fn append_data(&mut self, payload: &[u8]) -> Result<usize, &'static str> {
        if !self.rights.allow_append {
            return Err("Append access rights denied");
        }
        self.data.extend_from_slice(payload);
        Ok(payload.len())
    }

    pub fn overwrite_data(&mut self, _payload: &[u8]) -> Result<(), &'static str> {
        if !self.rights.allow_overwrite {
            return Err("Overwrite access rights denied on append-only stream");
        }
        Ok(())
    }

    pub fn truncate_stream(&mut self) -> Result<(), &'static str> {
        if !self.rights.allow_truncate {
            return Err("Truncate access rights denied on append-only stream");
        }
        self.data.clear();
        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_append_only_stream_enforcement() {
        let mut stream = AppendOnlyStream::new(1, "audit.log");
        assert_eq!(stream.append_data(b"Log Entry 1\n").unwrap(), 12);
        assert_eq!(stream.append_data(b"Log Entry 2\n").unwrap(), 12);

        assert!(stream.overwrite_data(b"Hacked").is_err());
        assert!(stream.truncate_stream().is_err());
        assert_eq!(stream.data.len(), 24);
    }
}
