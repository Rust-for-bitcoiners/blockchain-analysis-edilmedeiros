use bitcoincore_rpc::{bitcoin::BlockHash, Client, Error, RpcApi};

pub fn get_best_block_hash(rpc: &Client) -> Result<BlockHash, Error> {
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    Ok(best_block_hash)
}

pub fn get_uptime(rpc: &Client) -> Result<String, Error> {
    fn format_uptime(uptime_seconds: u64) -> String {
        let days = uptime_seconds / 86_400;
        let hours = (uptime_seconds % 86_400) / 3600;
        let minutes = (uptime_seconds % 3600) / 60;
        let seconds = uptime_seconds % 60;

        format!("{days} days, {hours} hours, {minutes} minutes, {seconds} seconds")
    }

    let uptime_seconds = rpc.uptime()?;
    let formatted_uptime = format_uptime(uptime_seconds);
    println!("uptime: {}", &formatted_uptime);
    Ok(formatted_uptime)
}

pub fn stop(rpc: &Client) -> Result<(), Error> {
    let result = rpc.stop()?;
    println!("{result}");
    Ok(())
}
