use super::*;

#[derive(Clone, Default)]
pub struct SnarkosMempoolApi;

#[async_trait]
impl MempoolApiRouter for SnarkosMempoolApi {}

#[async_trait]
impl MempoolApi for SnarkosMempoolApi {
    type NodeCaller = SnarkosCaller;
}
