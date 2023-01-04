use super::*;

#[derive(Clone, Debug, Default)]
pub struct SnarkosOptionalApi;

// #[async_trait]
// impl OptionalApiRouter for SnarkosOptionalApi {
//     async fn call_health(
//         &self,
//         caller: Caller,
//         mode: &Mode,
//         node_caller: &Self::NodeCaller,
//         server_pid: ServerPid,
//         node_pid: NodePid,
//     ) -> MentatResponse<HealthCheckResponse> {
//         Ok(Json(
//             self.health(caller, mode, node_caller, server_pid, node_pid)
//                 .await?,
//         ))
//     }
// }

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {
    type NodeCaller = SnarkosCaller;

    async fn synced(&self, _node_caller: &Self::NodeCaller) -> Result<Synced> {
        // No way to do this, aleo doesn't run the rest server unless the node
        // is finished syncing.
        // Ok(Synced {
        //     local_tip: 0,
        //     global_tip: node_caller.rest_call(Request::LATEST_HEIGHT).await?,
        // })
        MentatError::not_implemented()
    }

    async fn node_address(&self, node_caller: &Self::NodeCaller) -> Result<String> {
        Ok(node_caller.rest_call(Request::NODE_ADDRESS).await?)
    }

    async fn node_connections(
        &self,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeConnections>> {
        Ok(if let Mode::Offline = mode {
            Some(NodeConnections::Offline)
        } else {
            let total = node_caller.rest_call(Request::COUNT_PEERS).await?;

            // TODO I'm assuming all connected peers are outbound and inbound ones.
            Some(NodeConnections::Online {
                total,
                outbound: total,
                inbound: total,
            })
        })
    }

    async fn node_net_usage(
        &self,
        mode: &Mode,
        _node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeNetwork>> {
        Ok(if let Mode::Offline = mode {
            Some(NodeNetwork::Offline)
        } else {
            // TODO ask about ways to get bytes received and sent
            return Ok(None);
        })
    }
}
