use std::error::Error as StdError;
use std::fmt;

pub fn extract_sshkey_from_profile(
    username: &str,
    profile_json: serde_json::Value,
) -> Result<String, ExtractionError> {
    extract_user_profile(username, &profile_json)
        .and_then(extract_accounts)
        .and_then(extract_ssh_service)
        .and_then(extract_ssh_public_key)
}

fn extract_user_profile(
    username: &str,
    profile_json: &serde_json::Value,
) -> Result<serde_json::Value, ExtractionError> {
    profile_json
        .get(username)
        .map(|val| val.to_owned())
        .ok_or(ExtractionError::new(Kind::MissingUserProperty(String::from(username))))

}

//TODO: model the profile object as a struct and let serde handle this
//      This would reduce these next few methods to something like
//      profile.accounts.find(|item|item.service == "ssh").map(|ssh|ssh.identifer)
//      with the cost of maintaining the struct…
fn extract_accounts(profile_json: serde_json::Value) -> Result<Vec<serde_json::Value>, ExtractionError> {
    let accounts_value = &profile_json["profile"]["account"];
    accounts_value
        .as_array()
        .map(|array| array.to_vec())
        .ok_or(ExtractionError::new(Kind::MissingProfileOrAccount))
}

fn extract_ssh_service(accounts: Vec<serde_json::Value>) -> Result<serde_json::Value, ExtractionError> {
    accounts
        .iter()
        .find(|item| item["service"] == "ssh")
        .map(|val| val.to_owned())
        .ok_or(ExtractionError::new(Kind::MissingSshService))
}

fn extract_ssh_public_key(ssh_account: serde_json::Value) -> Result<String, ExtractionError> {
    let id = &ssh_account["identifier"];
    id.as_str().map(String::from).ok_or(ExtractionError::new(Kind::MissingSshIdentifier))

}

#[derive(PartialEq)]
pub struct ExtractionError {
    inner: Box<ErrorInner>,
}
#[derive(PartialEq)]
struct ErrorInner {
    kind: Kind,
}
impl ExtractionError {
    fn new(kind: Kind) -> ExtractionError {
        ExtractionError {
            inner: Box::new(ErrorInner {
                kind,
            }),
        }
    }
}
#[derive(Debug, PartialEq)]
pub(crate) enum Kind {
    MissingUserProperty(String),
    MissingProfileOrAccount,
    MissingSshService,
    MissingSshIdentifier,
}
impl fmt::Debug for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ExtractionError")
            .field("kind", &self.inner.kind)
            .finish()
    }
}
impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inner.kind {
            Kind::MissingUserProperty(username) => write!(f, "The user property '{}' is missing from the JSON object.", username),
            Kind::MissingProfileOrAccount => f.write_str("The profile or its account object are missing."),
            Kind::MissingSshService => f.write_str("No service of type ssh was found in profile account list."),
            Kind::MissingSshIdentifier => f.write_str("The SSH service is present but missing its identifier, which is the key text."),
        }
    }
}
impl StdError for ExtractionError {
    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::MissingUserProperty(_) => "The property containing the username is missing from the JSON object.",
            Kind::MissingProfileOrAccount => "The profile or its account object are missing.",
            Kind::MissingSshService => "No service of type ssh was found in profile account list.",
            Kind::MissingSshIdentifier => "The SSH service is present but missing its identifier, which is the key text.",
        }
    }
}

impl From<ExtractionError> for String {
    fn from(ee: ExtractionError) -> String {
        format!("{}", ee)
    }
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
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Ok(String::from("yep"))
    )
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
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Err(ExtractionError::new(Kind::MissingSshIdentifier))
    )
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
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Err(ExtractionError::new(Kind::MissingSshService))
    )
}

#[test]
fn test_no_profile() {
    let json_text = r#"
      {"test": {} }
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Err(ExtractionError::new(Kind::MissingProfileOrAccount))
    )
}

#[test]
fn test_no_account() {
    let json_text = r#"
      {"test": {
        "profile": { }
        }
      }
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Err(ExtractionError::new(Kind::MissingProfileOrAccount))
    )
}

#[test]
fn test_no_user_property() {
    let json_text = r#"
      {}
    "#;
    let json = serde_json::from_str::<serde_json::Value>(json_text).unwrap();
    assert_eq!(
        extract_sshkey_from_profile("test", json),
        Err(ExtractionError::new(Kind::MissingUserProperty(String::from("test"))))
    )
}
