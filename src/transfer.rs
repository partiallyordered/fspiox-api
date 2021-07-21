// Definitions in this file should map directly to definitions in api.yaml
// TODO: can this whole module be replaced by something that comes out of swagger generator?
// TODO: implement validation
//       https://blog.logrocket.com/json-input-validation-in-rust-web-services/
//       or try swagger generator?

use serde::{Serialize, Deserialize};
use tokio_postgres::types::ToSql;
use crate::common::*;
use strum_macros::EnumString;
use derive_more::{FromStr, Display};

// ^[A-Za-z0-9-_]{43}$
// TODO: validation
pub type IlpCondition = String;

// ^[A-Za-z0-9-_]+[=]{0,2}$
// minLength: 1, maxLength: 32768
// TODO: validation
pub type IlpPacket = String;

#[derive(Deserialize, Serialize, Debug, ToSql, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display)]
pub struct TransferId(pub CorrelationId);

#[derive(Serialize, Deserialize, Debug)]
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
pub type IlpFulfilment = String;

#[derive(Serialize, Deserialize, Debug, ToSql, EnumString)]
pub enum TransferState {
    RECEIVED,
    RESERVED,
    COMMITTED,
    ABORTED,
}

#[derive(Serialize, Deserialize, Debug)]
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
