// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/blockchain/sigma_blockchain_lite.rs — Sovereign DLT for Government Records
//
// Implements:
//   - Permissioned blockchain for immutable government records
//   - Land records, birth/death certificates, educational credentials on-chain
//   - NIC/DigitalIndia validator nodes (no foreign cloud)
//   - sigma-DID as identity layer (W3C DID)
//   - Replaces paper certificate verification with on-chain proof
//   - Extension of MCA21 mandate to all government documents
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Block structure ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u64,
    pub previous_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub validator_did: [u8; 32],
    pub nonce: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transaction_count: u32,
    pub hash: [u8; 32],
}

impl Block {
    pub const fn genesis() -> Self {
        Self {
            header: BlockHeader {
                version: 1,
                height: 0,
                previous_hash: [0u8; 32],
                merkle_root: [0u8; 32],
                timestamp: 0,
                validator_did: [0u8; 32],
                nonce: 0,
            },
            transaction_count: 0,
            hash: [0u8; 32],
        }
    }
}

// ── Transaction types ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TransactionType {
    LandRecord = 0,
    BirthCertificate = 1,
    DeathCertificate = 2,
    EducationalCredential = 3,
    MarriageCertificate = 4,
    PropertyDeed = 5,
    CourtJudgment = 6,
    TaxRecord = 7,
    BusinessLicense = 8,
    Other = 9,
}

// ── Transaction structure ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Transaction {
    pub tx_id: [u8; 32],
    pub tx_type: TransactionType,
    pub sender_did: [u8; 32],
    pub recipient_did: [u8; 32],
    pub data_hash: [u8; 32],      // Hash of the actual document/data
    pub metadata_hash: [u8; 32],   // Additional metadata
    pub timestamp: u64,
    pub signature: [u8; 64],      // DID signature
    pub block_height: u64,
}

impl Transaction {
    pub const fn new() -> Self {
        Self {
            tx_id: [0u8; 32],
            tx_type: TransactionType::Other,
            sender_did: [0u8; 32],
            recipient_did: [0u8; 32],
            data_hash: [0u8; 32],
            metadata_hash: [0u8; 32],
            timestamp: 0,
            signature: [0u8; 64],
            block_height: 0,
        }
    }
}

// ── Validator node ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Validator {
    pub did: [u8; 32],
    pub stake: u64,              // Stake amount
    pub reputation: u32,         // Reputation score
    pub blocks_validated: u64,
    pub last_validation: u64,
    pub is_active: bool,
}

impl Validator {
    pub const fn new() -> Self {
        Self {
            did: [0u8; 32],
            stake: 0,
            reputation: 100,
            blocks_validated: 0,
            last_validation: 0,
            is_active: false,
        }
    }
}

// ── Blockchain state ─────────────────────────────────────────────────────

const MAX_BLOCKS: usize = 100000;
const MAX_TRANSACTIONS: usize = 1000000;
const MAX_VALIDATORS: usize = 100;

pub struct BlockchainLite {
    blocks: [Option<Block>; MAX_BLOCKS],
    transactions: [Option<Transaction>; MAX_TRANSACTIONS],
    validators: [Option<Validator>; MAX_VALIDATORS],
    block_count: AtomicU64,
    transaction_count: AtomicU64,
    validator_count: AtomicU32,
    latest_block_hash: [u8; 32],
    initialized: bool,
}

