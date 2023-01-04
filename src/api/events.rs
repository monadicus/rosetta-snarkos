use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosEventsApi;

#[async_trait]
impl EventsApi for SnarkosEventsApi {
    type NodeCaller = SnarkosCaller;
}
