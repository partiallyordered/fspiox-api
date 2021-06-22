// Definitions in this file should map directly to definitions in api.yaml
// TODO: can this whole module be replaced by something that comes out of swagger generator?
// TODO: implement validation
//       https://blog.logrocket.com/json-input-validation-in-rust-web-services/
//       or try swagger generator?

use serde::{Serialize, Deserialize};
use tokio_postgres::types::ToSql;
use crate::common::*;
use strum_macros::EnumString;

// ^[A-Za-z0-9-_]{43}$
// TODO: validation
pub type IlpCondition = String;

// ^[A-Za-z0-9-_]+[=]{0,2}$
// minLength: 1, maxLength: 32768
// TODO: validation
pub type IlpPacket = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferPrepareRequestBody {
    pub transfer_id: CorrelationId,
    pub payee_fsp: FspId,
    pub payer_fsp: FspId,
    pub amount: Money,
    pub ilp_packet: IlpPacket,
    pub condition: IlpCondition,
    #[serde(with = "fspiop_serde_date_formatter")]
    pub expiration: DateTime,
    // TODO: handle extensionList
}

// Thanks go to https://serde.rs/custom-date-format.html
mod fspiop_serde_date_formatter {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
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
    pub fulfilment: IlpFulfilment,
    pub completed_timestamp: DateTime,
    pub transfer_state: TransferState,
    // TODO: handle extensionList
}
