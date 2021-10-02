use hyper::client::conn;
use hyper::body::Body;
use crate::transfer;
use crate::clients::clients::*;

#[derive(Debug)]
pub struct Client {
    sender: conn::SendRequest<Body>,
}

impl FspiopClient for Client {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams =
        k8s::KubernetesParams {
            label: "app.kubernetes.io/name=ml-api-adapter-service",
            container_name: "ml-api-adapter-service",
            port: k8s::Port::Number(3000),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> Client {
        Client {
            sender
        }
    }
}

pub trait TransferRequest {}

impl TransferRequest for transfer::TransferPrepareRequest {}
impl TransferRequest for transfer::TransferFulfilRequest {}

impl From<transfer::TransferFulfilRequest> for http::Request<hyper::Body> {
    fn from(item: transfer::TransferFulfilRequest) -> http::Request<hyper::Body> {
        item.0.into()
    }
}

impl From<transfer::TransferPrepareRequest> for http::Request<hyper::Body> {
    fn from(item: transfer::TransferPrepareRequest) -> http::Request<hyper::Body> {
        item.0.into()
    }
}

impl Client {
    pub async fn send<T: TransferRequest>(&mut self, msg: T)
        -> Result<()>
    where
        T: TransferRequest + std::fmt::Debug + Clone,
        http::Request<hyper::Body>: From<T>
    {
        request::<T, NoBody>(&mut self.sender, msg).await.and(Ok(()))
    }
}
