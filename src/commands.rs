use bitcoincore_rpc::{bitcoin::BlockHash, Client, Error, RpcApi};

pub fn get_uptime(rpc: &Client) -> Result<String, Error> {
    let uptime_seconds = rpc.uptime()?;
    let formatted_uptime = format_uptime(uptime_seconds);
    println!("uptime: {}", &formatted_uptime);
    Ok(formatted_uptime)
}

fn format_uptime(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86_400;
    let hours = (uptime_seconds % 86_400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    format!("{days} days, {hours} hours, {minutes} minutes, {seconds} seconds")
}
