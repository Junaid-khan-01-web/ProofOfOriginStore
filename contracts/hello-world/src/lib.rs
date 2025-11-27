#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Bytes, Env, String, Symbol,
};

const ORIGIN_NS: Symbol = symbol_short!("ORIGIN"); // namespace for keys

#[contracttype]
#[derive(Clone)]
pub struct OriginRecord {
    pub batch_id: String,      // unique batch / lot identifier
    pub producer_id: Symbol,   // farm, factory, or supplier id
    pub origin_country: String,
    pub origin_region: String, // region / state / certified zone
    pub metadata_hash: Bytes,  // hash of detailed off‑chain docs (CoA, lab report, etc.)
    pub created_at: u64,       // ledger timestamp when registered
    pub locked: bool,          // if true, record is immutable (no further edits)
}

#[contract]
pub struct ProofOfOriginStore;

#[contractimpl]
impl ProofOfOriginStore {
    /// Register a new batch with origin data.
    /// Off‑chain system should enforce producer permissions.
    pub fn register_batch(
        env: Env,
        batch_id: String,
        producer_id: Symbol,
        origin_country: String,
        origin_region: String,
        metadata_hash: Bytes,
    ) {
        let key = Self::batch_key(batch_id.clone());
        // Prevent overwrite if already locked
        let existing: Option<OriginRecord> = env.storage().instance().get(&key);
        if let Some(r) = existing {
            if r.locked {
                panic!("Batch already locked");
            }
        }

        let created_at = env.ledger().timestamp();
        let record = OriginRecord {
            batch_id,
            producer_id,
            origin_country,
            origin_region,
            metadata_hash,
            created_at,
            locked: false,
        };

        env.storage().instance().set(&key, &record);
    }

    /// Lock a batch record so no more edits are allowed.
    pub fn lock_batch(env: Env, batch_id: String) {
        let key = Self::batch_key(batch_id.clone());
        let mut record: OriginRecord = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("Batch not found"));

        record.locked = true;
        env.storage().instance().set(&key, &record);
    }

    /// Check if a batch exists and is locked (finalized origin proof).
    pub fn is_batch_final(env: Env, batch_id: String) -> bool {
        let key = Self::batch_key(batch_id);
        let rec: Option<OriginRecord> = env.storage().instance().get(&key);
        match rec {
            Some(r) => r.locked,
            None => false,
        }
    }

    /// Get full origin information for a batch.
    pub fn get_origin(env: Env, batch_id: String) -> Option<OriginRecord> {
        let key = Self::batch_key(batch_id);
        env.storage().instance().get(&key)
    }

    /// Internal helper: composite storage key under ORIGIN_NS.
    fn batch_key(batch_id: String) -> (Symbol, String) {
        (ORIGIN_NS, batch_id)
    }
}
