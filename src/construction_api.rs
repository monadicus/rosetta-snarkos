use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
    axum::{async_trait, Json},
    requests::ConstructionSubmitRequest,
    responses::TransactionIdentifierResponse,
    server::RpcCaller,
};

use crate::{
    request::SnarkosJrpc,
    responses::{construction::*, Response},
};

#[derive(Clone, Default)]
pub struct SnarkosConstructionApi;

#[async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

#[async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        let result = rpc_caller
            .rpc_call::<Response<SendTransactionResult>>(SnarkosJrpc::new(
                "sendtransaction",
                vec![data.signed_transaction],
            ))
            .await?;

        Ok(Json(result.into()))
    }
}
