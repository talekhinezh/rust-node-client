/*
 * Radix Core API
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidatorFieldStateValue {
    #[serde(rename = "sorted_key", skip_serializing_if = "Option::is_none")]
    pub sorted_key: Option<Box<models::SubstateKey>>,
    #[serde(rename = "public_key")]
    pub public_key: Box<models::EcdsaSecp256k1PublicKey>,
    #[serde(rename = "is_registered")]
    pub is_registered: bool,
    #[serde(rename = "accepts_delegated_stake")]
    pub accepts_delegated_stake: bool,
    /// A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`. 
    #[serde(rename = "validator_fee_factor")]
    pub validator_fee_factor: String,
    #[serde(rename = "validator_fee_change_request", skip_serializing_if = "Option::is_none")]
    pub validator_fee_change_request: Option<Box<models::ValidatorFeeChangeRequest>>,
    /// The Bech32m-encoded human readable version of the resource address
    #[serde(rename = "stake_unit_resource_address")]
    pub stake_unit_resource_address: String,
    #[serde(rename = "stake_xrd_vault")]
    pub stake_xrd_vault: Box<models::EntityReference>,
    /// The Bech32m-encoded human readable version of the resource address
    #[serde(rename = "claim_token_resource_address")]
    pub claim_token_resource_address: String,
    #[serde(rename = "pending_xrd_withdraw_vault")]
    pub pending_xrd_withdraw_vault: Box<models::EntityReference>,
    #[serde(rename = "locked_owner_stake_unit_vault")]
    pub locked_owner_stake_unit_vault: Box<models::EntityReference>,
    #[serde(rename = "pending_owner_stake_unit_unlock_vault")]
    pub pending_owner_stake_unit_unlock_vault: Box<models::EntityReference>,
    #[serde(rename = "pending_owner_stake_unit_withdrawals")]
    pub pending_owner_stake_unit_withdrawals: Vec<models::PendingOwnerStakeWithdrawal>,
    /// A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`. 
    #[serde(rename = "already_unlocked_owner_stake_unit_amount")]
    pub already_unlocked_owner_stake_unit_amount: String,
}

impl ValidatorFieldStateValue {
    pub fn new(public_key: models::EcdsaSecp256k1PublicKey, is_registered: bool, accepts_delegated_stake: bool, validator_fee_factor: String, stake_unit_resource_address: String, stake_xrd_vault: models::EntityReference, claim_token_resource_address: String, pending_xrd_withdraw_vault: models::EntityReference, locked_owner_stake_unit_vault: models::EntityReference, pending_owner_stake_unit_unlock_vault: models::EntityReference, pending_owner_stake_unit_withdrawals: Vec<models::PendingOwnerStakeWithdrawal>, already_unlocked_owner_stake_unit_amount: String) -> ValidatorFieldStateValue {
        ValidatorFieldStateValue {
            sorted_key: None,
            public_key: Box::new(public_key),
            is_registered,
            accepts_delegated_stake,
            validator_fee_factor,
            validator_fee_change_request: None,
            stake_unit_resource_address,
            stake_xrd_vault: Box::new(stake_xrd_vault),
            claim_token_resource_address,
            pending_xrd_withdraw_vault: Box::new(pending_xrd_withdraw_vault),
            locked_owner_stake_unit_vault: Box::new(locked_owner_stake_unit_vault),
            pending_owner_stake_unit_unlock_vault: Box::new(pending_owner_stake_unit_unlock_vault),
            pending_owner_stake_unit_withdrawals,
            already_unlocked_owner_stake_unit_amount,
        }
    }
}

