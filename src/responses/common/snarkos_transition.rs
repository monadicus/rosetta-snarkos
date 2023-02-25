use mentat_server::serde_json::json;

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct SnarkosTransition {
    id: String,
    program: String,
    function: String,
    // inputs: Vec<type, id, tag>,
    // outputs: Vec<type, id, checksum, value>,
    proof: String,
    tpk: String,
    tcm: String,
    fee: i64,
}

impl From<SnarkosTransition> for Operation {
    fn from(transition: SnarkosTransition) -> Self {
        Self {
            operation_identifier: OperationIdentifier {
                // IDK
                index: 0,
                network_index: None,
            },
            related_operations: Vec::new(),
            type_: format!("{}:{}", transition.program, transition.function),
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
            metadata: indexmap! {
                "proof".into() => json!(transition.proof),
                "tpk".into() => json!(transition.tpk),
                "tcm".into() => json!(transition.tcm),
            },
        }
    }
}
