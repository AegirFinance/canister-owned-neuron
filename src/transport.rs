use ic_agent::{
    agent::{ AgentFuture, ReplicaV2Transport, RequestId },
    export::Principal,
};

#[derive(Debug, Default)]
pub struct OutboundHTTPTransport {
    // Base url for the ic replica
    url: String,
}

impl OutboundHTTPTransport {
    pub fn create(url: String) -> Self {
        Self { url }
    }
}

impl ReplicaV2Transport for OutboundHTTPTransport {
    /// Sends an asynchronous request to a Replica. The Request ID is non-mutable and
    /// depends on the content of the envelope.
    ///
    /// This normally corresponds to the `/api/v2/canister/<effective_canister_id>/call` endpoint.
    fn call(
        &self,
        effective_canister_id: Principal,
        envelope: Vec<u8>,
        request_id: RequestId,
    ) -> AgentFuture<()> {
        todo!()
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
        todo!()
    }

    /// Sends a synchronous request to a Replica. This call includes the body of the request message
    /// itself (envelope).
    ///
    /// This normally corresponds to the `/api/v2/canister/<effective_canister_id>/query` endpoint.
    fn query(&self, effective_canister_id: Principal, envelope: Vec<u8>) -> AgentFuture<Vec<u8>> {
        todo!()
    }

    /// Sends a status request to the Replica, returning whatever the replica returns.
    /// In the current spec v2, this is a CBOR encoded status message, but we are not
    /// making this API attach semantics to the response.
    fn status(&self) -> AgentFuture<Vec<u8>> {
        todo!()
    }
}
