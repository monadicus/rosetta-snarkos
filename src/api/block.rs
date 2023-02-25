use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosBlockApi;

#[async_trait]
impl BlockApi for SnarkosBlockApi {
    type NodeCaller = SnarkosCaller;

    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<BlockResponse> {
        if let Some(hash) = data.block_identifier.hash.as_ref() {
            tracing::debug!(hash);
            let block = node_caller
                .rest_call::<BlockResult>(Request::get_block_by_hash(hash))
                .await?;
            return Ok(block.into());
        }

        let id = if let Some(block_id) = data.block_identifier.index {
            block_id
        } else {
            node_caller
                .rest_call::<usize>(Request::LATEST_HEIGHT)
                .await?
        };
        tracing::debug!(id);

        let block = node_caller
            .rest_call::<BlockResult>(Request::get_block_by_height(id))
            .await?;
        Ok(block.into())
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<BlockTransactionResponse> {
        let transaction = node_caller
            .rest_call::<SnarkosTransaction>(Request::get_transaction(
                &data.transaction_identifier.hash,
            ))
            .await?;
        Ok(transaction.into())
    }
}
