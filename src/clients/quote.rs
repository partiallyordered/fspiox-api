use hyper::client::conn;
use hyper::body::Body;
use crate::quote;
use crate::clients::clients::*;

#[derive(Debug)]
pub struct Client {
    sender: conn::SendRequest<Body>,
}

impl FspiopClient for Client {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams =
        k8s::KubernetesParams {
            label: "app.kubernetes.io/name=quoting-service",
            container_name: "quoting-service",
            port: k8s::Port::Name("http-api"),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> Client {
        Client {
            sender
        }
    }
}

impl From<quote::QuoteRequest> for http::Request<hyper::Body> {
    fn from(item: quote::QuoteRequest) -> http::Request<hyper::Body> {
        item.0.into()
    }
}

pub trait QuoteRequest {}

impl QuoteRequest for quote::QuoteRequest {}

impl Client {
    pub async fn send<T: QuoteRequest>(&mut self, msg: T)
        -> Result<()>
    where
        T: QuoteRequest + std::fmt::Debug + Clone,
        http::Request<hyper::Body>: From<T>
    {
        request::<T, NoBody>(&mut self.sender, msg).await.and(Ok(()))
    }
}
