/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AuthenticationValue : Contains the authentication value. The format depends on the authentication type used in the AuthenticationInfo complex type.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationValue {
}

impl AuthenticationValue {
  /// Contains the authentication value. The format depends on the authentication type used in the AuthenticationInfo complex type.
  pub fn new() -> AuthenticationValue {
    AuthenticationValue {
    }
  }

}



