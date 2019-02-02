const BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE: &str = "https://core.blockstack.org/v1/users/:username";

pub fn retrieve_user_profile(username: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = build_profile_url(&username);

    reqwest::get(url.as_str())?
        .error_for_status()?
        .json::<serde_json::Value>()
}

fn build_profile_url(username: &str) -> String {
    String::from(BLOCKSTACK_PROFILE_ENDPOINT_TEMPLATE).replace(":username", username)
}
