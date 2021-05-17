/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PersonalIdentifierType : Below are the allowed values for the enumeration - PASSPORT A passport number is used as reference to a Party. - NATIONAL_REGISTRATION A national registration number is used as reference to a Party. - DRIVING_LICENSE A driving license is used as reference to a Party. - ALIEN_REGISTRATION An alien registration number is used as reference to a Party. - NATIONAL_ID_CARD A national ID card number is used as reference to a Party. - EMPLOYER_ID A tax identification number is used as reference to a Party. - TAX_ID_NUMBER A tax identification number is used as reference to a Party. - SENIOR_CITIZENS_CARD A senior citizens card number is used as reference to a Party. - MARRIAGE_CERTIFICATE A marriage certificate number is used as reference to a Party. - HEALTH_CARD A health card number is used as reference to a Party. - VOTERS_ID A voter’s identification number is used as reference to a Party. - UNITED_NATIONS An UN (United Nations) number is used as reference to a Party. - OTHER_ID Any other type of identification type number is used as reference to a Party.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalIdentifierType {
}

impl PersonalIdentifierType {
  /// Below are the allowed values for the enumeration - PASSPORT A passport number is used as reference to a Party. - NATIONAL_REGISTRATION A national registration number is used as reference to a Party. - DRIVING_LICENSE A driving license is used as reference to a Party. - ALIEN_REGISTRATION An alien registration number is used as reference to a Party. - NATIONAL_ID_CARD A national ID card number is used as reference to a Party. - EMPLOYER_ID A tax identification number is used as reference to a Party. - TAX_ID_NUMBER A tax identification number is used as reference to a Party. - SENIOR_CITIZENS_CARD A senior citizens card number is used as reference to a Party. - MARRIAGE_CERTIFICATE A marriage certificate number is used as reference to a Party. - HEALTH_CARD A health card number is used as reference to a Party. - VOTERS_ID A voter’s identification number is used as reference to a Party. - UNITED_NATIONS An UN (United Nations) number is used as reference to a Party. - OTHER_ID Any other type of identification type number is used as reference to a Party.
  pub fn new() -> PersonalIdentifierType {
    PersonalIdentifierType {
    }
  }

}

// TODO enum 
// List of PersonalIdentifierType
//const (
//  
//  
//  
//)


