use mentat::models::Transaction;

use super::*;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosTransactions {
    pub transactions: Vec<SnarkosTransaction>,
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Transaction>> for SnarkosTransactions {
    fn into(self) -> Vec<Transaction> {
        self.transactions.into_iter().map(|t| t.into()).collect()
    }
}
