mod parse;
mod commands;
mod rpc;

// Project shortcuts
use parse::{Command, get_args};
use commands::*;

// TODO: Task 1
// fn time_to_mine(block_height: u64) -> Duration {
//     // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
//     // when the lazy macro is expanded
//     // if a value has a static lifetime then it means that value lives as long as the program lives
//     let rpc_client: &Client = &*RPC_CLIENT;
//     rpc_client.get_block_hash(234);
//     todo!()
// }

// // TODO: Task 2
// fn number_of_transactions(block_height: u64) -> u16 {
//     let some_value = Box::new(4 as u32);
//     todo!()
// }

// TODO: build utxo set
// TODO: compute transaction fee

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
    }

    Ok(())
}
