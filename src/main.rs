extern crate reqwest;
extern crate serde_json;

use std::env;

mod profile;
mod retrieve;

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

fn retrieve_key_for_user(username: &str) -> Result<String, String> {
    let profile_json = retrieve::retrieve_user_profile(username).map_err(|err| err.to_string())?;
    let ssh_key = profile::extract_sshkey_from_profile(username, profile_json)?;
    Ok(ssh_key)
}
