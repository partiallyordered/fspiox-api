// TODO: handle api version 1.0 and 1.1. How? Just a different crate for 1.1 which mostly imports
// 1.0? And re-exports 1.0? A single crate that exports both? (Probably the latter..).


use serde::{Serialize, Deserialize};
pub mod transfer;
pub mod common;
use std::option::Option;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiResourceType {
    Participants,
    Parties,
    Quotes,
    TransactionRequests,
    Authorizations,
    Transfers,
    Transactions,
    BulkQuotes,
    BulkTransfers,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    V1pt0,
    V1pt1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FspiopRequestBody {
    TransferFulfil (transfer::TransferFulfilRequestBody),
    TransferPrepare (transfer::TransferPrepareRequestBody),
}

// TODO: this is not quite correct by construction (as per the aims of this project) because it is
// possible to specify a resource_type property that does not correlate with the body property. For
// example, it is possible to specify `resource_type: Quotes` and `body:
// TransferPrepareRequestBody`. It's probably best to simply derive the former from the latter
// with pattern matching.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct FspiopRequest {
    pub request_api_version: ApiVersion,
    pub accept_api_versions: Vec<ApiVersion>,
    // TODO: should probably be a type without fractions of a second, and should accept different
    // time-zones
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub source: common::FspId,
    pub destination: common::FspId,
    // See the TODO above this struct. In the short term, the resource_type will be derived from
    // the FspiopRequestBody type when converting this request to an HTTP request.
    // pub resource_type: ApiResourceType,
    pub body: FspiopRequestBody,
    // TODO:
    // - X-Forwarded-For
    // - FSPIOP-Encryption
    // - FSPIOP-Signature
    // - FSPIOP-URI
    // See: https://github.com/mojaloop/mojaloop-specification/blob/d9393fa490ec825689ea5f325ac38e97d06956cf/fspiop-api/documents/API%20Definition%20v1.1.md#3211-http-request-header-fields
}
