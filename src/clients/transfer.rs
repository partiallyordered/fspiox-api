use hyper::client::conn;
use hyper::body::Body;
use crate::transfer;
use crate::clients::clients::*;

#[derive(Debug)]
pub struct Client {
    sender: conn::SendRequest<Body>,
}

#[derive(Debug)]
pub enum Request {
    TransferPrepare(transfer::TransferPrepareRequest),
    TransferFulfil(transfer::TransferFulfilRequest),
}

impl From<transfer::TransferPrepareRequest> for Request {
    fn from(i: transfer::TransferPrepareRequest) -> Request {
        Request::TransferPrepare(i)
    }
}

impl From<transfer::TransferFulfilRequest> for Request {
    fn from(i: transfer::TransferFulfilRequest) -> Request {
        Request::TransferFulfil(i)
    }
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
    pub async fn send(&mut self, msg: Request) -> Result<()> {
        use crate::FspiopRequest;
        match msg {
            Request::TransferFulfil(m) => request::<FspiopRequest, NoBody>(&mut self.sender, m.0).await.and(Ok(())),
            Request::TransferPrepare(m) => request::<FspiopRequest, NoBody>(&mut self.sender, m.0).await.and(Ok(())),
        }
    }
}
