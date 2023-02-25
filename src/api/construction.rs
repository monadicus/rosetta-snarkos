use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosConstructionApi;

// TODO
// Not possible till mentat is finished &
// Snarkos has a way for users to get addresses
#[async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    type NodeCaller = SnarkosCaller;

    // async fn submit(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionSubmitRequest,
    //     node_caller: &Self::NodeCaller,
    // ) -> Result<TransactionIdentifierResponse> {
    //     // let result = node_caller.rest_call(Request::Get())
    //     // let result = node_caller
    //     //     .rpc_call::<Response<SendTransactionResult>>(Jrpc::new(
    //     //         "sendtransaction",
    //     //         vec![data.signed_transaction],
    //     //     ))
    //     //     .await?
    //     //     .unwrap_response()?;

    //     // Ok(result.into())
    //     todo!()
    // }
}
