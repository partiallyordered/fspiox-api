// Definitions in this file should map directly to definitions in api.yaml
// TODO: can this whole module be replaced by something that comes out of swagger generator?
// TODO: implement validation
//       https://blog.logrocket.com/json-input-validation-in-rust-web-services/
//       or try swagger generator?

use serde::{Serialize, Deserialize};
use crate::common::*;
use strum_macros::EnumString;
use derive_more::{FromStr, Display};

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// ^[A-Za-z0-9-_]{43}$
// TODO: validation
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display)]
pub struct IlpCondition(arrayvec::ArrayString<43>);

impl IlpCondition {
    pub fn from(item: &str) -> Result<Self, arrayvec::CapacityError<&str>> {
        Ok(IlpCondition(arrayvec::ArrayString::from(item)?))
    }
}

#[cfg(feature = "typescript_types")]
impl TS for IlpCondition {
    fn name() -> String {
        "IlpCondition".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type IlpCondition = string".to_string()
    }
}

// ^[A-Za-z0-9-_]+[=]{0,2}$
// minLength: 1, maxLength: 32768
// TODO: validation
// TODO: we've limited the size of the ILP packet here because:
// 1. we're not the actual ML API
// 2. 32768 bytes?! Yeah, I get it. But I'd probably limit that on my own API for performance.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display, FromStr)]
pub struct IlpPacket(arrayvec::ArrayString<256>);

impl IlpPacket {
    pub fn from(item: &str) -> Result<Self, arrayvec::CapacityError<&str>> {
        Ok(IlpPacket(arrayvec::ArrayString::from(item)?))
    }
}


#[cfg(feature = "typescript_types")]
impl TS for IlpPacket {
    fn name() -> String {
        "IlpPacket".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type IlpPacket = string".to_string()
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display)]
pub struct TransferId(pub CorrelationId);

#[cfg(feature = "typescript_types")]
impl TS for TransferId {
    fn name() -> String {
        "string".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TransferPrepareRequestBody {
    pub transfer_id: TransferId,
    pub payee_fsp: FspId,
    pub payer_fsp: FspId,
    pub amount: Money,
    pub ilp_packet: IlpPacket,
    pub condition: IlpCondition,
    pub expiration: DateTime,
    // TODO: handle extensionList
}

// pattern: ^[A-Za-z0-9-_]{43}$
// but..
// maxLength: 48
// according to the openapi spec
// TODO: validation
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display)]
pub struct IlpFulfilment(arrayvec::ArrayString<43>);

impl IlpFulfilment {
    pub fn from(item: &str) -> Result<Self, arrayvec::CapacityError<&str>> {
        Ok(IlpFulfilment(arrayvec::ArrayString::from(item)?))
    }
}

#[cfg(feature = "typescript_types")]
impl TS for IlpFulfilment {
    fn name() -> String {
        "IlpFulfilment".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type IlpFulfilment = string".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, EnumString, Clone, Copy)]
pub enum TransferState {
    RECEIVED,
    RESERVED,
    COMMITTED,
    ABORTED,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TransferFulfilRequestBody {
    // TODO:
    // The spec doesn't actually require all of these fields. This is because the requests
    // supported by PUT /transfers/$id are actually of three different types, depending on the
    // provided transfer_state. See:
    //   https://github.com/mojaloop/mojaloop-specification/blob/a66ea83f2a0c1e0073fe8bf93f301c01469d025d/fspiop-api/documents/v1.1-document-set/fspiop-v1.1-openapi2.yaml#L4358-L4359
    // This is a shortcoming of Swagger, and is probably best modeled as a sum type in Rust.
    pub fulfilment: IlpFulfilment,
    pub completed_timestamp: DateTime,
    pub transfer_state: TransferState,
    // TODO: handle extensionList
}
