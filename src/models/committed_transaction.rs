/*
 * Radix Core API - Babylon
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.0.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommittedTransaction {
    #[serde(rename = "resultant_state_identifiers")]
    pub resultant_state_identifiers: Box<crate::models::CommittedStateIdentifier>,
    #[serde(rename = "ledger_transaction")]
    pub ledger_transaction: Box<crate::models::LedgerTransaction>,
    #[serde(rename = "receipt")]
    pub receipt: Box<crate::models::TransactionReceipt>,
    /// An integer between `0` and `10^14`, marking the proposer timestamp in ms.
    #[serde(rename = "proposer_timestamp_ms")]
    pub proposer_timestamp_ms: i64,
}

impl CommittedTransaction {
    pub fn new(resultant_state_identifiers: crate::models::CommittedStateIdentifier, ledger_transaction: crate::models::LedgerTransaction, receipt: crate::models::TransactionReceipt, proposer_timestamp_ms: i64) -> CommittedTransaction {
        CommittedTransaction {
            resultant_state_identifiers: Box::new(resultant_state_identifiers),
            ledger_transaction: Box::new(ledger_transaction),
            receipt: Box::new(receipt),
            proposer_timestamp_ms,
        }
    }
}


