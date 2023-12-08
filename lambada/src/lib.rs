pub mod executor;

use surf_disco::Url;
use clap::Parser;
use ethers::types::Address;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_DEMO_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Address of HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_HOTSHOT_ADDRESS")]
    pub hotshot_address: Address,

    #[clap(long, env = "MACHINE_DIR")]
    pub machine_dir: String,

    #[clap(long, env = "DB_DIR", default_value = "sequencer_db")]
    pub db_dir: String,

    #[clap(long, env = "APPCHAIN")]
    pub appchain: String,

    #[clap(long, env = "COMPUTE_ONLY")]
    pub compute_only: bool,
}