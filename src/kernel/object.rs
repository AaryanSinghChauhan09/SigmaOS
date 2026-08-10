// SigmaOS Windows/Linux/BSD-Inspired Advanced Object Manager (Obp)
// Implements advanced Object Manager namespaces, symbolic link translation,
// driver entry contexts, dynamic unloading, and Non-Paged Pool memory tracking.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::klib::HashMap;