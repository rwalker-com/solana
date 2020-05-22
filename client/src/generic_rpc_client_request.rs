use crate::{client_error::Result, rpc_request::RpcRequest};

<<<<<<< HEAD:client/src/generic_rpc_client_request.rs
pub(crate) trait GenericRpcClientRequest {
    fn send(
        &self,
        request: RpcRequest,
        params: serde_json::Value,
        retries: usize,
    ) -> Result<serde_json::Value>;
=======
pub trait RpcSender {
    fn send(&self, request: RpcRequest, params: serde_json::Value) -> Result<serde_json::Value>;
>>>>>>> 4779858dd... Clean up RPCClient retry handling: only retry on 429, after a little sleep (#10182):client/src/rpc_sender.rs
}
