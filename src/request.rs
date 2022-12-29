use std::fmt::Debug;

use mentat_server::{
    conf::Configuration,
    reqwest,
    serde::{de::DeserializeOwned, Serialize},
    serde_json::{self, json, Value},
};
use mentat_types::{MapErrMentat, Result};

use crate::node::Config;

pub enum Request<'a> {
    Get(&'a str),
    Post(&'a str),
}

impl<'a> Request<'a> {
    pub const ALL_PEERS: Self = Self::Get("api/testnet3/peers/all");
    pub const ALL_RECORDS: Self = Self::Get("api/testnet3/records/all");
    pub const COUNT_PEERS: Self = Self::Get("api/testnet3/peers/count");
    pub const LATEST_BLOCK: Self = Self::Get("api/testnet3/latest/block");
    pub const LATEST_HASH: Self = Self::Get("api/testnet3/latest/hash");
    pub const LATEST_HEIGHT: Self = Self::Get("api/testnet3/latest/height");
    pub const NODE_ADDRESS: Self = Self::Get("api/testnet3/node/address");
    pub const SPENT_RECORDS: Self = Self::Get("api/testnet3/records/spent");
    pub const TRANSACTIONS_BROADCAST: Self = Self::Get("api/testnet3/transactions/broadcast");
    pub const TRANSACTIONS_MEMPOOL: Self = Self::Get("api/testnet3/transactions/mempool");
    pub const UNSPENT_RECORDS: Self = Self::Get("api/testnet3/records/unspent");
    pub const VALIDATORS: Self = Self::Get("api/testnet3/validators");

    pub fn get_block(height: usize) -> Self {
        let ep = format!("api/testnet3/block/{height}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    pub fn get_blocks(start: usize, end: usize) -> Self {
        let ep = format!("api/testnet3/block?start={start}&end={end}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    pub fn get_transactions(height: usize) -> Self {
        let ep = format!("api/testnet3/transactions/{height}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    pub fn get_transaction(id: String) -> Self {
        let ep = format!("api/testnet3/transaction/{id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    pub fn get_program(id: String) -> Self {
        let ep = format!("api/testnet3/program/{id}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
    }

    pub fn get_state_path(commitment: String) -> Self {
        let ep = format!("api/testnet3/statePath/{commitment}");
        let leaked = Box::leak(ep.into_boxed_str());
        Self::Get(leaked)
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
            Request::Post(url) => {
                self.client
                    .post(&format!("{}{url}", self.node_rpc_url))
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