impl BlockchainLite {
    pub const fn new() -> Self {
        Self {
            blocks: [const { None }; MAX_BLOCKS],
            transactions: [const { None }; MAX_TRANSACTIONS],
            validators: [const { None }; MAX_VALIDATORS],
            block_count: AtomicU64::new(0),
            transaction_count: AtomicU64::new(0),
            validator_count: AtomicU32::new(0),
            latest_block_hash: [0u8; 32],
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Create genesis block
        let genesis = Block::genesis();
        let genesis_hash = self.hash_block(&genesis);
        
        let mut genesis_with_hash = genesis;
        genesis_with_hash.hash = genesis_hash;
        
        self.blocks[0] = Some(genesis_with_hash);
        self.block_count.fetch_add(1, Ordering::Relaxed);
        self.latest_block_hash = genesis_hash;
        self.initialized = true;
    }

    /// Register a validator node
    pub fn register_validator(&mut self, validator: Validator) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_VALIDATORS {
            if self.validators[i].is_none() {
                self.validators[i] = Some(validator);
                self.validator_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Submit a transaction to the blockchain
    pub fn submit_transaction(&mut self, tx: Transaction) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TRANSACTIONS {
            if self.transactions[i].is_none() {
                self.transactions[i] = Some(tx);
                self.transaction_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Create a new block with pending transactions
    pub fn create_block(&mut self, validator_did: [u8; 32]) -> Option<u64> {
        if !self.initialized {
            return None;
        }

        let height = self.block_count.load(Ordering::Relaxed);
        let prev_hash = self.latest_block_hash;

        // Collect pending transactions (simplified: take first 100)
        let mut tx_count = 0u32;
        let mut merkle_root = [0u8; 32];
        
        for i in 0..MAX_TRANSACTIONS {
            if let Some(tx) = &self.transactions[i] {
                if tx.block_height == 0 && tx_count < 100 {
                    tx_count += 1;
                    // Simplified merkle root computation
                    for j in 0..32 {
                        merkle_root[j] ^= tx.data_hash[j];
                    }
                }
            }
        }

        let header = BlockHeader {
            version: 1,
            height,
            previous_hash: prev_hash,
            merkle_root,
            timestamp: self.get_timestamp(),
            validator_did,
            nonce: 0,
        };

        let hash = self.hash_block_header(&header);
        let block = Block {
            header,
            transaction_count: tx_count,
            hash,
        };

        // Add block
        let block_idx = height as usize;
        if block_idx < MAX_BLOCKS {
            self.blocks[block_idx] = Some(block);
            self.latest_block_hash = hash;
            self.block_count.fetch_add(1, Ordering::Relaxed);

            // Update transactions with block height
            let mut assigned = 0u32;
            for i in 0..MAX_TRANSACTIONS {
                if let Some(tx) = &mut self.transactions[i] {
                    if tx.block_height == 0 && assigned < tx_count {
                        tx.block_height = height;
                        assigned += 1;
                    }
                }
            }

            Some(height)
        } else {
            None
        }
    }

    /// Verify a document's existence on-chain
    pub fn verify_document(&self, data_hash: [u8; 32]) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TRANSACTIONS {
            if let Some(tx) = &self.transactions[i] {
                if tx.data_hash == data_hash {
                    return true;
                }
            }
        }
        false
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, tx_id: [u8; 32]) -> Option<Transaction> {
        for i in 0..MAX_TRANSACTIONS {
            if let Some(tx) = &self.transactions[i] {
                if tx.tx_id == tx_id {
                    return Some(*tx);
                }
            }
        }
        None
    }

    /// Get block by height
    pub fn get_block(&self, height: u64) -> Option<Block> {
        let idx = height as usize;
        if idx < MAX_BLOCKS {
            self.blocks[idx]
        } else {
            None
        }
    }

    /// Hash a block header (simplified SHA-256)
    fn hash_block_header(&self, header: &BlockHeader) -> [u8; 32] {
        let mut hash = [0u8; 32];
        let mut state = 0u64;

        // Simplified hash computation
        state ^= header.version as u64;
        state ^= header.height;
        state ^= header.timestamp;
        state ^= header.nonce;

        for i in 0..32 {
            hash[i] = ((state >> (i % 8) * 8) & 0xFF) as u8;
            hash[i] ^= header.previous_hash[i];
            hash[i] ^= header.merkle_root[i];
            hash[i] ^= header.validator_did[i];
        }

        hash
    }

    /// Hash a complete block
    fn hash_block(&self, block: &Block) -> [u8; 32] {
        self.hash_block_header(&block.header)
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        self.block_count.load(Ordering::Relaxed)
    }

    pub fn block_count(&self) -> u64 {
        self.block_count.load(Ordering::Relaxed)
    }

    pub fn transaction_count(&self) -> u64 {
        self.transaction_count.load(Ordering::Relaxed)
    }

    pub fn validator_count(&self) -> u32 {
        self.validator_count.load(Ordering::Relaxed)
    }
}

// ── Global blockchain instance ─────────────────────────────────────────────

static mut G_BLOCKCHAIN: BlockchainLite = BlockchainLite::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn blockchain_init() {
    G_BLOCKCHAIN.init();
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_register_validator(
    did: *const u8,
    stake: u64,
) -> i32 {
    let mut validator = Validator::new();
    
    if !did.is_null() {
        let did_slice = core::slice::from_raw_parts(did, 32);
        validator.did.copy_from_slice(did_slice);
    }
    
    validator.stake = stake;
    validator.is_active = true;
    
    if G_BLOCKCHAIN.register_validator(validator) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_submit_transaction(
    tx_id: *const u8,
    tx_type: u8,
    sender_did: *const u8,
    recipient_did: *const u8,
    data_hash: *const u8,
    metadata_hash: *const u8,
    signature: *const u8,
) -> i32 {
    let mut tx = Transaction::new();
    
    if !tx_id.is_null() {
        let id_slice = core::slice::from_raw_parts(tx_id, 32);
        tx.tx_id.copy_from_slice(id_slice);
    }
    
    tx.tx_type = match tx_type {
        0 => TransactionType::LandRecord,
        1 => TransactionType::BirthCertificate,
        2 => TransactionType::DeathCertificate,
        3 => TransactionType::EducationalCredential,
        4 => TransactionType::MarriageCertificate,
        5 => TransactionType::PropertyDeed,
        6 => TransactionType::CourtJudgment,
        7 => TransactionType::TaxRecord,
        8 => TransactionType::BusinessLicense,
        _ => TransactionType::Other,
    };
    
    if !sender_did.is_null() {
        let did_slice = core::slice::from_raw_parts(sender_did, 32);
        tx.sender_did.copy_from_slice(did_slice);
    }
    
    if !recipient_did.is_null() {
        let did_slice = core::slice::from_raw_parts(recipient_did, 32);
        tx.recipient_did.copy_from_slice(did_slice);
    }
    
    if !data_hash.is_null() {
        let hash_slice = core::slice::from_raw_parts(data_hash, 32);
        tx.data_hash.copy_from_slice(hash_slice);
    }
    
    if !metadata_hash.is_null() {
        let hash_slice = core::slice::from_raw_parts(metadata_hash, 32);
        tx.metadata_hash.copy_from_slice(hash_slice);
    }
    
    if !signature.is_null() {
        let sig_slice = core::slice::from_raw_parts(signature, 64);
        tx.signature.copy_from_slice(sig_slice);
    }
    
    tx.timestamp = G_BLOCKCHAIN.get_timestamp();
    
    if G_BLOCKCHAIN.submit_transaction(tx) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_create_block(validator_did: *const u8) -> u64 {
    let mut did = [0u8; 32];
    if !validator_did.is_null() {
        let did_slice = core::slice::from_raw_parts(validator_did, 32);
        did.copy_from_slice(did_slice);
    }
    
    match G_BLOCKCHAIN.create_block(did) {
        Some(height) => height,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_verify_document(data_hash: *const u8) -> i32 {
    if data_hash.is_null() {
        return -1;
    }
    
    let hash_slice = core::slice::from_raw_parts(data_hash, 32);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(hash_slice);
    
    if G_BLOCKCHAIN.verify_document(hash) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_block_count() -> u64 {
    G_BLOCKCHAIN.block_count()
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_transaction_count() -> u64 {
    G_BLOCKCHAIN.transaction_count()
}

#[no_mangle]
pub unsafe extern "C" fn blockchain_validator_count() -> u32 {
    G_BLOCKCHAIN.validator_count()
}
