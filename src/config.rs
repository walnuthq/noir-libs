use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub company_name: String,
    pub company_tld: String,
    pub repository_url: String,
}
