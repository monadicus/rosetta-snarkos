use std::fmt::Debug;

use mentat_server::{
    conf::Configuration,
    reqwest,
    serde::{de::DeserializeOwned, Serialize},
    serde_json::{self, json, Value},
};
use mentat_types::{MapErrMentat, Result};

use crate::node::Config;

/// The `RpcCaller` struct is a wrapper to hold a rpc caller instance
/// that holds a request client and the url for the RPC.
#[derive(Debug, Clone)]
pub struct SnarkosCaller {
    /// The request client.
    pub client: reqwest::Client,
    /// The RPC url.
    pub node_rpc_url: reqwest::Url,
}

impl From<Configuration<Config>> for SnarkosCaller {
    fn from(conf: Configuration<Config>) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: Config::build_url(&conf),
        }
    }
}

impl SnarkosCaller {
    /// Makes the RPC call returning the expected output given the input type.
    pub async fn rpc_call<O: DeserializeOwned + Debug>(&self, req: Jrpc) -> Result<O> {
        let resp = self
            .client
            .post(self.node_rpc_url.clone())
            .json(&req)
            .send()
            .await?;

        let resp_text = resp.text().await?;
        let response_type = serde_json::from_str::<O>(&resp_text)
            .merr(|e| format!("failed to serialize response: `{e}`\ntext: `{resp_text}`"))?;
        Ok(response_type)
    }
}

// TODO this needs to be removed.
// See https://github.com/AleoHQ/snarkOS/blob/testnet3/node/rest/src/routes.rs
#[derive(Debug, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct Jrpc {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<Value>,
}

impl Jrpc {
    pub fn new<P: Serialize>(method: &str, params: Vec<P>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params: params.iter().map(|p| json!(p)).collect(),
        }
    }
}
