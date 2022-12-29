use std::{process::Command, str::FromStr};

use mentat_server::{
    axum::async_trait,
    conf::{Configuration, NodeConf},
    reqwest::Url,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct Config {
    pub private_key: String,
}

#[async_trait]
impl NodeConf for Config {
    const BLOCKCHAIN: &'static str = "Snarkos";

    fn node_command(config: &Configuration<Self>) -> Command {
        // TODO: make it so snarkos checks for updates and rebuilds automatically.
        let mut command = Command::new(&config.node_path);
        command.args([
            "start",
            "--nodisplay",
            "--node",
            &format!("{}:4133", config.address),
            "--rest",
            &format!("{}:{}", config.address, config.node_rpc_port),
            "--beacon",
            &config.custom.private_key,
        ]);
        command
    }
}

impl Config {
    pub fn build_url(conf: &Configuration<Self>) -> Url {
        let url = format!(
            "{}://{}:{}",
            if conf.secure_http { "https" } else { "http" },
            "vm.aleo.org",
            "80"
        );

        Url::from_str(&url).expect("Invalid node url: {url}")
    }
}
