use super::*;

#[derive(Clone, Default)]
pub struct SnarkosNetworkApi;

#[async_trait]
impl NetworkApiRouter for SnarkosNetworkApi {}

#[async_trait]
impl NetworkApi for SnarkosNetworkApi {
    type NodeCaller = SnarkosCaller;
}
