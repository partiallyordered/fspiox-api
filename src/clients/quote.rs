use hyper::client::conn;
use hyper::body::Body;
use crate::quote;
use crate::clients::clients::*;

pub struct Client {
    sender: conn::SendRequest<Body>,
}

pub enum Request {
    QuoteRequest(quote::QuoteRequest),
}

impl From<Request> for http::Request<hyper::Body> {
    fn from(item: Request) -> http::Request<hyper::Body> {
        match item {
            Request::QuoteRequest(i) => i.0.into(),
        }
    }
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

impl Client {
    pub async fn send(&mut self, msg: Request) -> Result<()> {
        match msg {
            Request::QuoteRequest(m) => client_send(&mut self.sender, m.0).await,
        }
    }
}
