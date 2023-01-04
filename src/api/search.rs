use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosSearchApi;

#[async_trait]
impl SearchApi for SnarkosSearchApi {
    type NodeCaller = SnarkosCaller;
}
