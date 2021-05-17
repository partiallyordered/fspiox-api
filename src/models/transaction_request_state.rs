/*
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * The version of the OpenAPI document: 1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionRequestState : Below are the allowed values for the enumeration - RECEIVED Payer FSP has received the transaction from the Payee FSP. - PENDING Payer FSP has sent the transaction request to the Payer. - ACCEPTED Payer has approved the transaction. - REJECTED Payer has rejected the transaction.

/// Below are the allowed values for the enumeration - RECEIVED Payer FSP has received the transaction from the Payee FSP. - PENDING Payer FSP has sent the transaction request to the Payer. - ACCEPTED Payer has approved the transaction. - REJECTED Payer has rejected the transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionRequestState {
    #[serde(rename = "RECEIVED")]
    RECEIVED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "ACCEPTED")]
    ACCEPTED,
    #[serde(rename = "REJECTED")]
    REJECTED,

}




