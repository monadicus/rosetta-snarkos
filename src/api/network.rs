use super::*;

#[derive(Clone, Default)]
pub struct SnarkosNetworkApi;

#[async_trait]
impl NetworkApiRouter for SnarkosNetworkApi {}

#[async_trait]
impl NetworkApi for SnarkosNetworkApi {
    type NodeCaller = SnarkosCaller;

    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<NetworkListResponse> {
        MentatError::not_implemented()
        // Ok(NetworkListResponse {
        //     network_identifiers: vec![NetworkIdentifier {
        //         blockchain: "snarkos".into(),
        //         network: unimplemented!("Snarkos needs a way to to tell us
        // what networks we have or we can manually do it"),         // We have
        // no shards         sub_network_identifier: None,
        //     }],
        // })
    }

    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<NetworkOptionsResponse> {
        MentatError::not_implemented()
        // Ok(NetworkOptionsResponse {
        //     version: Version {
        //         // TODO: fetch this
        //         rosetta_version: "1.4.12".into(),
        //         node_version: unimplemented!("snarkos needs a way to tell us
        // current version."),         middleware_version:
        // Some(env!("CARGO_PKG_VERSION").into()),         metadata:
        // Default::default(),     },
        //     allow: Allow {
        //         // TODO I think these should be all program:functions?
        //         operation_statuses: Vec::new(),
        //         operation_types: Vec::new(),
        //         // TODO fill out when finished implementing.
        //         errors: Vec::new(),
        //         historical_balance_lookup: true,
        //         timestamp_start_index: Some(1),
        //         // TODO populate when call is populated.
        //         call_methods: vec!["get".into(), "post".into()],
        //         balance_exemptions: Vec::new(),
        //         // TODO need to ask Aleo about this.
        //         mempool_coins: false,
        //     },
        // })
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<NetworkStatusResponse> {
        let latest_block = node_caller
            .rest_call::<BlockResult>(Request::LATEST_BLOCK)
            .await?;
        let latest_block_timestamp = latest_block.timestamp();
        let genesis_block = node_caller
            .rest_call::<BlockResult>(Request::get_block_by_height(1))
            .await?;

        Ok(NetworkStatusResponse {
            current_block_identifier: latest_block.into(),
            current_block_timestamp: latest_block_timestamp,
            genesis_block_identifier: genesis_block.into(),
            oldest_block_identifier: None,
            sync_status: None,
            // TODO make a from<String> method for Peer in mentat.
            // TODO use ALL_PEERS_METRICS once it works
            peers: node_caller
                .rest_call::<Vec<String>>(Request::ALL_PEERS)
                .await?
                .into_iter()
                .map(|peer_id| Peer {
                    peer_id,
                    metadata: Default::default(),
                })
                .collect(),
        })
    }
}
