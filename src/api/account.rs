use super::*;

#[derive(Clone, Default)]
pub struct SnarkosAccountApi;

#[async_trait]
impl AccountApiRouter for SnarkosAccountApi {}

#[async_trait]
impl AccountApi for SnarkosAccountApi {
    type NodeCaller = SnarkosCaller;

    // TODO
    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountBalanceResponse> {
        MentatError::not_implemented()
    }

    // TODO
    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountCoinsResponse> {
        MentatError::not_implemented()
    }
}
