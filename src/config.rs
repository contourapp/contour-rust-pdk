use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigPlugin {
    pub name: String,
    pub version: String,
    pub language: PluginLanguage,
    #[serde(default)]
    pub dimensions: HashMap<String, ConfigDimension>,
    #[serde(default)]
    pub listeners: HashMap<String, ConfigListener>,
    #[serde(default)]
    pub domains: HashMap<String, ConfigDomain>,
}

impl ConfigPlugin {
    pub fn required_envs(&self) -> HashSet<String> {
        let mut envs: HashSet<String> = self
            .domains
            .iter()
            .flat_map(|(slug, config_domain)| config_domain.required_envs(slug))
            .collect();

        for listener in self.listeners.clone() {
            if let ConfigListenerConfig::Email = listener.1.config {
                envs.extend(vec![
                    "MAILSLURP_API_KEY".to_string(),
                    "MAILSLURP_INBOX_ID".to_string(),
                ]);
            }
        }

        envs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginLanguage {
    Rust,
    AssemblyScript,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigDimension {
    pub name: Option<String>,
    #[serde(default)]
    pub managed_tags: Option<HashMap<String, String>>,
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

impl std::fmt::Display for ConfigListenerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigListenerConfig::Manual(manual) => write!(f, "Manual<{}>", manual.model),
            ConfigListenerConfig::Upload(upload) => write!(f, "Upload<{}>", upload.model),
            ConfigListenerConfig::Cron(cron) => write!(f, "Cron<{}>", cron.model),
            ConfigListenerConfig::Email => write!(f, "Email"),
            ConfigListenerConfig::Scraper(_) => write!(f, "Scraper"),
            ConfigListenerConfig::Request(request) => {
                write!(f, "Request<{}, {}>", request.body, request.metadata)
            }
            ConfigListenerConfig::Created(created) => {
                write!(f, "Created<{}<{}>>", created.model, created.record)
            }
            ConfigListenerConfig::Updated(updated) => {
                write!(f, "Updated<{}<{}>>", updated.model, updated.record)
            }
            ConfigListenerConfig::Deleted(deleted) => {
                write!(f, "Deleted<{}<{}>>", deleted.model, deleted.record)
            }
        }
    }
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

impl ConfigDomain {
    pub fn required_envs(&self, slug: &str) -> Vec<String> {
        let mut envs = vec![format!("{}_environment", slug)];

        match &self.auth {
            ConfigDomainAuth::OAuth2 { .. } => {
                envs.push(format!("{}_client_id", slug));
                envs.push(format!("{}_secret", slug));
            }
            ConfigDomainAuth::Token => {
                envs.push(format!("{}_token", slug));
            }
        }

        envs
    }
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
