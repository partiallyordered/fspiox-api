/*
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * The version of the OpenAPI document: 1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticationInfo : Data model for the complex type AuthenticationInfo



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationInfo {
    #[serde(rename = "authentication")]
    pub authentication: crate::models::AuthenticationType,
    /// Contains the authentication value. The format depends on the authentication type used in the AuthenticationInfo complex type.
    #[serde(rename = "authenticationValue")]
    pub authentication_value: String,
}

impl AuthenticationInfo {
    /// Data model for the complex type AuthenticationInfo
    pub fn new(authentication: crate::models::AuthenticationType, authentication_value: String) -> AuthenticationInfo {
        AuthenticationInfo {
            authentication,
            authentication_value,
        }
    }
}


