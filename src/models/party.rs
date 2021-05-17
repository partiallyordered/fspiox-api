/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Party : Data model for the complex type Party.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
  /// Party Id type, id, sub ID or type, and FSP Id.
  #[serde(rename = "partyIdInfo")]
  party_id_info: ::models::PartyIdInfo,
  /// Used in the context of Payee Information, where the Payee happens to be a merchant accepting merchant payments.
  #[serde(rename = "merchantClassificationCode")]
  merchant_classification_code: Option<::models::MerchantClassificationCode>,
  /// Display name of the Party, could be a real name or a nick name.
  #[serde(rename = "name")]
  name: Option<::models::PartyName>,
  /// Personal information used to verify identity of Party such as first, middle, last name and date of birth.
  #[serde(rename = "personalInfo")]
  personal_info: Option<::models::PartyPersonalInfo>
}

impl Party {
  /// Data model for the complex type Party.
  pub fn new(party_id_info: ::models::PartyIdInfo) -> Party {
    Party {
      party_id_info: party_id_info,
      merchant_classification_code: None,
      name: None,
      personal_info: None
    }
  }

  pub fn set_party_id_info(&mut self, party_id_info: ::models::PartyIdInfo) {
    self.party_id_info = party_id_info;
  }

  pub fn with_party_id_info(mut self, party_id_info: ::models::PartyIdInfo) -> Party {
    self.party_id_info = party_id_info;
    self
  }

  pub fn party_id_info(&self) -> &::models::PartyIdInfo {
    &self.party_id_info
  }


  pub fn set_merchant_classification_code(&mut self, merchant_classification_code: ::models::MerchantClassificationCode) {
    self.merchant_classification_code = Some(merchant_classification_code);
  }

  pub fn with_merchant_classification_code(mut self, merchant_classification_code: ::models::MerchantClassificationCode) -> Party {
    self.merchant_classification_code = Some(merchant_classification_code);
    self
  }

  pub fn merchant_classification_code(&self) -> Option<&::models::MerchantClassificationCode> {
    self.merchant_classification_code.as_ref()
  }

  pub fn reset_merchant_classification_code(&mut self) {
    self.merchant_classification_code = None;
  }

  pub fn set_name(&mut self, name: ::models::PartyName) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: ::models::PartyName) -> Party {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&::models::PartyName> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_personal_info(&mut self, personal_info: ::models::PartyPersonalInfo) {
    self.personal_info = Some(personal_info);
  }

  pub fn with_personal_info(mut self, personal_info: ::models::PartyPersonalInfo) -> Party {
    self.personal_info = Some(personal_info);
    self
  }

  pub fn personal_info(&self) -> Option<&::models::PartyPersonalInfo> {
    self.personal_info.as_ref()
  }

  pub fn reset_personal_info(&mut self) {
    self.personal_info = None;
  }

}



