pub fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, String> {
    extract_user_profile(username, &profile_json)
        .and_then(extract_accounts)
        .and_then(extract_ssh_service)
        .and_then(extract_ssh_public_key)
        .ok_or_else(|| String::from("Unable to extract SSH key"))
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
fn extract_accounts(profile_json: serde_json::Value) -> Option<Vec<serde_json::Value>> {
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
    id.as_str().map(String::from)
}

#[test]
fn test_valid_json() {
    let json_text = r#"
      {"test": {
        "profile": {
          "account":
            [
              {
                "service":"ssh",
                "identifier":"yep"
              }
            ]
          }
        }
      }
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(extract_sshkey_from_profile("test", json), Ok(String::from("yep")))
}