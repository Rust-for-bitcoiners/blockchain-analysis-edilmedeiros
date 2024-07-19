mod parse;
mod commands;
mod rpc;

// Project shortcuts
use parse::{Command, get_args};
use commands::*;

// TODO: build utxo set
// TODO: compute transaction fee

// TODO: implement proper error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get the arguments from our wrapper parser
    let opts = get_args();

    // Get the rpc client from our rpc wrapper module
    let rpc = rpc::rpc(
        &opts.rpc_url.unwrap(),
        &opts.rpc_port.unwrap(),
        &opts.rpc_user.unwrap(),
        &opts.rpc_password.unwrap())?;

    // Match command args and call the right function!
    match opts.command {
        Command::Uptime => {
            get_uptime(&rpc)?;
        },
        Command::GetBestBlockHash => {
            get_best_block_hash(&rpc)?;
        },
        Command::Stop => {
            stop(&rpc)?;
        },
        Command::TimeToMine { height } => {
            let result = time_to_mine(&rpc, height)?;
            println!("{result}");
        },
        Command::NumberOfTransactions { height } => {
            let result = number_of_transactions(&rpc, height)?;
            println!("{result}");
        },

    }

    Ok(())
}
