use bitcoincore_rpc::{bitcoin::BlockHash, Client, Error, RpcApi};

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
