use soroban_sdk::{contracttype, Address, Env};

use crate::errors::ContractError;

/// Storage keys used by the relay contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Address of the currently active relay signer.
    RelaySigner,
    /// Address of the contract administrator.
    Admin,
}

/// Persist the active relay signer address in instance storage.
pub fn set_relay_signer(env: &Env, signer: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::RelaySigner, signer);
}

/// Read the active relay signer address from instance storage.
///
/// Returns a clean [`ContractError::NotInitialized`] when the contract has not
/// been initialized yet, rather than panicking on a missing storage key.
pub fn get_relay_signer(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::RelaySigner)
        .ok_or(ContractError::NotInitialized)
}
