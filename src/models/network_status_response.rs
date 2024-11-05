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
pub struct NetworkStatusResponse {
    /// The ledger state identifier of a fresh ledger before any genesis transactions. 
    #[serde(rename = "pre_genesis_state_identifier")]
    pub pre_genesis_state_identifier: Box<models::CommittedStateIdentifier>,
    /// The epoch details for the genesis epoch and round. The genesis epoch will be the last Olympia epoch + 1, and have a \"fake\" round-number 1 (because there is no round for the genesis transaction). In the Gateway, this can be used for the epoch and round number before the first RoundUpdate transaction. It is not present until genesis has been run. 
    #[serde(rename = "genesis_epoch_round", skip_serializing_if = "Option::is_none")]
    pub genesis_epoch_round: Option<Box<models::EpochRound>>,
    /// The ledger state after the genesis transactions have been executed. It is not present until genesis has been run. 
    #[serde(rename = "post_genesis_state_identifier", skip_serializing_if = "Option::is_none")]
    pub post_genesis_state_identifier: Option<Box<models::CommittedStateIdentifier>>,
    /// The post-genesis epoch and round. 
    #[serde(rename = "post_genesis_epoch_round", skip_serializing_if = "Option::is_none")]
    pub post_genesis_epoch_round: Option<Box<models::EpochRound>>,
    /// The current state identifier at the top of the node's copy of the ledger (ie as of the latest committed transaction). It is not present until genesis has been run. 
    #[serde(rename = "current_state_identifier", skip_serializing_if = "Option::is_none")]
    pub current_state_identifier: Option<Box<models::CommittedStateIdentifier>>,
    /// The current epoch and round of the ledger. It is not present until genesis has been run. 
    #[serde(rename = "current_epoch_round", skip_serializing_if = "Option::is_none")]
    pub current_epoch_round: Option<Box<models::EpochRound>>,
    /// A descriptor for the current protocol version that the node is running. 
    #[serde(rename = "current_protocol_version")]
    pub current_protocol_version: String,
}

impl NetworkStatusResponse {
    pub fn new(pre_genesis_state_identifier: models::CommittedStateIdentifier, current_protocol_version: String) -> NetworkStatusResponse {
        NetworkStatusResponse {
            pre_genesis_state_identifier: Box::new(pre_genesis_state_identifier),
            genesis_epoch_round: None,
            post_genesis_state_identifier: None,
            post_genesis_epoch_round: None,
            current_state_identifier: None,
            current_epoch_round: None,
            current_protocol_version,
        }
    }
}

