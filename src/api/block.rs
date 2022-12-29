use super::*;
use crate::data::BlockResult;

#[derive(Clone, Default)]
pub struct SnarkosBlockApi;

#[async_trait]
impl BlockApiRouter for SnarkosBlockApi {}

#[async_trait]
impl BlockApi for SnarkosBlockApi {
    type NodeCaller = SnarkosCaller;

    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<BlockResponse> {
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
}
