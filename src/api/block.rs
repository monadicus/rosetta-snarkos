use super::*;

#[derive(Clone, Default)]
pub struct SnarkosBlockApi;

#[async_trait]
impl BlockApiRouter for SnarkosBlockApi {}

#[async_trait]
impl BlockApi for SnarkosBlockApi {
    type NodeCaller = SnarkosCaller;
}
