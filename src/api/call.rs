use super::*;

#[derive(Clone, Default)]
pub struct SnarkosCallApi;

#[async_trait]
impl CallApiRouter for SnarkosCallApi {}

#[async_trait]
impl CallApi for SnarkosCallApi {
    type NodeCaller = SnarkosCaller;
}
