use super::*;

#[derive(Clone, Default)]
pub struct SnarkosOptionalApi;

#[async_trait]
impl OptionalApiRouter for SnarkosOptionalApi {
    async fn call_health(
        &self,
        caller: Caller,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
        server_pid: Pid,
        node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        Ok(Json(
            self.health(caller, mode, node_caller, server_pid, node_pid)
                .await?,
        ))
    }
}

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {
    type NodeCaller = SnarkosCaller;
}
