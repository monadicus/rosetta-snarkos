mod api;
use api::*;
mod node;
mod request;
mod responses;

use mentat_asserter::Asserter;
use mentat_server::{
    conf::{AsserterTable, Configuration, NodeConf},
    server::{Server, ServerBuilder, ServerType},
};
pub use responses::*;

// This way also works
// TODO but need a way to enable optional api from macro
// #[mentat_server::mentat]
struct MentatSnarkos;

impl ServerType for MentatSnarkos {
    type AccountApi = SnarkosAccountApi;
    type BlockApi = SnarkosBlockApi;
    type CallApi = SnarkosCallApi;
    type ConstructionApi = SnarkosConstructionApi;
    type CustomConfig = node::Config;
    type EventsApi = SnarkosEventsApi;
    type MempoolsApi = SnarkosMempoolApi;
    type NetworkApi = SnarkosNetworkApi;
    type NodeCaller = request::SnarkosCaller;
    type OptionalApi = SnarkosOptionalApi;
    type SearchApi = SnarkosSearchApi;

    fn init_asserters(config: &Configuration<Self::CustomConfig>) -> AsserterTable {
        Asserter::new_server(
            // TODO required to have values, but need to figure out.
            vec!["INPUT".into(), "OUTPUT".into(), "COINBASE".into()],
            true,
            vec![
                (
                    Self::CustomConfig::BLOCKCHAIN,
                    config.network.to_string().as_str(),
                )
                    .into(),
            ],
            vec!["get".into(), "post".into()],
            false,
            None,
        )
        .unwrap()
        .into()
    }
}

#[mentat_server::main]
async fn main() -> Server<MentatSnarkos> {
    println!("hello rosetta!");
    ServerBuilder::default()
        .account_api(SnarkosAccountApi)
        .block_api(SnarkosBlockApi)
        .call_api(SnarkosCallApi)
        .construction_api(SnarkosConstructionApi)
        .custom_configuration_from_arg()
        .events_api(SnarkosEventsApi)
        .mempool_api(SnarkosMempoolApi)
        .network_api(SnarkosNetworkApi)
        .optional_api(SnarkosOptionalApi, true)
        .search_api(SnarkosSearchApi)
        .build()
}
