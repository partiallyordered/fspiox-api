/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartyComplexName : Data model for the complex type PartyComplexName.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartyComplexName {
  /// Party’s first name.
  #[serde(rename = "firstName")]
  first_name: Option<::models::FirstName>,
  /// Party’s middle name.
  #[serde(rename = "middleName")]
  middle_name: Option<::models::MiddleName>,
  /// Party’s last name.
  #[serde(rename = "lastName")]
  last_name: Option<::models::LastName>
}

impl PartyComplexName {
  /// Data model for the complex type PartyComplexName.
  pub fn new() -> PartyComplexName {
    PartyComplexName {
      first_name: None,
      middle_name: None,
      last_name: None
    }
  }

  pub fn set_first_name(&mut self, first_name: ::models::FirstName) {
    self.first_name = Some(first_name);
  }

  pub fn with_first_name(mut self, first_name: ::models::FirstName) -> PartyComplexName {
    self.first_name = Some(first_name);
    self
  }

  pub fn first_name(&self) -> Option<&::models::FirstName> {
    self.first_name.as_ref()
  }

  pub fn reset_first_name(&mut self) {
    self.first_name = None;
  }

  pub fn set_middle_name(&mut self, middle_name: ::models::MiddleName) {
    self.middle_name = Some(middle_name);
  }

  pub fn with_middle_name(mut self, middle_name: ::models::MiddleName) -> PartyComplexName {
    self.middle_name = Some(middle_name);
    self
  }

  pub fn middle_name(&self) -> Option<&::models::MiddleName> {
    self.middle_name.as_ref()
  }

  pub fn reset_middle_name(&mut self) {
    self.middle_name = None;
  }

  pub fn set_last_name(&mut self, last_name: ::models::LastName) {
    self.last_name = Some(last_name);
  }

  pub fn with_last_name(mut self, last_name: ::models::LastName) -> PartyComplexName {
    self.last_name = Some(last_name);
    self
  }

  pub fn last_name(&self) -> Option<&::models::LastName> {
    self.last_name.as_ref()
  }

  pub fn reset_last_name(&mut self) {
    self.last_name = None;
  }

}



