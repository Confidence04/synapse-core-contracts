//! Soroban contract entry points.
//!
//! This module wires the public `#[contractimpl]` surface to the storage and
//! event helpers. See `THREAT_MODEL.md` §4.7 for the read-only query list.

use soroban_sdk::{contractimpl, Address, Env};

use crate::storage;

/// Current contract semver.
///
/// This is intentionally distinct from `schema_version()`: `version()` tracks
/// the contract release (bumped per `EVENTS.md`'s semver policy), while
/// `schema_version()` tracks the persisted storage/event schema. Conflating the
/// two was explicitly avoided per `DECISIONS.md`'s schema-guard discussion.
pub const CONTRACT_VERSION: u32 = 1;

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Initialize the contract with an admin and relay signer.
    pub fn initialize(env: Env, admin: Address, relay_signer: Address) {
        storage::set_admin(&env, &admin);
        storage::set_relay_signer(&env, &relay_signer);
    }

    /// Return the current contract semver.
    ///
    /// Stable across calls within a single deployment and independent from
    /// `schema_version()`. Bumped on schema changes per `EVENTS.md`.
    pub fn version(_env: Env) -> u32 {
        CONTRACT_VERSION
    }

    /// Minimal liveness / self-consistency check.
    ///
    /// Returns `true` only when both the admin and relay signer are set. Safe to
    /// call before `initialize()`: it reports `false` rather than panicking.
    pub fn health(env: Env) -> bool {
        storage::get_admin(&env).is_some() && storage::get_relay_signer(&env).is_some()
    }
}
