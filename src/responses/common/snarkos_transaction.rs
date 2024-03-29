use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
#[serde(tag = "type")]
pub enum SnarkosTransaction {
    #[serde(rename = "deploy")]
    Deploy(Box<DeployTx>),
    #[serde(rename = "execute")]
    Execute(Box<ExecTx>),
}

impl SnarkosTransaction {
    pub fn id(&self) -> String {
        match self {
            Self::Deploy(tx) => tx.id.clone(),
            Self::Execute(tx) => tx.id.clone(),
        }
    }

    fn operations(self) -> Vec<Operation> {
        match self {
            Self::Deploy(_) => todo!(),
            Self::Execute(tx) => tx
                .execution
                .transitions
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct DeployTx {
    id: String,
    deployment: Deployment,
    // additional_fee: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct Deployment {
    // edition: u16,
    // pub program: (),
    // pub verifying_keys: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct ExecTx {
    id: String,
    execution: Execution,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct Execution {
    transitions: Vec<SnarkosTransition>,
    global_state_root: String,
    inclusion: String,
}

impl From<SnarkosTransaction> for Transaction {
    fn from(transaction: SnarkosTransaction) -> Self {
        Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: transaction.id(),
            },
            operations: transaction.operations(),
            related_transactions: Vec::new(),
            metadata: IndexMap::new(),
        }
    }
}

impl From<SnarkosTransaction> for BlockTransactionResponse {
    fn from(transaction: SnarkosTransaction) -> Self {
        BlockTransactionResponse {
            transaction: transaction.into(),
        }
    }
}

impl From<SnarkosTransaction> for TransactionIdentifier {
    fn from(transaction: SnarkosTransaction) -> Self {
        TransactionIdentifier {
            hash: transaction.id(),
        }
    }
}
