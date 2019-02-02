extern crate reqwest;
extern crate serde_json;

use std::env;

fn main() {
    get_usernames()
        .iter()
        .for_each(|username| match retrieve_key_for_user(username) {
            Ok(keytext) => println!("{}", keytext),
            Err(error) => eprintln!("{}", error),
        })
}

fn get_usernames() -> Vec<String> {
    env::args().skip(1).collect()
}

const BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE: &str = "https://core.blockstack.org/v1/users/:username";

fn retrieve_key_for_user(username: &str) -> Result<String, String> {
    let profile_json = retrieve_user_profile(username).map_err(|err| err.to_string())?;
    let ssh_key = extract_sshkey_from_profile(username, profile_json)?;
    Ok(ssh_key)
}

fn retrieve_user_profile(username: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = build_profile_url(&username);

    reqwest::get(url.as_str())?
        .error_for_status()?
        .json::<serde_json::Value>()
}

fn build_profile_url(username: &str) -> String {
    String::from(BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE).replace(":username", username)
}

fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, String> {
    extract_user_profile(username, &profile_json)
        .and_then(|profile| extract_accounts(profile))
        .and_then(|accounts| extract_ssh_service(accounts.to_owned()))
        .and_then(|ssh_service| extract_ssh_public_key(&ssh_service))
        .ok_or(String::from("Unable to extract SSH key"))
}

fn extract_user_profile(
    username: &str,
    profile_json: &serde_json::Value,
) -> Option<serde_json::Value> {
    profile_json.get(username).map(|val| val.to_owned())
}

//TODO: model the profile object as a struct and let serde handle this
//      This would reduce these next few methods to something like
//      profile.accounts.find(|item|item.service == "ssh").map(|ssh|ssh.identifer)
//      with the cost of maintaining the struct…
fn extract_accounts<'a>(profile_json: serde_json::Value) -> Option<Vec<serde_json::Value>> {
    let accounts_value = &profile_json["profile"]["account"];
    accounts_value.as_array().map(|array| array.to_vec())
}

fn extract_ssh_service(accounts: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    accounts
        .iter()
        .find(|item| item["service"] == "ssh")
        .map(|val| val.to_owned())
}

fn extract_ssh_public_key(ssh_account: &serde_json::Value) -> Option<String> {
    let id = &ssh_account["identifier"];
    id.as_str().map(|s| String::from(s))
}
