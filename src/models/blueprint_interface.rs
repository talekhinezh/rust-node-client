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
pub struct BlueprintInterface {
    #[serde(rename = "outer_blueprint", skip_serializing_if = "Option::is_none")]
    pub outer_blueprint: Option<String>,
    #[serde(rename = "generic_type_parameters")]
    pub generic_type_parameters: Vec<crate::models::GenericTypeParameter>,
    /// If true, an instantiation of this blueprint cannot be persisted. EG buckets and proofs are transient.
    #[serde(rename = "is_transient")]
    pub is_transient: bool,
    #[serde(rename = "features")]
    pub features: Vec<String>,
    #[serde(rename = "state")]
    pub state: Box<crate::models::IndexedStateSchema>,
    /// A map from the function name to the FunctionSchema
    #[serde(rename = "functions", deserialize_with = "Option::deserialize")]
    pub functions: Option<::std::collections::HashMap<String, crate::models::FunctionSchema>>,
    /// A map from the event name to the event payload type reference.
    #[serde(rename = "events", deserialize_with = "Option::deserialize")]
    pub events: Option<::std::collections::HashMap<String, crate::models::BlueprintPayloadDef>>,
    /// A map from the registered type name to the concrete type, resolved against a schema from the package's schema partition. 
    #[serde(rename = "types", deserialize_with = "Option::deserialize")]
    pub types: Option<::std::collections::HashMap<String, crate::models::ScopedTypeId>>,
}

impl BlueprintInterface {
    pub fn new(generic_type_parameters: Vec<crate::models::GenericTypeParameter>, is_transient: bool, features: Vec<String>, state: crate::models::IndexedStateSchema, functions: Option<::std::collections::HashMap<String, crate::models::FunctionSchema>>, events: Option<::std::collections::HashMap<String, crate::models::BlueprintPayloadDef>>, types: Option<::std::collections::HashMap<String, crate::models::ScopedTypeId>>) -> BlueprintInterface {
        BlueprintInterface {
            outer_blueprint: None,
            generic_type_parameters,
            is_transient,
            features,
            state: Box::new(state),
            functions,
            events,
            types,
        }
    }
}


