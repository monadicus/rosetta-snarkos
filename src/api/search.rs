use super::*;

#[derive(Clone, Default)]
pub struct SnarkosSearchApi;

#[async_trait]
impl SearchApiRouter for SnarkosSearchApi {}

#[async_trait]
impl SearchApi for SnarkosSearchApi {
    type NodeCaller = SnarkosCaller;
}
