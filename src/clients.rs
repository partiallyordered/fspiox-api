use crate::transfer;
use hyper::client::conn;
use hyper::body::Body;
use crate::ErrorResponse;
use async_trait::async_trait;

pub enum Error {
    ConnectError(String),
    ConnectionError(String),
    MojaloopApiError(ErrorResponse),
    InvalidResponseBody(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
trait FspiopClient {
    fn get_sender(&mut self) -> &mut conn::SendRequest<Body>;

    async fn send(&mut self, msg: MlApiAdapterRequest) -> Result<()> {
        // Right now, I think I need to do something like the following:
        // - Change FspiopRequest into a trait, implement that trait for the transfer prepare and
        //   fulfil request body types. This would mean checking everywhere FspiopRequest is used
        //   and ensuring this change would be achievable. Also POD beats traits.
        // - Implement From/Into FspiopRequest for the transfer prepare and fulfil request body
        //   types, and use
        //     msg: impl Into<FspiopRequest>
        //   or whatever, as the parameter to this function
        // In both cases, how do we restrict which requests are sent with which service at
        // compile-time? Perhaps in the former case we attach the service to the FspiopRequest then
        // lazy-connect? This is getting very tricky though, and trickiness will bite us later on.
        // Maybe each service could accept its own type, which is just a newtyped FspiopRequest,
        // and we could supply
        //   impl Into<MLApiAdapterRequest> for TransferPrepareRequestBody
        // etc. for these service modules. And if a user wants to create their own crazy impl so
        // they can send transfer prepares to the quoting service, so be it. So now we'd have to
        // implement
        //   impl Into<FspiopRequest> for TransferPrepareRequestBody
        //   impl Into<MlApiAdapterRequest> for TransferPrepareRequestBody
        // but the latter would just be a newtype-constructed instance of the FspiopRequest. We
        // also *might not need* to implement <FspiopRequest> at all if we implement
        // service-specific request types for all requests. I.e. there might be nowhere that would
        // use a general FspiopRequest type.
        let msg = match msg {
            MlApiAdapterRequest::TransferPrepare(m) => m.0,
            MlApiAdapterRequest::TransferFulfil(m) => m.0,
        };
        let resp = self.get_sender().send_request(msg.into()).await
            .map_err(|e| Error::ConnectionError(format!("{}", e)))?;

        // Got the response okay, need to check if we have an ML API error
        let (parts, body) = resp.into_parts();

        let body_bytes = hyper::body::to_bytes(body).await
            .map_err(|e| Error::ConnectionError(format!("{}", e)))?;

        // In case of an HTTP error response (response code > 399), attempt to deserialize a
        // Mojaloop API error from the response.
        if !parts.status.is_success() {
            serde_json::from_slice::<ErrorResponse>(&body_bytes)
                .map_or_else(
                    |e| Err(Error::InvalidResponseBody(
                            format!(
                                // TODO: is this an FSPIOP API error, or a Mojaloop error?
                                "Unhandled error parsing FSPIOP API error out of response {} {}",
                                std::str::from_utf8(&body_bytes).unwrap(),
                                e,
                            )
                        )
                    ),
                    |ml_api_err| Err(Error::MojaloopApiError(ml_api_err))
                )?
        }

        Ok(())
    }
}

pub struct MlApiAdapterClient {
    sender: conn::SendRequest<Body>,
}

async fn build_from_stream(
    stream: impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
) -> Result<conn::SendRequest<Body>> {
    let (sender, connection) = conn::Builder::new().handshake(stream).await
        .or_else(|e| Err(Error::ConnectError(e.to_string())))?;

    // spawn a task to poll the connection and drive the HTTP state
    tokio::spawn(async move {
        // TODO: should we be closing this manually when we're done? Perhaps using the .into_parts
        // or .parts method?
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    Ok(sender)
}

pub enum MlApiAdapterRequest {
    TransferPrepare(transfer::TransferPrepareRequest),
    TransferFulfil(transfer::TransferFulfilRequest),
}

impl FspiopClient for MlApiAdapterClient {
    fn get_sender(&mut self) -> &mut conn::SendRequest<Body> {
        &mut self.sender
    }
}

impl MlApiAdapterClient {
    pub async fn new(
        stream: impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
    ) -> Result<MlApiAdapterClient> {
        Ok(MlApiAdapterClient { sender: build_from_stream(stream).await? })
    }
}
