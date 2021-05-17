/*
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * The version of the OpenAPI document: 1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartyPersonalInfo : Data model for the complex type PartyPersonalInfo.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartyPersonalInfo {
    #[serde(rename = "complexName", skip_serializing_if = "Option::is_none")]
    pub complex_name: Option<crate::models::PartyComplexName>,
    /// Date of Birth of the Party.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
}

impl PartyPersonalInfo {
    /// Data model for the complex type PartyPersonalInfo.
    pub fn new() -> PartyPersonalInfo {
        PartyPersonalInfo {
            complex_name: None,
            date_of_birth: None,
        }
    }
}


