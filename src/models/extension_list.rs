/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionList : Data model for the complex type ExtensionList

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionList {
  /// Number of Extension elements
  #[serde(rename = "extension")]
  extension: Vec<::models::Extension>
}

impl ExtensionList {
  /// Data model for the complex type ExtensionList
  pub fn new(extension: Vec<::models::Extension>) -> ExtensionList {
    ExtensionList {
      extension: extension
    }
  }

  pub fn set_extension(&mut self, extension: Vec<::models::Extension>) {
    self.extension = extension;
  }

  pub fn with_extension(mut self, extension: Vec<::models::Extension>) -> ExtensionList {
    self.extension = extension;
    self
  }

  pub fn extension(&self) -> &Vec<::models::Extension> {
    &self.extension
  }


}



