use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigPlugin {
    pub name: String,
    pub version: String,
    pub language: PluginLanguage,
    #[serde(default)]
    pub tags: HashMap<String, ConfigTags>,
    #[serde(default)]
    pub listeners: HashMap<String, ConfigListener>,
    #[serde(default)]
    pub domains: HashMap<String, ConfigDomain>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginLanguage {
    Rust,
    AssemblyScript,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTags {
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigListener {
    #[serde(default)] // defaults to false
    pub form: bool,
    #[serde(flatten)]
    pub config: ConfigListenerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "listener_type")]
pub enum ConfigListenerConfig {
    Manual(ConfigManual),
    Upload(ConfigUpload),
    Cron(ConfigCron),
    Email,
    Scraper(ConfigScraper),
    Request(ConfigResponse),
    Created(ConfigRecordChanged),
    Updated(ConfigRecordChanged),
    Deleted(ConfigRecordChanged),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigManual {
    pub model: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigCron {
    pub model: String,
    pub schedule: String,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(default)]
    pub until: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigScraper {
    pub model: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigUpload {
    pub model: String,
    #[serde(default)]
    pub file_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigResponse {
    pub body: String,
    #[serde(default = "default_type")]
    pub metadata: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigRecordChanged {
    pub model: String,
    pub record: String,
}

fn default_type() -> String {
    "()".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigDomain {
    pub name: Option<String>,
    #[serde(default)]
    pub environments: HashMap<String, String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub auth: ConfigDomainAuth,
    #[serde(default)]
    pub rate_limit: Option<ConfigDomainRateLimit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigDomainRateLimit {
    pub number: i64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigDomainAuth {
    OAuth2 {
        auth_url: String,
        token_url: String,
        scopes: Vec<String>,
    },
    Token,
}
