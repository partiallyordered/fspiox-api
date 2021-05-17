/*
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * The version of the OpenAPI document: 1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IndividualTransferResult : Data model for the complex type IndividualTransferResult.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualTransferResult {
    /// Identifier that correlates all messages of the same sequence. The API data type UUID (Universally Unique Identifier) is a JSON String in canonical format, conforming to RFC 4122, that is restricted by a regular expression for interoperability reasons. An UUID is always 36 characters long, 32 hexadecimal symbols and 4 dashes (‘-‘).
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Fulfilment that must be attached to the transfer by the Payee.
    #[serde(rename = "fulfilment", skip_serializing_if = "Option::is_none")]
    pub fulfilment: Option<String>,
    #[serde(rename = "errorInformation", skip_serializing_if = "Option::is_none")]
    pub error_information: Option<crate::models::ErrorInformation>,
    #[serde(rename = "extensionList", skip_serializing_if = "Option::is_none")]
    pub extension_list: Option<crate::models::ExtensionList>,
}

impl IndividualTransferResult {
    /// Data model for the complex type IndividualTransferResult.
    pub fn new(transfer_id: String) -> IndividualTransferResult {
        IndividualTransferResult {
            transfer_id,
            fulfilment: None,
            error_information: None,
            extension_list: None,
        }
    }
}


