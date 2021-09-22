use crate::transfer;
use hyper::client::conn;
use hyper::body::Body;
use crate::ErrorResponse;
#[cfg(feature = "clients-kube")]
use async_trait::async_trait;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "clients-kube")]
    KubernetesError(k8s::Error),

    ConnectError(String),
    ConnectionError(String),
    // TODO: is this an FSPIOP API error, or a Mojaloop error? I.e. is this type defined in the
    // FSPIOP API spec, or does it exist only in the _Mojaloop_ implementation of that spec?
    MojaloopApiError(ErrorResponse),
    InvalidResponseBody(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "clients-kube")]
mod k8s {
    use kube::{Client, api::Api};
    use k8s_openapi::api::core::v1::Pod;
    use crate::clients::Result;
    use std::convert::TryFrom;

    #[derive(Debug, Clone)]
    pub enum Port {
        Name(String),
        Number(i32),
    }

    #[derive(Debug)]
    pub enum Error {
        UnableToLoadKubeconfig(String),
        ServicePortNotFound(Port, String),
        ClusterConnectionError(String),
        PodNotFound(String),
        UnexpectedPodImplementation(String),
        ServiceContainerNotFound(String, String),
        FailedToEstablishPortForward(String),
        PortForwardHttpConnectionFailed(String),
    }

    impl From<Error> for crate::clients::Error {
        fn from(e: Error) -> Self {
            Self::KubernetesError(e)
        }
    }

    #[derive(Debug, Clone)]
    pub struct KubernetesParams {
        pub label: &'static str,
        pub container_name: &'static str,
        pub port: Port,
    }

    pub async fn get_pods(
        kubeconfig: &Option<std::path::PathBuf>,
        namespace: &Option<String>,
    ) -> Result<Api<Pod>> {
        let client = match kubeconfig {
            Some(path) => {
                let custom_config = kube::config::Kubeconfig::read_from(path.as_path())
                    .map_err(|e| Error::UnableToLoadKubeconfig(e.to_string()))?;
                // TODO: expose some of this to the user?
                let options = kube::config::KubeConfigOptions {
                    context: None,
                    cluster: None,
                    user: None,
                };
                let config = kube::Config::from_custom_kubeconfig(custom_config, &options).await
                    .map_err(|e| Error::UnableToLoadKubeconfig(e.to_string()))?;
                Client::try_from(config)
                    .map_err(|e| Error::UnableToLoadKubeconfig(e.to_string()))?
            },
            None => Client::try_default().await
                .map_err(|e| Error::UnableToLoadKubeconfig(e.to_string()))?
        };
        Ok(
            match namespace {
                Some(ns) => Api::namespaced(client, &ns),
                None => Api::default_namespaced(client),
            }
        )
    }
}

async fn client_send(sender: &mut conn::SendRequest<Body>, msg: crate::FspiopRequest) -> Result<()> {
    let resp = sender.send_request(msg.into()).await
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

#[cfg_attr(feature = "clients-kube", async_trait)]
trait FspiopClient: Sized {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams;

    fn from_sender(sender: conn::SendRequest<Body>) -> Self;

    #[cfg(feature = "clients-kube")]
    async fn from_k8s_params(
        kubeconfig: &Option<std::path::PathBuf>,
        namespace: &Option<String>,
    ) -> Result<Self> {
        use k8s::{get_pods, Port, KubernetesParams, Error};
        use kube::api::ListParams;
        use std::convert::TryInto;

        let pods = get_pods(kubeconfig, namespace).await?;
        let KubernetesParams { label, container_name, port } = Self::K8S_PARAMS;
        let lp = ListParams::default().labels(&label);
        let pod = pods
            .list(&lp).await.map_err(|e| Error::ClusterConnectionError(e.to_string()))?
            .items.get(0).ok_or(Error::PodNotFound(label.to_string()))?.clone();
        let pod_name = pod.metadata.name.clone().unwrap();
        let pod_port = pod
            .spec
            .ok_or(Error::UnexpectedPodImplementation(pod_name.clone()))?
            .containers.iter().find(|c| c.name == container_name)
            .ok_or(Error::ServiceContainerNotFound(container_name.to_string(), pod_name.clone()))?
            .ports.as_ref().ok_or(Error::ServicePortNotFound(port.clone(), container_name.to_string()))?
            .iter()
            .find(|p| {
                match &port {
                    Port::Name(port_name) => p.name.as_ref().map_or(false, |name| name == port_name),
                    Port::Number(port_num) => p.container_port == *port_num,
                }
            })
            .ok_or(Error::ServicePortNotFound(port, container_name.to_string()))?.clone();

        // TODO: somehow, when establishing port-forward fails because the pod is still coming up, this
        // doesn't cause the application to fail.
        let mut pf = pods.portforward(
            &pod_name,
            &[pod_port.container_port.try_into().unwrap()]
        ).await.map_err(|e| Error::FailedToEstablishPortForward(e.to_string()))?;
        let mut ports = pf.ports().unwrap();
        let result = ports[0].stream().unwrap();
        let (sender, connection) = conn::Builder::new().handshake(result).await
            .map_err(|e| Error::PortForwardHttpConnectionFailed(e.to_string()))?;

        // spawn a task to poll the connection and drive the HTTP state
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error in connection: {}", e);
            }
        });

        Ok(Self::from_sender(sender))
    }
}

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

#[cfg_attr(feature = "clients-kube", async_trait)]
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
