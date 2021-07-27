use serde::{Serialize, Deserialize};
use crate::common::*;
use strum_macros::EnumString;
use derive_more::{FromStr, Display};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmountType {
    Receive,
    Send,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// https://github.com/mojaloop/mojaloop-specification/blob/7d8e1be6bb131a0142dc47e3d5acbb5a3f1655c7/fspiop-api/documents/API-Definition_v1.1.md#756-partyidtype
pub enum PartyIdType {
    Msisdn,
    Iban,
    Email,
    PersonalId,
    Business,
    Device,
    AccountId,
    Alias,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyIdInfo {
    pub fsp_id: FspId,
    pub party_id_type: PartyIdType,
    pub party_identifier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payee {
    pub party_id_info: PartyIdInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComplexName {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    pub complex_name: ComplexName,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payer {
    pub party_id_info: PartyIdInfo,
    pub personal_info: PersonalInfo,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display)]
pub struct TransactionId(pub CorrelationId);

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, FromStr, Display)]
pub struct QuoteId(pub CorrelationId);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InitiatorType {
    Consumer,
    Agent,
    Business,
    Device,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Initiator {
    Payer,
    Payee,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Scenario {
    Deposit,
    Withdrawal,
    Transfer,
    Payment,
    Refund,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    pub scenario: Scenario,
    pub initiator: Initiator,
    pub initiator_type: InitiatorType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequestBody {
    pub quote_id: QuoteId,
    pub transaction_id: TransactionId,
    pub payer: Payer,
    pub payee: Payee,
    pub amount_type: AmountType,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub note: String,
    pub expiration: DateTime,
}
