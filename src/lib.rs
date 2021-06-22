// TODO: handle api version 1.0 and 1.1. How? Just a different crate for 1.1 which mostly imports
// 1.0? And re-exports 1.0? A single crate that exports both? (Probably the latter..).


use serde::Serialize;
pub mod transfer;
pub mod common;
pub mod quote;
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

#[derive(Debug, derive_more::Display)]
pub enum ApiVersion {
    #[display(fmt = "1.0")]
    V1pt0,
    #[display(fmt = "1.1")]
    V1pt1,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum FspiopRequestBody {
    TransferFulfil (transfer::TransferFulfilRequestBody),
    TransferPrepare (transfer::TransferPrepareRequestBody),
    PostQuotes (quote::QuoteRequestBody),
    NoBody,
}

#[derive(Debug, strum_macros::ToString)]
pub enum FspiopMethod {
    GET,
    PUT,
    POST,
    // TODO: PATCH
    // (this will probably occur as part of a broader v1.1 effort)
}

#[cfg(feature = "http")]
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
#[derive(Debug, strum_macros::Display)]
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
#[derive(Debug)]
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
    pub path: String,
    // TODO:
    // - X-Forwarded-For
    // - FSPIOP-Encryption
    // - FSPIOP-Signature
    // - FSPIOP-URI
    // See: https://github.com/mojaloop/mojaloop-specification/blob/d9393fa490ec825689ea5f325ac38e97d06956cf/fspiop-api/documents/API%20Definition%20v1.1.md#3211-http-request-header-fields
}

// TODO: probably move this to the transfer crate, and everything else it depends on to the common
// crate
pub fn build_post_quotes(
    payer_fsp: common::FspId,
    payee_fsp: common::FspId,
    amount: common::Amount,
    currency: common::Currency,
) -> FspiopRequest {
    FspiopRequest {
        source: payer_fsp.clone(),
        destination: payee_fsp.clone(),
        path: "/quotes".to_string(),
        resource: FspiopResource::Quotes,
        method: FspiopMethod::POST,
        request_api_version: ApiVersion::V1pt0,
        accept_api_versions: vec![ApiVersion::V1pt0],
        date: Some(Utc::now()),
        body: FspiopRequestBody::PostQuotes(
            quote::QuoteRequestBody {
                quote_id: quote::QuoteId(common::CorrelationId::new()),
                transaction_id: quote::TransactionId(common::CorrelationId::new()),
                payer: quote::Payer {
                    party_id_info: quote::PartyIdInfo {
                        fsp_id: payer_fsp.clone(),
                        party_id_type: quote::PartyIdType::Msisdn,
                        party_identifier: "1234567890".to_string(),
                    },
                    personal_info: quote::PersonalInfo {
                        complex_name: quote::ComplexName {
                            first_name: "Mats".to_string(),
                            last_name: "Hagman".to_string(),
                        },
                    },
                },
                payee: quote::Payee {
                    party_id_info: quote::PartyIdInfo {
                        fsp_id: payee_fsp.clone(),
                        party_id_type: quote::PartyIdType::Msisdn,
                        party_identifier: "1234567890".to_string(),
                    },
                },
                amount_type: quote::AmountType::Send,
                amount: common::Money {
                    currency,
                    amount,
                },
                transaction_type: quote::TransactionType {
                    scenario: quote::Scenario::Transfer,
                    initiator: quote::Initiator::Payer,
                    initiator_type: quote::InitiatorType::Consumer,
                },
                note: "Created by fspiox-api".to_string(),
                expiration: Utc::now().checked_add_signed(chrono::Duration::hours(1)).unwrap(),
            }
        )
    }
}

// TODO: probably move this to the transfer crate, and everything else it depends on to the common
// crate
pub fn build_transfer_prepare(
    payer_fsp: common::FspId,
    payee_fsp: common::FspId,
    amount: common::Amount,
    currency: common::Currency,
    transfer_id: Option<transfer::TransferId>,
) -> FspiopRequest {
    FspiopRequest {
        source: payer_fsp.clone(),
        destination: payee_fsp.clone(),
        path: "/transfers".to_string(),
        resource: FspiopResource::Transfers,
        method: FspiopMethod::POST,
        request_api_version: ApiVersion::V1pt0,
        accept_api_versions: vec![ApiVersion::V1pt0],
        date: Some(Utc::now()),
        body: FspiopRequestBody::TransferPrepare(
            transfer::TransferPrepareRequestBody {
                transfer_id: transfer_id.unwrap_or(transfer::TransferId(common::CorrelationId::new())),
                payer_fsp,
                payee_fsp,
                amount: common::Money {
                    amount,
                    currency,
                },
                ilp_packet: "ilp_packet".to_string(),
                // /^[A-Za-z0-9-_]{43}$/"
                condition: "_ilp_condition_ilp_condition_ilp_condition_".to_string(),
                expiration: Utc::now().checked_add_signed(chrono::Duration::hours(1)).unwrap(),
            }
        )
    }
}

#[cfg(feature = "fsp_http")]
pub fn to_hyper_request(req: FspiopRequest) -> Result<hyper::Request<hyper::body::Body>, http::Error>
{
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

    hyper::Request::builder()
        .uri(req.path.clone())
        // .header("Date", req.date.unwrap_or(Utc::now()).to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        // The implementation of ml-api-adapter says:
        //   \"date\" must be in ddd, D MMM YYYY H:mm:ss [GMT] format
        // In practice, it requires the GMT string. And does not accept other timezones (this may
        // be intentional, but when the implementation clearly contradicts the error message, it
        // becomes more difficult to be certain). Ostensibly, it follows the RFC here:
        // https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2
        .header("Date", req.date.unwrap_or(Utc::now()).format("%a, %d %b %Y %T GMT").to_string())
        .header("FSPIOP-URI", req.path)
        .header("FSPIOP-HTTP-Method", req.method.to_string())
        .header("FSPIOP-Source", req.source)
        .header("FSPIOP-Destination", req.destination)
        .header("Accept", accept)
        .header("Content-Type", format!("application/vnd.interoperability.{}+json;version={}", req.resource, req.request_api_version))
        .method(req.method)
        .body(hyper::Body::from(body))
}
