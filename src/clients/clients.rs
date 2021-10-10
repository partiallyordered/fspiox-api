use serde::{Deserialize, Serialize, Serializer};
use hyper::client::conn;
use hyper::body::Body;
use crate::ErrorResponse;
use thiserror::Error;
#[cfg(feature = "clients-kube")] use async_trait::async_trait;

// A type that deserializes from and to an empty string. Many Mojaloop APIs expect and return an
// empty response, even though as JSON they should accept and return {} or [].
#[derive(Debug, Clone, Copy)]
pub struct NoBody;

impl Serialize for NoBody {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("")
    }
}

impl<'de> Deserialize<'de> for NoBody {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<NoBody, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() == 0 {
            Ok(NoBody {})
        } else {
            Err(serde::de::Error::custom(format!("Expected empty response, received {}", s)))
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "clients-kube")]
    #[error("{0}")]
    KubernetesError(k8s::Error),

    #[error("Error in client HTTP connection: {0}")]
    HttpConnectionError(String),
    // TODO: is this an FSPIOP API error, or a Mojaloop error? I.e. is this type defined in the
    // FSPIOP API spec, or does it exist only in the _Mojaloop_ implementation of that spec?
    #[error("Unhandled error parsing FSPIOP API error out of response {1}. Response: {0}")]
    InvalidResponseBody(String, String),
    #[error("Mojaloop API error response returned: {0}")]
    MojaloopApiError(ErrorResponse),
    #[error("Failed to deserialize FSPIOP response. Error: {0}. Body: {1}.")]
    FailureToDeserializeResponseBody(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "clients-kube")]
pub mod k8s {
    use kube::{Client, api::Api};
    use k8s_openapi::api::core::v1::Pod;
    use super::Result;
    use thiserror::Error;
    use derive_more::Display;
    use crate::clients::{transfer, quote, FspiopClient};

    #[derive(Debug, Clone, Display)]
    pub enum Port {
        Name(&'static str),
        Number(i32),
    }

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("Unable to load kubeconfig file: {0}")]
        UnableToLoadKubeconfig(String),
        #[error("Unable to find service port {0} on container {1}")]
        ServicePortNotFound(Port, String),
        #[error("Unable to retrieve pod list, possible connection error: {0}")]
        ClusterConnectionError(String),
        #[error("Unable to find pod with label {0}")]
        PodNotFound(String),
        #[error("Pod {0} does not appear to have a pod spec")]
        PodSpecNotFound(String),
        #[error("Pod {0} does not appear to contain a container {1}")]
        ServiceContainerNotFound(String, String),
        #[error("Unable to establish port-forward: {0}")]
        FailedToEstablishPortForward(String),
        #[error("Unable to establish HTTP connection over pod port-forward: {0}")]
        PortForwardHttpConnectionFailed(String),
    }

    impl From<Error> for super::Error {
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

    pub struct Clients {
        pub transfer: transfer::Client,
        pub quote: quote::Client,
    }

    pub async fn ensure_client(
        client: Option<kube::client::Client>,
    ) -> Result<kube::Client> {
        Ok(
            match client {
                Some(c) => c,
                None => Client::try_default().await
                    .map_err(|e| Error::UnableToLoadKubeconfig(e.to_string()))?
            }
        )
    }

    pub async fn get_all_from_k8s(
        client: Option<kube::client::Client>,
        namespace: &Option<String>,
    ) -> Result<Clients> {
        let client = ensure_client(client).await?;
        let (transfer, quote) = tokio::try_join!(
            transfer::Client::from_k8s_params(Some(client.clone()), namespace),
            quote::Client::from_k8s_params(Some(client), namespace),
        )?;
        Ok(Clients { transfer, quote })
    }

    pub async fn port_forward_stream(
        client: Option<kube::client::Client>,
        namespace: &Option<String>,
        params: KubernetesParams,
    ) -> Result<(impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin)> {
        use kube::api::ListParams;
        use std::convert::TryInto;

        let client = ensure_client(client).await?;
        let pods: kube::Api<Pod> = match namespace {
            Some(ns) => Api::namespaced(client, ns),
            None => Api::default_namespaced(client),
        };

        let KubernetesParams { label, container_name, port } = params;

        let lp = ListParams::default().labels(&label);
        let pod = pods
            .list(&lp).await.map_err(|e| Error::ClusterConnectionError(e.to_string()))?
            .items.get(0).ok_or(Error::PodNotFound(label.to_string()))?.clone();
        let pod_name = pod.metadata.name.clone().unwrap();
        let pod_port = pod
            .spec
            .ok_or(Error::PodSpecNotFound(pod_name.clone()))?
            .containers.iter().find(|c| c.name == container_name)
            .ok_or(Error::ServiceContainerNotFound(pod_name.clone(), container_name.to_string()))?
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
        Ok(ports[0].stream().unwrap())
    }
}

