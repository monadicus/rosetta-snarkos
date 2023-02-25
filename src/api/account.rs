use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosAccountApi;

#[async_trait]
impl AccountApi for SnarkosAccountApi {
    type NodeCaller = SnarkosCaller;

    // TODO not possible no way to get addresses of other nodes.
    // Therefore no way to get balance of an address.
    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountBalanceResponse> {
        MentatError::not_implemented()
        // data.account_identifier
        // Ok(AccountBalanceResponse {
        // block_identifier: todo!(),
        // balances: Vec::new(),
        // metadata: Default::default(),
        // })
    }

    // TODO not possible no way to get addresses of other nodes.
    // Therefore no way to get unspent coins of an address.
    // Also not sure if we need at all.
    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountCoinsResponse> {
        MentatError::not_implemented()
    }
}
