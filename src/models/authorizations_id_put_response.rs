/*
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * The version of the OpenAPI document: 1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthorizationsIdPutResponse : PUT /authorizations/{ID} object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationsIdPutResponse {
    #[serde(rename = "authenticationInfo", skip_serializing_if = "Option::is_none")]
    pub authentication_info: Option<crate::models::AuthenticationInfo>,
    #[serde(rename = "responseType")]
    pub response_type: crate::models::AuthorizationResponse,
}

impl AuthorizationsIdPutResponse {
    /// PUT /authorizations/{ID} object
    pub fn new(response_type: crate::models::AuthorizationResponse) -> AuthorizationsIdPutResponse {
        AuthorizationsIdPutResponse {
            authentication_info: None,
            response_type,
        }
    }
}


