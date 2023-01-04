use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosCallApi;

#[async_trait]
impl CallApi for SnarkosCallApi {
    type NodeCaller = SnarkosCaller;

    async fn call(
        &self,
        _caller: Caller,
        data: CallRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<CallResponse> {
        let result = node_caller.rest_call(data.try_into()?).await?;
        Ok(CallResponse {
            result,
            // TODO test and figure out
            idempotent: false,
        })
    }
}
