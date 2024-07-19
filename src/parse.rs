use clap::{Parser, Subcommand};
use std::env;

/// Specify the command line interface options
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    // Bitcoin core node URL to JSON-RPC
    #[arg(long, value_name = "url")]//, default_value = "127.0.0.1")]
    pub rpc_url: Option<String>,

    /// Connect to JSON-RPC on <port>
    #[arg(long, value_name = "port")]//, default_value = "8332")]
    pub rpc_port: Option<String>,

    /// Username for JSON_RPC connections
    #[arg(long, value_name = "username")]
    pub rpc_user: Option<String>,

    /// Password for JSON-RPC connections
    #[arg(long, value_name = "password")]
    pub rpc_password: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

/// Specify the methods this CLI tool will understand
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Returns the hash of the best (tip) in the most-work fully-validated
    /// chain.
    #[command(name = "getbestblockhash")]
    GetBestBlockHash,

    /// Returns the total uptime of the server.
    #[command(name = "uptime")]
    Uptime,

    /// Stops the node
    #[command(name = "stop")]
    Stop,

    /// Estimate the time it took to mine block at height n
    #[command(name = "timetomine")]
    TimeToMine {
        height: u64,
    },

    /// Get the number of transactions in block
    #[command(name = "numberoftransactions")]
    NumberOfTransactions {
        height: u64,
    }

}

/// Returns the arguments from the clap parser as an Args struct defined above
pub fn get_args() -> Opts {

    // If any of url, port, user and password are not set from the command line,
    // try getting from the .env file
    dotenv::dotenv().ok();

    // Parse the command line
    let mut opts = Opts::parse();

    // Process the node URL: if data doesn't come from the command line try to
    // get from the environment. If none is provided by the user, use default
    // localhost.
    // TODO: improve error handling of var().
    opts.rpc_url = opts.rpc_url
        .or_else(|| env::var("BITCOIN_RPC_URL").ok())
        .or(Some("127.0.0.1".to_owned()));

    // Process the node port: if data doesn't come from the command line try to
    // get from the environment. If none is provided by the user, use default
    // 8332 (mainnet).
    // TODO: improve error handling of var().
    opts.rpc_port = opts.rpc_port
        .or_else(|| env::var("BITCOIN_RPC_PORT").ok())
        .or(Some("8332".to_owned()));

    // Process the node user name: if data doesn't come from the command line try to
    // get from the environment. If none is provided by the user, use default "".
    // TODO: improve error handling of var().
    opts.rpc_user = opts.rpc_user
        .or_else(|| env::var("BITCOIN_RPC_USER").ok())
        .or(Some("".to_owned()));

    // Process the node password: if data doesn't come from the command line try to
    // get from the environment. If none is provided by the user, use default "".
    // TODO: improve error handling of var().
    opts.rpc_password= opts.rpc_password
        .or_else(|| env::var("BITCOIN_RPC_PASSWORD").ok())
        .or(Some("".to_owned()));

    opts
}
