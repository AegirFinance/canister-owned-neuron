use core::{future::Future, pin::Pin};
use ic_agent::{
    agent::{AgentError, ReplicaV2Transport},
    export::Principal,
    RequestId,
};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use url::Url;

type AgentFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, AgentError>> + Send + 'a>>;

#[derive(Debug)]
pub struct OutboundHttpTransport {
    // Base url for the ic replica
    url: Url,
}

impl OutboundHttpTransport {
    pub fn create(s: &str) -> Result<Self, AgentError> {
        Ok(Self {
            url: Url::parse(s).map_err(|_| AgentError::InvalidReplicaUrl(s.to_string()))?,
        })
    }

    async fn execute(
        &self,
        method: HttpMethod,
        endpoint: &str,
        body: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, AgentError> {
        let url = self.url.join(endpoint)?;
        let host = url.host_str().ok_or(AgentError::TransportError(
            "Invalid transport HTTP url".into(),
        ))?;
        let mut host_header = host.clone().to_owned();
        host_header.push_str(":443");
        // prepare system http_request call
        let headers = vec![
            HttpHeader {
                name: "Host".to_string(),
                value: host_header,
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "ic-agent outbound-http-request".to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/cbor".to_string(),
            },
            // TODO: Do we need authorization here?
        ];
        let request = CanisterHttpRequestArgument {
            url: url.to_string(),
            method,
            body,
            max_response_bytes: None,
            transform: Some(TransformContext::new(transform, vec![])),
            headers,
        };
        match http_request(request).await {
            Ok((response,)) => Ok(response.body),
            Err((r, m)) => {
                // TODO: Better error handling
                let message = format!(
                    "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                );
                Err(AgentError::TransportError(message.into()))
            }
        }
    }
}

impl ReplicaV2Transport for OutboundHttpTransport {
    /// Sends an asynchronous request to a Replica. The Request ID is non-mutable and
    /// depends on the content of the envelope.
    ///
    /// This normally corresponds to the `/api/v2/canister/<effective_canister_id>/call` endpoint.
    fn call(
        &self,
        effective_canister_id: Principal,
        envelope: Vec<u8>,
        _request_id: RequestId,
    ) -> AgentFuture<()> {
        Box::pin(async move {
            let endpoint = format!("canister/{}/call", effective_canister_id.to_text());
            self.execute(HttpMethod::POST, &endpoint, Some(envelope))
                .await?;
            Ok(())
        })
    }

    /// Sends a synchronous request to a Replica. This call includes the body of the request message
    /// itself (envelope).
    ///
    /// This normally corresponds to the `/api/v2/canister/<effective_canister_id>/read_state` endpoint.
    fn read_state(
        &self,
        effective_canister_id: Principal,
        envelope: Vec<u8>,
    ) -> AgentFuture<Vec<u8>> {
        Box::pin(async move {
            let endpoint = format!("canister/{effective_canister_id}/read_state");
            self.execute(HttpMethod::POST, &endpoint, Some(envelope))
                .await
        })
    }

    /// Sends a synchronous request to a Replica. This call includes the body of the request message
    /// itself (envelope).
    ///
    /// This normally corresponds to the `/api/v2/canister/<effective_canister_id>/query` endpoint.
    fn query(&self, effective_canister_id: Principal, envelope: Vec<u8>) -> AgentFuture<Vec<u8>> {
        Box::pin(async move {
            let endpoint = format!("canister/{effective_canister_id}/query");
            self.execute(HttpMethod::POST, &endpoint, Some(envelope))
                .await
        })
    }

    /// Sends a status request to the Replica, returning whatever the replica returns.
    /// In the current spec v2, this is a CBOR encoded status message, but we are not
    /// making this API attach semantics to the response.
    fn status(&self) -> AgentFuture<Vec<u8>> {
        Box::pin(async move { self.execute(HttpMethod::GET, "status", None).await })
    }
}

fn transform(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response.clone();
    sanitized.headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];
    sanitized
}
