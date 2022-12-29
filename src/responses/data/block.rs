use super::*;
use crate::common::SnarkosTransaction;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
struct Metadata {
    network: usize,
    round: usize,
    height: usize,
    coinbase_target: usize,
    proof_target: usize,
    last_coinbase_target: usize,
    last_coinbase_timestamp: usize,
    timestamp: usize,
}

// TODO make this a proc macro
impl From<Metadata> for IndexMap<String, Value> {
    fn from(metadata: Metadata) -> Self {
        let mut map = IndexMap::new();
        map.insert("network".to_string(), metadata.network.into());
        map.insert("round".to_string(), metadata.round.into());
        map.insert("height".to_string(), metadata.height.into());
        map.insert(
            "coinbase_target".to_string(),
            metadata.coinbase_target.into(),
        );
        map.insert("proof_target".to_string(), metadata.proof_target.into());
        map.insert(
            "last_coinbase_target".to_string(),
            metadata.last_coinbase_target.into(),
        );
        map.insert(
            "last_coinbase_timestamp".to_string(),
            metadata.last_coinbase_timestamp.into(),
        );
        map.insert("timestamp".to_string(), metadata.timestamp.into());
        map
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
struct Header {
    // previous_state_root: String,
    // transactions_root: String,
    // coinbase_accumulator_point: String,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct BlockResult {
    block_hash: String,
    previous_hash: String,
    header: Header,
    transactions: Vec<SnarkosTransaction>,
    // signature: String,
}

impl From<BlockResult> for BlockResponse {
    fn from(result: BlockResult) -> Self {
        BlockResponse {
            block: Some(Block {
                block_identifier: BlockIdentifier {
                    index: result.header.metadata.height,
                    hash: result.block_hash,
                },
                parent_block_identifier: BlockIdentifier {
                    index: result.header.metadata.height.saturating_sub(1),
                    hash: result.previous_hash,
                },
                timestamp: result.header.metadata.timestamp,
                transactions: result.transactions.into_iter().map(|t| t.into()).collect(),
                metadata: result.header.metadata.into(),
            }),
            other_transactions: Vec::new(),
        }
    }
}
