/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Extension : Data model for the complex type Extension

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
  /// Extension key.
  #[serde(rename = "key")]
  key: ::models::ExtensionKey,
  /// Extension value.
  #[serde(rename = "value")]
  value: ::models::ExtensionValue
}

impl Extension {
  /// Data model for the complex type Extension
  pub fn new(key: ::models::ExtensionKey, value: ::models::ExtensionValue) -> Extension {
    Extension {
      key: key,
      value: value
    }
  }

  pub fn set_key(&mut self, key: ::models::ExtensionKey) {
    self.key = key;
  }

  pub fn with_key(mut self, key: ::models::ExtensionKey) -> Extension {
    self.key = key;
    self
  }

  pub fn key(&self) -> &::models::ExtensionKey {
    &self.key
  }


  pub fn set_value(&mut self, value: ::models::ExtensionValue) {
    self.value = value;
  }

  pub fn with_value(mut self, value: ::models::ExtensionValue) -> Extension {
    self.value = value;
    self
  }

  pub fn value(&self) -> &::models::ExtensionValue {
    &self.value
  }


}



