use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct SnarkosTransition {
    id: String,
    // program: String,
    // function: String,
    // inputs: Vec<type, id, tag>,
    // outputs: Vec<type, id, checksum, value>,
    // proof: String,
    // tpk: String,
    // tcm: String,
    fee: i64,
}

impl From<SnarkosTransition> for Operation {
    fn from(transition: SnarkosTransition) -> Self {
        Self {
            operation_identifier: OperationIdentifier {
                index: 0,
                network_index: None,
            },
            // related_operations: Some(transition.events.into_iter().map(|e| e.into()).collect()),
            related_operations: Vec::new(),
            // TODO: I see no information on this.
            type_: "N/A".to_string(),
            status: None,
            account: None,
            amount: Some(Amount {
                value: transition.fee.to_string(),
                currency: Currency {
                    symbol: "ALEO".to_string(),
                    decimals: 18,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }),
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: transition.id,
                },
                // TODO: I see no information on this.
                coin_action: CoinAction::CoinSpent,
            }),
            metadata: IndexMap::new(),
        }
    }
}
