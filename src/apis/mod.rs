use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod authorizations_api;
pub use self::authorizations_api::{ AuthorizationsApi, AuthorizationsApiClient };
mod bulk_quotes_api;
pub use self::bulk_quotes_api::{ BulkQuotesApi, BulkQuotesApiClient };
mod bulk_transfers_api;
pub use self::bulk_transfers_api::{ BulkTransfersApi, BulkTransfersApiClient };
mod participants_api;
pub use self::participants_api::{ ParticipantsApi, ParticipantsApiClient };
mod parties_api;
pub use self::parties_api::{ PartiesApi, PartiesApiClient };
mod quotes_api;
pub use self::quotes_api::{ QuotesApi, QuotesApiClient };
mod transaction_requests_api;
pub use self::transaction_requests_api::{ TransactionRequestsApi, TransactionRequestsApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod transfers_api;
pub use self::transfers_api::{ TransfersApi, TransfersApiClient };

pub mod configuration;
pub mod client;
