const BLOCKSTACK_NODE_ENDPOINT_DEFAULT: &str = "https://core.blockstack.org";
const BLOCKSTACK_API_USERS_ENDPOINT_TEMPLATE: &str = "/v1/users/:username";
use std::env;
pub fn retrieve_user_profile(username: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = build_profile_url(&username);

    if env::var("DEBUG").is_ok() {
        eprintln!("DEBUG: Retrieving {}", url);
    }

    reqwest::get(url.as_str())?
        .error_for_status()?
        .json::<serde_json::Value>()
}

fn build_profile_url(username: &str) -> String {
    let endpoint = env::var("ENDPOINT")
        .unwrap_or_else(|_| String::from(BLOCKSTACK_NODE_ENDPOINT_DEFAULT));
    format!(
        "{}{}",
        endpoint,
        String::from(BLOCKSTACK_API_USERS_ENDPOINT_TEMPLATE).replace(":username", username))
}

#[test]
fn creates_url_from_default() {
    assert_eq!(build_profile_url("test"), String::from("https://core.blockstack.org/v1/users/test"))
}
#[test]
fn creates_url_from_env_override() {
    env::set_var("ENDPOINT", "https://example.test");
    assert_eq!(build_profile_url("test"), String::from("https://example.test/v1/users/test"));
    env::remove_var("ENDPOINT")
}