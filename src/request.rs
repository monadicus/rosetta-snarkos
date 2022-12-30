use std::fmt::Debug;

use mentat_server::{
    conf::Configuration,
    reqwest,
    serde::de::DeserializeOwned,
    serde_json::{self, json, Value},
};
use mentat_types::{MapErrMentat, MentatError, Result};

use crate::{node::Config, SnarkosTransaction};

pub enum Request<'a> {
    Get(&'a str),
    Post(&'a str, Value),
}

impl<'a> Request<'a> {
    /// Returns the peers connected to the node.
    pub const ALL_PEERS: Self = Self::Get("api/testnet3/peers/all");
    /// Returns the metrics for peers connected to the node.
    pub const ALL_PEERS_METRICS: Self = Self::Get("api/testnet3/peers/all/metrics");
    /// Returns the list of current beacons.
    pub const BEACONS: Self = Self::Get("api/testnet3/beacons");
    /// Returns the number of peers connected to the node.
    pub const COUNT_PEERS: Self = Self::Get("api/testnet3/peers/count");
    /// Returns the latest block.
    pub const LATEST_BLOCK: Self = Self::Get("api/testnet3/latest/block");
    /// Returns the latest block hash.
    pub const LATEST_HASH: Self = Self::Get("api/testnet3/latest/hash");
    /// Returns the latest block height.
    pub const LATEST_HEIGHT: Self = Self::Get("api/testnet3/latest/height");
    /// Returns the latest state root.
    pub const LATEST_STATE_ROOT: Self = Self::Get("api/testnet3/latest/stateRoot");
    /// Returns the transactions in the memory pool.
    pub const MEM_POOL_TRANSACTIONS: Self = Self::Get("api/testnet3/memoryPool/transactions");
    pub const NODE_ADDRESS: Self = Self::Get("api/testnet3/node/address");

    /// Returns the block for the given block height.
    pub fn get_block_by_height(height: usize) -> Self {
        let ep = format!("api/testnet3/block/{height}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the block for the given block hash.
    pub fn get_block_by_hash(hash: &str) -> Self {
        let ep = format!("api/testnet3/block/{hash}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the block height for the given block hash.
    pub fn get_block_height_by_hash(hash: &str) -> Self {
        let ep = format!("api/testnet3/height/{hash}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the transactions for the given block height.
    pub fn get_block_transactions(height: usize) -> Self {
        let ep = format!("api/testnet3/block/{height}/transactions");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the blocks for the given block range.
    pub fn get_blocks(start: usize, end: usize) -> Self {
        let ep = format!("api/testnet3/blocks?start={start}&end={end}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the transaction for the given transaction ID.
    pub fn get_transaction(id: &str) -> Self {
        let ep = format!("api/testnet3/transaction/{id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the program for the given program ID.
    pub fn get_program(id: &str) -> Self {
        let ep = format!("api/testnet3/program/{id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the state path for the given commitment.
    pub fn get_state_path(commitment: &str) -> Self {
        let ep = format!("api/testnet3/statePath/{commitment}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the block hash that contains the given `transaction ID`.
    pub fn find_block_hash(transaction_id: &str) -> Self {
        let ep = format!("api/testnet3/find/blockHash/{transaction_id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the transaction ID that contains the given `program ID`.
    pub fn find_deployment_id(program: &str) -> Self {
        let ep = format!("api/testnet3/find/deploymentID/{program}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the transaction ID that contains the given `transition ID`.
    pub fn find_transaction_id(transition_id: &str) -> Self {
        let ep = format!("api/testnet3/find/transactionID/{transition_id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Returns the transition ID that contains the given `input ID` or `output
    /// ID`.
    pub fn find_transition_id(id: &str) -> Self {
        let ep = format!("api/testnet3/find/transitionID/{id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    /// Broadcasts the transaction to the ledger.
    pub fn transaction_broadcast(transaction: SnarkosTransaction) -> Self {
        Self::Post("api/testnet3/transaction/broadcast", json!(transaction))
    }
}

impl<'a> TryFrom<mentat_types::CallRequest> for Request<'a> {
    type Error = MentatError;

    fn try_from(req: mentat_types::CallRequest) -> Result<Self, Self::Error> {
        let mut endpoint = req
            .parameters
            .get("endpoint")
            .ok_or_else(|| MentatError::from("no endpoint in `parameters` provided"))?
            .to_string();
        // TODO why is serde putting extra quotes in?
        endpoint = endpoint.replace('\"', "");
        let params = req
            .parameters
            .get("params")
            .ok_or_else(|| MentatError::from("no params in `parameters` provided"))?;
        let params_map = params
            .as_object()
            .ok_or_else(|| MentatError::from("params in `parameters` was not an object"))?;

        for (key, value) in params_map.iter() {
            endpoint = endpoint.replace(&format!("{{{key}}}"), &value.to_string());
        }
        let leaked = Box::leak(endpoint.into_boxed_str());

        Ok(match req.method.to_ascii_lowercase().as_str() {
            "post" => {
                let body = req.parameters.get("body").ok_or_else(|| {
                    MentatError::from(
                        "no body in `parameters` provided when post method was selected",
                    )
                })?;
                // TODO do we have to clone body here?
                Request::Post(leaked, body.clone())
            }
            "get" => Request::Get(leaked),
            _ => {
                return Err(MentatError::from(
                    "no endpoint params in `parameters` provided",
                ));
            }
        })
    }
}

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
    /// Makes the rest call returning the expected output given the input type.
    pub async fn rest_call<'a, O: DeserializeOwned + Debug>(&self, req: Request<'a>) -> Result<O> {
        let resp = match req {
            Request::Get(url) => {
                self.client
                    .get(&format!("{}{url}", self.node_rpc_url))
                    .send()
                    .await?
            }
            Request::Post(url, body) => {
                self.client
                    .post(&format!("{}{url}", self.node_rpc_url))
                    .json(&body)
                    .send()
                    .await?
            }
        };

        let resp_text = resp.text().await?;
        let response_type = serde_json::from_str::<O>(&resp_text)
            .merr(|e| format!("failed to serialize response: `{e}`\ntext: `{resp_text}`"))?;
        Ok(response_type)
    }
}
