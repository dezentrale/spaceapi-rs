use rand::RngCore;
use rocket::serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::{io::Read, time::Duration};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiKey(pub String);

impl ApiKey {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let key = format!("{:16x}{:16x}", rng.next_u64(), rng.next_u64());
        ApiKey(key)
    }
}

impl std::str::FromStr for ApiKey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(ApiKey(s.to_string()))
    }
}

impl From<&str> for ApiKey {
    fn from(s: &str) -> Self {
        ApiKey(s.to_string())
    }
}

fn deserialize_duration_secs_from_string<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|err| D::Error::custom(format!("{err}")))?;
    Ok(Duration::from_secs(value))
}

fn deserialize_duration_millis_from_string<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|err| D::Error::custom(format!("{err}")))?;
    Ok(Duration::from_millis(value))
}

fn default_keep_open_interval() -> Duration {
    Duration::from_secs(300)
}

fn default_tick_interval() -> Duration {
    Duration::from_millis(100)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminConfig {
    #[serde(default, rename = "api_key")]
    pub api_key: Option<ApiKey>,
    #[serde(default, rename = "enable")]
    pub enabled: bool,
    #[serde(
        default = "default_keep_open_interval",
        rename = "keep_open_interval",
        deserialize_with = "deserialize_duration_secs_from_string"
    )]
    pub keep_open_interval: Duration,
    #[serde(
        default = "default_tick_interval",
        rename = "tick_interval",
        deserialize_with = "deserialize_duration_millis_from_string"
    )]
    pub tick_interval: Duration,
}

impl Default for AdminConfig {
    fn default() -> Self {
        AdminConfig {
            api_key: None,
            enabled: false,
            keep_open_interval: Duration::from_secs(300),
            tick_interval: Duration::from_millis(100),
        }
    }
}

fn default_status_display_open() -> String {
    "open".to_string()
}

fn default_status_display_closed() -> String {
    "closed".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDisplay {
    #[serde(default = "default_status_display_open", rename = "open")]
    pub open: String,
    #[serde(default = "default_status_display_closed", rename = "closed")]
    pub closed: String,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        StatusDisplay {
            open: "open".to_string(),
            closed: "closed".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatusDisplayTypes {
    #[serde(default, rename = "text")]
    pub text: StatusDisplay,
    #[serde(default, rename = "html")]
    pub html: StatusDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceConfig {
    #[serde(rename = "publish")]
    pub publish: spaceapi_dezentrale::Status,
    #[serde(default, rename = "admin")]
    pub admin: AdminConfig,
    #[serde(default, rename = "status_display")]
    pub status_display: StatusDisplayTypes,
}

impl SpaceConfig {
    pub fn load<P>(path: P) -> Result<SpaceConfig, String>
    where
        P: AsRef<std::path::Path> + std::fmt::Display,
    {
        log::info!("Read config file `{}`", path);
        let mut file = std::fs::File::open(path).map_err(|err| format!("Can't open file: {err:?}"))?;
        let mut file_buf = vec![];
        file.read_to_end(&mut file_buf)
            .map_err(|err| format!("Can't read file: {err:?}"))?;

        let mut config = serde_yaml::from_slice::<SpaceConfig>(&file_buf)
            .map_err(|err| format!("Can't parse space config: {err}"))?;
        // Clear state
        config.publish.state = None;

        if config.admin.api_key.is_none() {
            let key = ApiKey::generate();
            if config.admin.enabled {
                log::warn!("API key isn't set. Generated a random one: {}", key.0);
            }
            config.admin.api_key = Some(key);
        }
        Ok(config)
    }
}
