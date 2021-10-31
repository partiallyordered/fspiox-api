use fspiox_api::clients::{transfer, quote, FspiopClient};

#[tokio::test]
async fn test_transfer_client_connect() {
    transfer::Client::from_k8s_defaults().await.unwrap();
}

#[tokio::test]
async fn test_quote_client_connect() {
    quote::Client::from_k8s_defaults().await.unwrap();
}

#[tokio::test]
async fn test_transfer_client() {
    use fspiox_api::transfer::*;
    use fspiox_api::{Amount, FspId, Currency};
    use std::str::FromStr;
    let mut client = transfer::Client::from_k8s_defaults().await.unwrap();
    let transfer_generator = std::iter::from_fn(|| Some(TransferPrepareRequest::new(
        FspId::from("payer_fsp").unwrap(),
        FspId::from("payee_fsp").unwrap(),
        Amount::from_str("123.34").unwrap(),
        Currency::BDT,
        None,
    )));
    for t in transfer_generator.take(1000) {
        client.send(t).await.expect("send okay");
    }
}

#[tokio::test]
async fn test_quote_client() {
    use fspiox_api::quote::*;
    use fspiox_api::{Amount, FspId, Currency};
    use std::str::FromStr;
    let mut client = quote::Client::from_k8s_defaults().await.unwrap();
    let quote_generator = std::iter::from_fn(|| Some(QuoteRequest::new(
        FspId::from("payer_fsp").unwrap(),
        FspId::from("payee_fsp").unwrap(),
        Amount::from_str("123.34").unwrap(),
        Currency::BDT,
    )));
    for q in quote_generator.take(1000) {
        client.send(q).await.expect("send okay");
    }
}
