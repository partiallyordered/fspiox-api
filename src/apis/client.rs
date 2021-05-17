use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  authorizations_api: Box<::apis::AuthorizationsApi>,
  bulk_quotes_api: Box<::apis::BulkQuotesApi>,
  bulk_transfers_api: Box<::apis::BulkTransfersApi>,
  participants_api: Box<::apis::ParticipantsApi>,
  parties_api: Box<::apis::PartiesApi>,
  quotes_api: Box<::apis::QuotesApi>,
  transaction_requests_api: Box<::apis::TransactionRequestsApi>,
  transactions_api: Box<::apis::TransactionsApi>,
  transfers_api: Box<::apis::TransfersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      authorizations_api: Box::new(::apis::AuthorizationsApiClient::new(rc.clone())),
      bulk_quotes_api: Box::new(::apis::BulkQuotesApiClient::new(rc.clone())),
      bulk_transfers_api: Box::new(::apis::BulkTransfersApiClient::new(rc.clone())),
      participants_api: Box::new(::apis::ParticipantsApiClient::new(rc.clone())),
      parties_api: Box::new(::apis::PartiesApiClient::new(rc.clone())),
      quotes_api: Box::new(::apis::QuotesApiClient::new(rc.clone())),
      transaction_requests_api: Box::new(::apis::TransactionRequestsApiClient::new(rc.clone())),
      transactions_api: Box::new(::apis::TransactionsApiClient::new(rc.clone())),
      transfers_api: Box::new(::apis::TransfersApiClient::new(rc.clone())),
    }
  }

  pub fn authorizations_api(&self) -> &::apis::AuthorizationsApi{
    self.authorizations_api.as_ref()
  }

  pub fn bulk_quotes_api(&self) -> &::apis::BulkQuotesApi{
    self.bulk_quotes_api.as_ref()
  }

  pub fn bulk_transfers_api(&self) -> &::apis::BulkTransfersApi{
    self.bulk_transfers_api.as_ref()
  }

  pub fn participants_api(&self) -> &::apis::ParticipantsApi{
    self.participants_api.as_ref()
  }

  pub fn parties_api(&self) -> &::apis::PartiesApi{
    self.parties_api.as_ref()
  }

  pub fn quotes_api(&self) -> &::apis::QuotesApi{
    self.quotes_api.as_ref()
  }

  pub fn transaction_requests_api(&self) -> &::apis::TransactionRequestsApi{
    self.transaction_requests_api.as_ref()
  }

  pub fn transactions_api(&self) -> &::apis::TransactionsApi{
    self.transactions_api.as_ref()
  }

  pub fn transfers_api(&self) -> &::apis::TransfersApi{
    self.transfers_api.as_ref()
  }


}
