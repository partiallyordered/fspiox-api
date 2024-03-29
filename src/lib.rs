// TODO: handle api version 1.0 and 1.1. How? Just a different crate for 1.1 which mostly imports
// 1.0? And re-exports 1.0? A single crate that exports both? (Probably the latter..).


#[cfg(feature = "fsp_http")]
use http;
#[cfg(feature = "fsp_http")]
use http_serde;
use serde::{Deserialize, Serialize};
#[cfg(feature = "clients")]
pub mod clients;
pub mod transfer;
pub mod quote;
mod common;
pub use common::*;
use std::option::Option;
use std::vec::Vec;
use chrono::Utc;
use std::string::ToString;

#[derive(Debug)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, derive_more::Display)]
pub enum ApiVersion {
    #[display(fmt = "1.0")]
    V1pt0,
    #[display(fmt = "1.1")]
    V1pt1,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FspiopRequestBody {
    TransferFulfil (transfer::TransferFulfilRequestBody),
    TransferPrepare (transfer::TransferPrepareRequestBody),
    PostQuotes (quote::QuoteRequestBody),
    // TODO: is this necessary still? Is it used anywhere? Should it instead be represented by an
    // Option type? I.e. body: Option<FspiopRequestBody> ?
    NoBody,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, strum_macros::ToString)]
pub enum FspiopMethod {
    GET,
    PUT,
    POST,
    // TODO: PATCH
    // (this will probably occur as part of a broader v1.1 effort)
}

impl From<FspiopMethod> for http::Method {
    fn from(item: FspiopMethod) -> Self {
        match item {
            FspiopMethod::GET => http::Method::GET,
            FspiopMethod::PUT => http::Method::PUT,
            FspiopMethod::POST => http::Method::POST,
        }
    }
}

// https://github.com/mojaloop/mojaloop-specification/blob/7d8e1be6bb131a0142dc47e3d5acbb5a3f1655c7/fspiop-api/documents/API-Definition_v1.1.md#3133-path
#[derive(Debug, Deserialize, Serialize, Clone, Copy, strum_macros::Display)]
#[strum(serialize_all = "camelCase")]
pub enum FspiopResource {
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

// TODO: this is not quite correct by construction (as per the aims of this project) because it is
// possible to specify a resource_type property that does not correlate with the body property. For
// example, it is possible to specify `resource_type: Quotes` and `body:
// TransferPrepareRequestBody`. It's probably best to simply derive the former from the latter
// with pattern matching.
// TODO: derive Copy? Or maybe just Sized?
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub method: FspiopMethod,
    pub resource: FspiopResource,
    // TODO: A more precise type for the path field would be http::uri::PathAndQuery (or, if it
    // existed, http::uri::Path). It might in fact be best if we wrote our own type here, as the
    // path goes into an HTTP header, which means it must be ascii bytes between 32 and 255
    // excluding 127/DEL, IIRC. We should create a type that can represent only this data.
    #[serde(with = "http_serde::uri")]
    pub path: http::Uri,
    // TODO:
    // - X-Forwarded-For
    // - FSPIOP-Encryption
    // - FSPIOP-Signature
    // - FSPIOP-URI
    // See: https://github.com/mojaloop/mojaloop-specification/blob/d9393fa490ec825689ea5f325ac38e97d06956cf/fspiop-api/documents/API%20Definition%20v1.1.md#3211-http-request-header-fields
}

