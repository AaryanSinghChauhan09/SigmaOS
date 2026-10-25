// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - DOS INT 21h Service Dispatcher
// Principles: INTERFACE SEGREGATION, COMMAND-QUERY SEPARATION, ENCAPSULATION

use alloc::{string::String, vec::Vec};

/// DOS service call variants (INT 21h AH register)
#[derive(Debug, Clone)]
pub enum DosCall {
    PrintString(String),
    PrintChar(u8),
    GetChar,
    OpenFile {
        name: String,
        mode: u8,
    },
    ReadFile {
        handle: u16,
        count: u16,
    },
    WriteFile {
        handle: u16,
        data: Vec<u8>,
    },
    CloseFile(u16),
    Terminate(u8),
    GetVersion,
    AllocMem(u16),
    FreeMem(u16),
    CreateFile {
        name: String,
        attr: u8,
    },
    DeleteFile(String),
    RenameFile {
        old: String,
        new: String,
    },
    GetCurrentDir,
    ChangeDir(String),
    GetDta,
    SetDta(u32),
    FindFirst {
        pattern: String,
        attr: u8,
    },
    FindNext,
    GetSetTime {
        set: bool,
        hour: u8,
        min: u8,
        sec: u8,
    },
    GetSetDate {
        set: bool,
        year: u16,
        month: u8,
        day: u8,
    },
}

/// Result of a DOS service call
#[derive(Debug, Clone)]
pub enum DosResult {
    Ok,
    Char(u8),
    Handle(u16),
    Data(Vec<u8>),
    Version { major: u8, minor: u8 },
    Segment(u16),
    String(String),
    Error { code: u16, message: String },
}

/// Next available file handle counter (simulated)
static mut NEXT_HANDLE: u16 = 3; // 0=stdin, 1=stdout, 2=stderr

/// INT 21h Dispatcher — routes DOS service calls to appropriate handlers
pub struct Int21Dispatcher {
    pub output_buffer: Vec<u8>,
    pub input_buffer: Vec<u8>,
    pub open_handles: alloc::collections::BTreeMap<u16, String>,
    pub allocated_segments: alloc::collections::BTreeMap<u16, u16>,
    pub current_dir: String,
}

impl Int21Dispatcher {
    pub fn new() -> Self {
        Self {
            output_buffer: Vec::new(),
            input_buffer: Vec::new(),
            open_handles: alloc::collections::BTreeMap::new(),
            allocated_segments: alloc::collections::BTreeMap::new(),
            current_dir: String::from("C:\\"),
        }
    }

    pub fn dispatch(&mut self, call: DosCall) -> DosResult {
        match call {
            DosCall::PrintString(s) => {
                self.output_buffer.extend_from_slice(s.as_bytes());
                DosResult::Ok
            }
            DosCall::PrintChar(c) => {
                self.output_buffer.push(c);
                DosResult::Ok
            }
            DosCall::GetChar => {
                let c = self.input_buffer.pop().unwrap_or(b'\n');
                DosResult::Char(c)
            }
            DosCall::GetVersion => DosResult::Version {
                major: 6,
                minor: 22,
            },
            DosCall::OpenFile { name, mode: _ } => {
                let handle = self.alloc_handle();
                self.open_handles.insert(handle, name);
                DosResult::Handle(handle)
            }
            DosCall::CreateFile { name, attr: _ } => {
                let handle = self.alloc_handle();
                self.open_handles.insert(handle, name);
                DosResult::Handle(handle)
            }
            DosCall::CloseFile(h) => {
                self.open_handles.remove(&h);
                DosResult::Ok
            }
            DosCall::ReadFile { handle: _, count } => {
                let data: Vec<u8> = (0..count).map(|i| (i % 256) as u8).collect();
                DosResult::Data(data)
            }
            DosCall::WriteFile { handle: _, data } => {
                self.output_buffer.extend_from_slice(&data);
                DosResult::Ok
            }
            DosCall::DeleteFile(_) | DosCall::RenameFile { .. } => DosResult::Ok,
            DosCall::AllocMem(paragraphs) => {
                let seg = 0x1000u16.wrapping_add(self.allocated_segments.len() as u16 * 0x100);
                self.allocated_segments.insert(seg, paragraphs);
                DosResult::Segment(seg)
            }
            DosCall::FreeMem(seg) => {
                self.allocated_segments.remove(&seg);
                DosResult::Ok
            }
            DosCall::GetCurrentDir => DosResult::String(self.current_dir.clone()),
            DosCall::ChangeDir(path) => {
                self.current_dir = path;
                DosResult::Ok
            }
            DosCall::Terminate(code) => DosResult::Error {
                code: code as u16,
                message: String::from("process terminated"),
            },
            DosCall::GetSetTime {
                set,
                hour,
                min,
                sec,
            } => DosResult::Ok,
            DosCall::GetSetDate { .. } => DosResult::Ok,
            _ => DosResult::Ok,
        }
    }

    fn alloc_handle(&mut self) -> u16 {
        // Simple incrementing handle allocator
        let h = self.open_handles.len() as u16 + 3;
        h
    }

    pub fn drain_output(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.output_buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_string() {
        let mut d = Int21Dispatcher::new();
        d.dispatch(DosCall::PrintString(String::from("HELLO")));
        assert_eq!(d.output_buffer, b"HELLO");
    }

    #[test]
    fn test_get_version() {
        let mut d = Int21Dispatcher::new();
        let r = d.dispatch(DosCall::GetVersion);
        assert!(matches!(
            r,
            DosResult::Version {
                major: 6,
                minor: 22
            }
        ));
    }

    #[test]
    fn test_open_close_file() {
        let mut d = Int21Dispatcher::new();
        let r = d.dispatch(DosCall::OpenFile {
            name: String::from("TEST.TXT"),
            mode: 0,
        });
        if let DosResult::Handle(h) = r {
            assert!(h >= 3);
            d.dispatch(DosCall::CloseFile(h));
        } else {
            panic!("expected handle");
        }
    }

    #[test]
    fn test_alloc_free_mem() {
        let mut d = Int21Dispatcher::new();
        let r = d.dispatch(DosCall::AllocMem(64));
        if let DosResult::Segment(seg) = r {
            assert!(seg > 0);
            d.dispatch(DosCall::FreeMem(seg));
            assert!(!d.allocated_segments.contains_key(&seg));
        } else {
            panic!("expected segment");
        }
    }
}
