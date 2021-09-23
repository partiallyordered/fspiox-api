use hyper::client::conn;
use hyper::body::Body;
use crate::transfer;
use crate::clients::clients::*;

pub struct MlApiAdapterClient {
    sender: conn::SendRequest<Body>,
}

pub enum MlApiAdapterRequest {
    TransferPrepare(transfer::TransferPrepareRequest),
    TransferFulfil(transfer::TransferFulfilRequest),
}

impl From<MlApiAdapterRequest> for http::Request<hyper::Body> {
    fn from(item: MlApiAdapterRequest) -> http::Request<hyper::Body> {
        match item {
            MlApiAdapterRequest::TransferFulfil(i) => i.0.into(),
            MlApiAdapterRequest::TransferPrepare(i) => i.0.into(),
        }
    }
}

impl FspiopClient for MlApiAdapterClient {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams =
        k8s::KubernetesParams {
            label: "app.kubernetes.io/name=ml-api-adapter-service",
            container_name: "ml-api-adapter-service",
            port: k8s::Port::Number(3000),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> MlApiAdapterClient {
        MlApiAdapterClient {
            sender
        }
    }
}

impl MlApiAdapterClient {
    pub async fn send(&mut self, msg: MlApiAdapterRequest) -> Result<()> {
        match msg {
            MlApiAdapterRequest::TransferFulfil(m) => client_send(&mut self.sender, m.0).await,
            MlApiAdapterRequest::TransferPrepare(m) => client_send(&mut self.sender, m.0).await,
        }
    }
}
