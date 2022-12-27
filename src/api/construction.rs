use super::*;
use crate::construction::*;

#[derive(Clone, Default)]
pub struct SnarkosConstructionApi;

#[async_trait]
impl ConstructionApiRouter for SnarkosConstructionApi {}

#[async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    type NodeCaller = SnarkosCaller;

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<TransactionIdentifierResponse> {
        let result = node_caller
            .rpc_call::<Response<SendTransactionResult>>(Jrpc::new(
                "sendtransaction",
                vec![data.signed_transaction],
            ))
            .await?
            .unwrap_response()?;

        Ok(result.into())
    }
}
