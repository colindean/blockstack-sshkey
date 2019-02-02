pub fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, String> {
    extract_user_profile(username, &profile_json)
        .and_then(extract_accounts)
        .and_then(extract_ssh_service)
        .and_then(extract_ssh_public_key)
        //.ok_or_else(|| String::from("Unable to extract SSH key"))
}

fn extract_user_profile(
    username: &str,
    profile_json: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    profile_json.get(username).map(|val| val.to_owned()).ok_or_else(||String::from(format!("The user property '{}' is missing from the JSON object.", username)))
}

//TODO: model the profile object as a struct and let serde handle this
//      This would reduce these next few methods to something like
//      profile.accounts.find(|item|item.service == "ssh").map(|ssh|ssh.identifer)
//      with the cost of maintaining the struct…
fn extract_accounts(profile_json: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    let accounts_value = &profile_json["profile"]["account"];
    accounts_value.as_array().map(|array| array.to_vec()).ok_or_else(|| String::from("The profile or its account object are missing."))
}

fn extract_ssh_service(accounts: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    accounts
        .iter()
        .find(|item| item["service"] == "ssh")
        .map(|val| val.to_owned())
        .ok_or_else(|| String::from("No service of type ssh was found in profile account list."))
}

fn extract_ssh_public_key(ssh_account: serde_json::Value) -> Result<String, String> {
    let id = &ssh_account["identifier"];
    id.as_str().map(String::from).ok_or_else(||String::from("The SSH service is present but missing its identifier, which is the key text."))
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

#[test]
fn test_no_sshkey_identifier() {
    let json_text = r#"
      {"test": {
        "profile": {
          "account":
            [
              {
                "service":"ssh"
              }
            ]
          }
        }
      }
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(extract_sshkey_from_profile("test", json), Err(String::from("The SSH service is present but missing its identifier, which is the key text.")))
}

#[test]
fn test_no_ssh_service() {
    let json_text = r#"
      {"test": {
        "profile": {
          "account": []
          }
        }
      }
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(extract_sshkey_from_profile("test", json), Err(String::from("No service of type ssh was found in profile account list.")))
}

#[test]
fn test_no_user_property() {
    let json_text = r#"
      {}
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(extract_sshkey_from_profile("test", json), Err(String::from("The user property 'test' is missing from the JSON object.")))
}