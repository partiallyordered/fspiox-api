use serde::{Serialize, Deserialize};
use crate::*;
use strum_macros::EnumString;
use derive_more::{FromStr, Display};
use chrono::Utc;

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

impl QuoteRequestBody {
    pub fn generate(
        payer_fsp: FspId,
        payee_fsp: FspId,
        amount: Amount,
        currency: Currency,
    ) -> QuoteRequestBody {
        QuoteRequestBody {
            quote_id: QuoteId(common::CorrelationId::new()),
            transaction_id: TransactionId(common::CorrelationId::new()),
            payer: Payer {
                party_id_info: PartyIdInfo {
                    fsp_id: payer_fsp.clone(),
                    party_id_type: PartyIdType::Msisdn,
                    party_identifier: "1234567890".to_string(),
                },
                personal_info: PersonalInfo {
                    complex_name: ComplexName {
                        first_name: "Mats".to_string(),
                        last_name: "Hagman".to_string(),
                    },
                },
            },
            payee: Payee {
                party_id_info: PartyIdInfo {
                    fsp_id: payee_fsp.clone(),
                    party_id_type: PartyIdType::Msisdn,
                    party_identifier: "1234567890".to_string(),
                },
            },
            amount_type: AmountType::Send,
            amount: common::Money {
                currency,
                amount,
            },
            transaction_type: TransactionType {
                scenario: Scenario::Transfer,
                initiator: Initiator::Payer,
                initiator_type: InitiatorType::Consumer,
            },
            note: "Created by fspiox-api".to_string(),
            expiration: common::DateTime(Utc::now().checked_add_signed(chrono::Duration::hours(1)).unwrap()),
        }

    }
}

pub struct QuoteRequest(pub FspiopRequest);

impl QuoteRequest {
    pub fn new(
        payer_fsp: FspId,
        payee_fsp: FspId,
        amount: Amount,
        currency: Currency,
    ) -> QuoteRequest {
        QuoteRequest(
            FspiopRequest {
                source: payer_fsp.clone(),
                destination: payee_fsp.clone(),
                path: "/quotes".parse::<http::Uri>().unwrap(), // TODO: we can make this infallible
                resource: FspiopResource::Quotes,
                method: FspiopMethod::POST,
                request_api_version: ApiVersion::V1pt0,
                accept_api_versions: vec![ApiVersion::V1pt0],
                date: Some(Utc::now()),
                body: FspiopRequestBody::PostQuotes(QuoteRequestBody::generate(
                    payer_fsp, payee_fsp, amount, currency
                )),
            }
        )
    }
}
