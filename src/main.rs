extern crate reqwest;
extern crate serde_json;

use std::env;

fn main() {
    get_usernames().iter().for_each(|username| {
        let key = retrieve_key_for_user(username);
        match key {
            Ok(keytext) => println!("{}", keytext),
            Err(error) => eprintln!("{}", error),
        }
    })
}

fn get_usernames() -> Vec<String> {
    env::args().skip(1).collect()
}

const BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE: &str = "https://core.blockstack.org/v1/users/:username";

fn retrieve_key_for_user(username: &str) -> Result<String, &'static str> {
    let profile_json = retrieve_user_profile(username);
    let ssh_key = extract_sshkey_from_profile(username, profile_json);
    ssh_key
}

fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, &'static str> {
    extract_user_profile(username, &profile_json)
        .and_then(|profile| extract_accounts(profile))
        .and_then(|accounts| extract_ssh_service(accounts.to_owned()))
        .and_then(|ssh_service| extract_ssh_public_key(&ssh_service))
        .ok_or("Unable to extract SSH key")
}

fn retrieve_user_profile(username: &str) -> serde_json::Value {
    let url = build_profile_url(&username);
    reqwest::get(url.as_str())
        .expect("Retrieval failure")
        .json()
        .expect("Oh noes")
}

fn build_profile_url(username: &str) -> String {
    String::from(BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE).replace(":username", username)
}
fn extract_user_profile(
    username: &str,
    profile_json: &serde_json::Value,
) -> Option<serde_json::Value> {
    profile_json.get(username).map(|val| val.to_owned())
}
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
