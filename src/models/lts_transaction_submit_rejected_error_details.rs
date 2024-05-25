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
pub struct LtsTransactionSubmitRejectedErrorDetails {
    #[serde(rename = "type")]
    pub r#type: crate::models::LtsTransactionSubmitErrorDetailsType,
    /// An explanation of the error
    #[serde(rename = "error_message")]
    pub error_message: String,
    /// Whether (true) this rejected status has just been calculated fresh, or (false) the status is from the pending transaction result cache. 
    #[serde(rename = "is_fresh")]
    pub is_fresh: bool,
    /// Whether the rejection of this payload is known to be permanent. 
    #[serde(rename = "is_payload_rejection_permanent")]
    pub is_payload_rejection_permanent: bool,
    /// Whether the rejection of this intent is known to be permanent - this is a stronger statement than the payload rejection being permanent, as it implies any payloads containing the intent will also be permanently rejected. 
    #[serde(rename = "is_intent_rejection_permanent")]
    pub is_intent_rejection_permanent: bool,
    /// The time after which the node will consider recalculating the validity of the transaction. Only present if the rejection is temporary, and not due to the header specifying a \"from epoch\" in the future. 
    #[serde(rename = "retry_from_timestamp", skip_serializing_if = "Option::is_none")]
    pub retry_from_timestamp: Option<Box<crate::models::Instant>>,
    /// An integer between `0` and `10^10`, marking the epoch after which the node will consider recalculating the validity of the transaction. Only present if the rejection is temporary due to a header specifying a \"from epoch\" in the future. 
    #[serde(rename = "retry_from_epoch", skip_serializing_if = "Option::is_none")]
    pub retry_from_epoch: Option<i64>,
    /// An integer between `0` and `10^10`, marking the epoch from which the transaction will no longer be valid, and be permanently rejected. Only present if the rejection isn't permanent. 
    #[serde(rename = "invalid_from_epoch", skip_serializing_if = "Option::is_none")]
    pub invalid_from_epoch: Option<i64>,
}

impl LtsTransactionSubmitRejectedErrorDetails {
    pub fn new(r#type: crate::models::LtsTransactionSubmitErrorDetailsType, error_message: String, is_fresh: bool, is_payload_rejection_permanent: bool, is_intent_rejection_permanent: bool) -> LtsTransactionSubmitRejectedErrorDetails {
        LtsTransactionSubmitRejectedErrorDetails {
            r#type,
            error_message,
            is_fresh,
            is_payload_rejection_permanent,
            is_intent_rejection_permanent,
            retry_from_timestamp: None,
            retry_from_epoch: None,
            invalid_from_epoch: None,
        }
    }
}


