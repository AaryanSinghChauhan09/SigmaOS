/// SigmaOS: usr/apps/sigma_db.rs
/// Embedded NoSQL Key-Value store for apps and UI states.
/// Zero-allocation, append-only linear array mapped to memory.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;
type SigmaBool  = bool;

pub const MAX_DB_RECORDS: SigmaUsize = 128;
pub const KEY_MAX_LEN: SigmaUsize = 16;
pub const VAL_MAX_LEN: SigmaUsize = 64;

#[derive(Copy, Clone)]
pub struct DbRecord {
    pub key: [u8; KEY_MAX_LEN],
    pub key_len: SigmaUsize,
    pub value: [u8; VAL_MAX_LEN],
    pub val_len: SigmaUsize,
    pub is_valid: SigmaBool,
}

impl DbRecord {
    pub const fn empty() -> Self {
        DbRecord {
            key: [0; KEY_MAX_LEN], key_len: 0,
            value: [0; VAL_MAX_LEN], val_len: 0,
            is_valid: false,
        }
    }
}

pub struct KeyValueDb {
    pub records: [DbRecord; MAX_DB_RECORDS],
}

static mut LOCAL_DB: KeyValueDb = KeyValueDb {
    records: [DbRecord::empty(); MAX_DB_RECORDS],
};

#[no_mangle]
pub unsafe extern "C" fn db_init() -> SigmaI32 {
    for rec in LOCAL_DB.records.iter_mut() {
        rec.is_valid = false;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn db_put(
    key_ptr: *const u8, k_len: SigmaUsize,
    val_ptr: *const u8, v_len: SigmaUsize
) -> SigmaI32 {
    if key_ptr.is_null() || val_ptr.is_null() { return -1; }
    
    let klen = core::cmp::min(k_len, KEY_MAX_LEN);
    let vlen = core::cmp::min(v_len, VAL_MAX_LEN);
    
    // Update existing key
    for i in 0..MAX_DB_RECORDS {
        if LOCAL_DB.records[i].is_valid && LOCAL_DB.records[i].key_len == klen {
            // Check equality
            let mut match_k = true;
            for j in 0..klen {
                if *key_ptr.add(j) != LOCAL_DB.records[i].key[j] {
                    match_k = false; break;
                }
            }
            if match_k {
                core::ptr::copy_nonoverlapping(val_ptr, LOCAL_DB.records[i].value.as_mut_ptr(), vlen);
                LOCAL_DB.records[i].val_len = vlen;
                return 0;
            }
        }
    }
    
    // Find free slot
    for i in 0..MAX_DB_RECORDS {
        if !LOCAL_DB.records[i].is_valid {
            core::ptr::copy_nonoverlapping(key_ptr, LOCAL_DB.records[i].key.as_mut_ptr(), klen);
            core::ptr::copy_nonoverlapping(val_ptr, LOCAL_DB.records[i].value.as_mut_ptr(), vlen);
            
            LOCAL_DB.records[i].key_len = klen;
            LOCAL_DB.records[i].val_len = vlen;
            LOCAL_DB.records[i].is_valid = true;
            return 0;
        }
    }
    
    -12 // ENOMEM (DB Full)
}

#[no_mangle]
pub unsafe extern "C" fn db_get(
    key_ptr: *const u8, k_len: SigmaUsize,
    out_buf: *mut u8, max_out: SigmaUsize,
    out_len: *mut SigmaUsize
) -> SigmaI32 {
    if key_ptr.is_null() || out_buf.is_null() || out_len.is_null() { return -1; }
    
    let klen = core::cmp::min(k_len, KEY_MAX_LEN);
    
    for i in 0..MAX_DB_RECORDS {
        if LOCAL_DB.records[i].is_valid && LOCAL_DB.records[i].key_len == klen {
            let mut match_k = true;
            for j in 0..klen {
                if *key_ptr.add(j) != LOCAL_DB.records[i].key[j] {
                    match_k = false; break;
                }
            }
            if match_k {
                let copylen = core::cmp::min(max_out, LOCAL_DB.records[i].val_len);
                core::ptr::copy_nonoverlapping(LOCAL_DB.records[i].value.as_ptr(), out_buf, copylen);
                *out_len = copylen;
                return 0;
            }
        }
    }
    -4 // ENOENT
}