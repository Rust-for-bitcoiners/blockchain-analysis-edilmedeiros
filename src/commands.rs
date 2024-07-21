use std::str::FromStr;

use bitcoincore_rpc::{Client, Error, RpcApi};
use bitcoincore_rpc::bitcoin::{BlockHash, blockdata::transaction::Txid};
use bitcoincore_rpc::bitcoincore_rpc_json;

pub fn get_best_block_hash(rpc: &Client) -> Result<BlockHash, Error> {
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    Ok(best_block_hash)
}

pub fn get_uptime(rpc: &Client) -> Result<String, Error> {
    let uptime_seconds = rpc.uptime()?;
    let formatted_uptime = format_time(uptime_seconds);
    println!("uptime: {}", &formatted_uptime);
    Ok(formatted_uptime)
}

fn format_time(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86_400;
    let hours = (uptime_seconds % 86_400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    let mut result = String::new();

    match (days, hours, minutes, seconds) {
        (0, 0, 0, _) => {
            result.push_str(&format!("{seconds} seconds"));
        }
        (0, 0, _, _) => {
            result.push_str(&format!("{minutes} minutes, {seconds} seconds"));
        }
        (0, _, _, _) => {
            result.push_str(&format!("{hours} hours, {minutes} minutes, {seconds} seconds"));
        }
        _            => {
            result.push_str(&format!("{days} days, {hours} hours, {minutes} minutes, {seconds} seconds"));
        }
    }
    result
}

pub fn stop(rpc: &Client) -> Result<(), Error> {
    let result = rpc.stop()?;
    println!("{result}");
    Ok(())
}

pub fn time_to_mine(rpc: &Client, height: u64) -> Result<String, Error> {
    // We don't know how long it toook to mine the genesis block
    if height == 0 {
        return Ok("Better ask Satoshi 😉".to_owned());
    }

    let block_hash = rpc.get_block_hash(height)?;
    let block_data = rpc.get_block(&block_hash)?;
    let block_time = block_data.header.time;

    let prev_block_hash = rpc.get_block_hash(height - 1)?;
    let prev_block_data = rpc.get_block(&prev_block_hash)?;
    let prev_block_time = prev_block_data.header.time;

    let time_to_mine = block_time - prev_block_time;

    Ok(format_time(time_to_mine.into()))
}

pub fn number_of_transactions(rpc: &Client, height: u64) -> Result<String, Error> {
    let block_data = rpc.get_block_stats_fields(height, &[bitcoincore_rpc_json::BlockStatsFields::Txs])?;
    let number_of_transactions = block_data.txs.unwrap();
    Ok(number_of_transactions.to_string())
}

pub fn get_transaction_fee(rpc: &Client, txid: String) -> Result<String, Error> {
    let txid = Txid::from_str(&txid).unwrap(); // TODO: error handling
    let transaction_data = rpc.get_raw_transaction(&txid, None)?;

    // Get output total
    let mut output_amount: u64 = 0;
    for txout in &transaction_data.output {
        output_amount += txout.value.to_sat();
    }

    // Get input total
    let mut input_amount: u64 = 0;
    for txin in &transaction_data.input {
        let previous_transaction = rpc.get_raw_transaction(&txin.previous_output.txid, None)?;
        input_amount += previous_transaction.output[txin.previous_output.vout as usize].value.to_sat();
    }

    let fee = input_amount - output_amount;

    Ok(fee.to_string())
}
