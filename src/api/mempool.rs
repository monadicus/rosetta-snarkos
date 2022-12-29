use super::*;

#[derive(Clone, Default)]
pub struct SnarkosMempoolApi;

#[async_trait]
impl MempoolApiRouter for SnarkosMempoolApi {}

#[async_trait]
impl MempoolApi for SnarkosMempoolApi {
    type NodeCaller = SnarkosCaller;

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<MempoolResponse> {
        let mempool = node_caller
            .rest_call::<Vec<SnarkosTransaction>>(Request::MEM_POOL_TRANSACTIONS)
            .await?;

        Ok(MempoolResponse {
            transaction_identifiers: mempool.into_iter().map(|t| t.into()).collect(),
        })
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<MempoolTransactionResponse> {
        let hash = data.transaction_identifier.hash;

        let mempool = node_caller
            .rest_call::<Vec<SnarkosTransaction>>(Request::MEM_POOL_TRANSACTIONS)
            .await?;

        if let Some(tx) = mempool.into_iter().find(|t| t.id() == hash) {
            // TODO should implement from so we can get metadata.
            Ok(MempoolTransactionResponse {
                transaction: tx.into(),
                metadata: Default::default(),
            })
        } else {
            MentatError::transaction_not_found(Some(&hash))
        }
    }
}
