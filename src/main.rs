mod api;
use api::*;
mod node;
mod request;
mod responses;

use mentat_asserter::Asserter;
use mentat_server::{
    conf::{AsserterTable, Configuration, NodeConf},
    mentat,
    server::ServerType,
};
pub use responses::*;

#[mentat]
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
            vec!["INPUT".into(), "OUTPUT".into(), "COINBASE".into()],
            true,
            vec![
                (
                    Self::CustomConfig::BLOCKCHAIN,
                    config.network.to_string().as_str(),
                )
                    .into(),
            ],
            Vec::new(),
            false,
            None,
        )
        .unwrap()
        .into()
    }
}
