//! Shared types for the contract.
//!
//! This module holds the small, storage-agnostic value types used across the
//! contract's public surface. Diagnostic read-only queries such as `health()`
//! and `version()` live on the contract impl in `lib.rs`; the constants and
//! helper types they rely on are defined here so they stay in one place.

use soroban_sdk::{contracttype, symbol_short, Symbol};

/// Current contract semver, exposed via the `version()` read-only query.
///
/// This is intentionally distinct from `schema_version()`: `version()` tracks
/// the contract's own semver (bumped per `EVENTS.md`'s semver policy on schema
/// changes), while `schema_version()` tracks the storage/event schema revision.
/// Conflating the two was explicitly avoided per `DECISIONS.md`.
pub const CONTRACT_VERSION: u32 = 1;

/// Symbol used to key the contract version in storage, if ever persisted.
pub const VERSION_KEY: Symbol = symbol_short!("version");

/// Result of the `health()` diagnostic query.
///
/// A minimal self-consistency snapshot: `healthy` is true only when the
/// contract's core configuration (admin and relay signer) is fully set.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthStatus {
    /// True when the contract's minimal self-consistency check passes.
    pub healthy: bool,
    /// True when an admin address has been configured.
    pub admin_set: bool,
    /// True when a relay signer has been configured.
    pub relay_signer_set: bool,
}

impl HealthStatus {
    /// Build a health snapshot from the two configuration flags.
    pub fn new(admin_set: bool, relay_signer_set: bool) -> Self {
        Self {
            healthy: admin_set && relay_signer_set,
            admin_set,
            relay_signer_set,
        }
    }

    /// Snapshot representing an uninitialized contract (never panics).
    pub fn uninitialized() -> Self {
        Self::new(false, false)
    }
}