// TODO: is this in use anywhere? We should at least replace it with From/Into.
#[cfg(feature = "fsp_http")]
pub fn to_http_request(
    req: FspiopRequest,
    host: &str
) -> Result<http::Request<String>, url::ParseError> {
    use url::Url;
    // For now, matching on method to build the body (or not) seems to be working. If it breaks
    // down, it should be possible to match on the body type. I.e.
    // match req.body {
    //     FspiopRequestBody::TransferPrepare(transfer_prepare_request_body) => { .. build body }
    // }
    let body = match req.method {
        FspiopMethod::GET => "".to_string(),
        // Cheeky: because we configured serde to serialize the FspiopRequestBody as untagged, the
        // result of to_string here will consist only of the actual request body.
        FspiopMethod::PUT | FspiopMethod::POST => serde_json::to_string(&req.body).unwrap(),
    };
    let accept = req.accept_api_versions
        .iter()
        .map(|v| format!("application/vnd.interoperability.{}+json;version={}", req.resource, v))
        .collect::<Vec<String>>()
        .join(",");
    Ok(
        http::request::Builder::new()
            // TODO: probably we should accept a url::Uri as host, then
            // .uri(host.join(clr.path().as_str()).unwrap().as_str())
            // then make sure in unit testing that every path we would use here is a valid URI
            // path, so that the unwrap() shouldn't panic (as long as host.join(path) is valid,
            // which it should be, I think..?). Or should we take a string and strip any trailing
            // slash, to allow a user to build a request with a relative uri using this function.
            .uri(Url::parse(host)?.join(&req.path.to_string())?.as_str())
            // .header("Date", req.date.unwrap_or(Utc::now()).to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            // The implementation of ml-api-adapter says:
            //   \"date\" must be in ddd, D MMM YYYY H:mm:ss [GMT] format
            // In practice, it requires the GMT string. And does not accept other timezones (this may
            // be intentional, but when the implementation clearly contradicts the error message, it
            // becomes more difficult to be certain). Ostensibly, it follows the RFC here:
            // https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2
            // TODO: this should be fixed upstream
            .header("Date", req.date.unwrap_or(Utc::now()).format("%a, %d %b %Y %T GMT").to_string())
            .header("FSPIOP-URI", req.path.to_string())
            .header("FSPIOP-HTTP-Method", req.method.to_string())
            .header("FSPIOP-Source", req.source.to_string())
            .header("FSPIOP-Destination", req.destination.to_string())
            .header("Accept", accept)
            .header("Content-Type", format!("application/vnd.interoperability.{}+json;version={}", req.resource, req.request_api_version))
            .method(req.method)
            .body(body)
            .unwrap()
    )
}

#[cfg(feature = "fsp_http")]
impl From<FspiopRequest> for http::Request<hyper::body::Body> {
    fn from(req: FspiopRequest) -> hyper::Request<hyper::body::Body> {
        // For now, matching on method to build the body (or not) seems to be working. If it breaks
        // down, it should be possible to match on the body type. I.e.
        // match req.body {
        //     FspiopRequestBody::TransferPrepare(transfer_prepare_request_body) => { .. build body }
        // }
        let body = match req.method {
            FspiopMethod::GET => "".to_string(),
            // Cheeky: because we configured serde to serialize the FspiopRequestBody as untagged, the
            // result of to_string here will consist only of the actual request body.
            // TODO: It might be better to implement From<FspiopRequestBody> for hyper::body::Body
            FspiopMethod::PUT | FspiopMethod::POST => serde_json::to_string(&req.body).unwrap(),
        };
        let accept = req.accept_api_versions
            .iter()
            .map(|v| format!("application/vnd.interoperability.{}+json;version={}", req.resource, v))
            .collect::<Vec<String>>()
            .join(",");

        // TODO: usage of unwrap() on the result of the builder tells us that we should replace
        // many of our types that can _only_ accept ASCII [[32-126], [128-255]].
        // https://docs.rs/http/0.2.4/http/header/struct.HeaderValue.html#method.from_bytes
        // In fact, here we're really assuming that none of these values contain non-printable
        // ASCII characters.
        // TODO: the only differences between this request builder and the http request builder in
        // to_http_request are in the body and the uri. Here the body is hyper::Body::from(body) and the
        // URI is relative. It _might_ make sense to deduplicate this stuff.
        hyper::Request::builder()
            .uri(req.path.clone())
            // .header("Date", req.date.unwrap_or(Utc::now()).to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            // The implementation of ml-api-adapter says:
            //   \"date\" must be in ddd, D MMM YYYY H:mm:ss [GMT] format
            // In practice, it requires the GMT string. And does not accept other timezones (this may
            // be intentional, but when the implementation clearly contradicts the error message, it
            // becomes more difficult to be certain). Ostensibly, it follows the RFC here:
            // https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2
            // TODO: this should be fixed upstream
            .header("Date", req.date.unwrap_or(Utc::now()).format("%a, %d %b %Y %T GMT").to_string())
            .header("FSPIOP-URI", req.path.to_string())
            .header("FSPIOP-HTTP-Method", req.method.to_string())
            .header("FSPIOP-Source", req.source.to_string())
            .header("FSPIOP-Destination", req.destination.to_string())
            .header("Accept", accept)
            .header("Content-Type", format!("application/vnd.interoperability.{}+json;version={}", req.resource, req.request_api_version))
            .method(req.method)
            .body(hyper::Body::from(body))
            .unwrap()
    }
}