#[derive(Debug)]
pub struct ResponseBody<RespBody: serde::de::DeserializeOwned> {
    body: hyper::Body,
    phantom: std::marker::PhantomData<RespBody>,
}

impl<RespBody: serde::de::DeserializeOwned> ResponseBody<RespBody> {
    pub async fn des(self) -> Result<RespBody> {
        let ResponseBody { body, .. } = self;
        let body_bytes = hyper::body::to_bytes(body).await
            .map_err(|e| Error::HttpConnectionError(e.to_string()))?;
        serde_json::from_slice::<RespBody>(&body_bytes).map_err(|e|
            Error::FailureToDeserializeResponseBody(
                e.to_string(),
                std::str::from_utf8(&body_bytes).unwrap().to_string(),
            )
        )
    }
}

// TODO: could have some request_no_body for no-body responses, so it's not possible for the user
// to request a body from a response that shouldn't contain one. And get rid of NoBody?
pub async fn request<ReqBody, RespBody>(
    sender: &mut conn::SendRequest<Body>,
    msg: ReqBody,
) -> Result<ResponseBody<RespBody>> where
    ReqBody: std::fmt::Debug + Clone,
    RespBody: serde::de::DeserializeOwned,
    http::Request<hyper::Body>: From<ReqBody>,
{
    let resp = sender.send_request(msg.clone().into()).await
        .map_err(|e| Error::HttpConnectionError(e.to_string()))?;

    // Got the response okay, need to check if we have an ML API error
    let (parts, body) = resp.into_parts();

    if parts.status.is_success() {
        Ok(ResponseBody { body, phantom: std::marker::PhantomData::<RespBody> })
    } else {
        let body_bytes = hyper::body::to_bytes(body).await
            .map_err(|e| Error::HttpConnectionError(e.to_string()))?;
        // In case of an HTTP error response (response code > 399), attempt to deserialize a
        // Mojaloop API error from the response.
        serde_json::from_slice::<ErrorResponse>(&body_bytes)
            .map_or_else(
                |e| Err(Error::InvalidResponseBody(
                        std::str::from_utf8(&body_bytes).unwrap().to_string(),
                        e.to_string(),
                    )
                ),
                // TODO: is this an FSPIOP API error, or a Mojaloop error?
                |ml_api_err| Err(Error::MojaloopApiError(ml_api_err))
            )
    }
}

#[cfg_attr(feature = "clients-kube", async_trait)]
pub trait FspiopClient: Sized {
    fn from_sender(sender: conn::SendRequest<Body>) -> Self;

    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams;

    #[cfg(feature = "clients-kube")]
    async fn from_k8s_defaults() -> Result<Self> {
        Self::from_k8s_params(None, &None).await
    }

    #[cfg(feature = "clients-kube")]
    async fn from_k8s_params(
        client: Option<kube::client::Client>,
        namespace: &Option<String>,
    ) -> Result<Self> {
        let stream = k8s::port_forward_stream(
            client,
            namespace,
            Self::K8S_PARAMS,
        ).await?;

        let (sender, connection) = conn::Builder::new().handshake(stream).await
            .map_err(|e| k8s::Error::PortForwardHttpConnectionFailed(e.to_string()))?;

        // spawn a task to poll the connection and drive the HTTP state
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error in connection: {}", e);
            }
        });

        Ok(Self::from_sender(sender))
    }
}
