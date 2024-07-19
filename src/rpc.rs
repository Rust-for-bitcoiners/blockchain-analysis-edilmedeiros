use bitcoincore_rpc::{Auth, Client, Error};

// Creates an rpc client used as a base to communicate with the API methods
pub fn rpc(url: &str, port: &str, user: &str, passwd: &str) -> Result<Client, Error> {
    let auth = Auth::UserPass(user.to_string(), passwd.to_string());
    let full_url = format!("{url}:{port}");
    Client::new(&full_url, auth)
}
