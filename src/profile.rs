pub fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, String> {
    extract_user_profile(username, &profile_json)
        .and_then(extract_accounts)
        .and_then(|accounts| extract_ssh_service(accounts.to_owned()))
        .and_then(extract_ssh_public_key)
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

fn extract_ssh_public_key(ssh_account: serde_json::Value) -> Option<String> {
    let id = &ssh_account["identifier"];
    id.as_str().map(|s| String::from(s))
}
