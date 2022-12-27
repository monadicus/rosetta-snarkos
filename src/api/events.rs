use super::*;

#[derive(Clone, Default)]
pub struct SnarkosEventsApi;

#[async_trait]
impl EventsApiRouter for SnarkosEventsApi {}

#[async_trait]
impl EventsApi for SnarkosEventsApi {
    type NodeCaller = SnarkosCaller;
}
