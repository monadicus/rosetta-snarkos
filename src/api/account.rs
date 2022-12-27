use super::*;

#[derive(Clone, Default)]
pub struct SnarkosAccountApi;

#[async_trait]
impl AccountApiRouter for SnarkosAccountApi {}

#[async_trait]
impl AccountApi for SnarkosAccountApi {
    type NodeCaller = SnarkosCaller;
}
