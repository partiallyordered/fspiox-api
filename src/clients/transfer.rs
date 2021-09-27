use hyper::client::conn;
use hyper::body::Body;
use crate::transfer;
use crate::clients::clients::*;

pub struct Client {
    sender: conn::SendRequest<Body>,
}

pub enum Request {
    TransferPrepare(transfer::TransferPrepareRequest),
    TransferFulfil(transfer::TransferFulfilRequest),
}

impl From<Request> for http::Request<hyper::Body> {
    fn from(item: Request) -> http::Request<hyper::Body> {
        match item {
            Request::TransferFulfil(i) => i.0.into(),
            Request::TransferPrepare(i) => i.0.into(),
        }
    }
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

impl Client {
    pub async fn send(&mut self, msg: Request) -> Result<NoBody> {
        match msg {
            Request::TransferFulfil(m) => request(&mut self.sender, m.0).await,
            Request::TransferPrepare(m) => request(&mut self.sender, m.0).await,
        }
    }
}
