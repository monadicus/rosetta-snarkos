mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod node;
mod optional_api;
mod request;
mod responses;

use mentat::server::{Server, ServerType};

#[derive(Clone)]
struct MentatSnarkos;

impl ServerType for MentatSnarkos {
    type CallApi = call_api::SnarkosCallApi;
    type ConstructionApi = construction_api::SnarkosConstructionApi;
    type CustomConfig = node::NodeConfig;
    type DataApi = data_api::SnarkosDataApi;
    type IndexerApi = indexer_api::SnarkosIndexerApi;
    type OptionalApi = optional_api::SnarkosOptionalApi;
}

#[mentat::main]
async fn main() -> Server<MentatSnarkos> {
    println!("hello rosetta!");
    Server::default()
}
