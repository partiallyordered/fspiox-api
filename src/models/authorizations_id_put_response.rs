/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AuthorizationsIdPutResponse : PUT /authorizations/{ID} object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsIdPutResponse {
  /// OTP or QR Code if entered, otherwise empty.
  #[serde(rename = "authenticationInfo")]
  authentication_info: Option<::models::AuthenticationInfo>,
  /// Enum containing response information; if the customer entered the authentication value, rejected the transaction, or requested a resend of the authentication value.
  #[serde(rename = "responseType")]
  response_type: ::models::AuthorizationResponse
}

impl AuthorizationsIdPutResponse {
  /// PUT /authorizations/{ID} object
  pub fn new(response_type: ::models::AuthorizationResponse) -> AuthorizationsIdPutResponse {
    AuthorizationsIdPutResponse {
      authentication_info: None,
      response_type: response_type
    }
  }

  pub fn set_authentication_info(&mut self, authentication_info: ::models::AuthenticationInfo) {
    self.authentication_info = Some(authentication_info);
  }

  pub fn with_authentication_info(mut self, authentication_info: ::models::AuthenticationInfo) -> AuthorizationsIdPutResponse {
    self.authentication_info = Some(authentication_info);
    self
  }

  pub fn authentication_info(&self) -> Option<&::models::AuthenticationInfo> {
    self.authentication_info.as_ref()
  }

  pub fn reset_authentication_info(&mut self) {
    self.authentication_info = None;
  }

  pub fn set_response_type(&mut self, response_type: ::models::AuthorizationResponse) {
    self.response_type = response_type;
  }

  pub fn with_response_type(mut self, response_type: ::models::AuthorizationResponse) -> AuthorizationsIdPutResponse {
    self.response_type = response_type;
    self
  }

  pub fn response_type(&self) -> &::models::AuthorizationResponse {
    &self.response_type
  }


}



