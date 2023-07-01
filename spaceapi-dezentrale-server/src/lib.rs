#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use rand::RngCore;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{
        hyper::header::{
            ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        },
        ContentType, Header, Status,
    },
    outcome::Outcome,
    request::{self, FromRequest, Request},
    response::Response,
    serde::{json::Json, Deserialize, Serialize},
    tokio::sync::RwLock,
    Build, Rocket, State,
};
use std::{
    io::Read,
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

const SOFTWARE: &str = std::env!("CARGO_PKG_NAME");
const VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unix time")
        .as_secs()
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let key = format!("{:16x}{:16x}", rng.next_u64(), rng.next_u64());
        ApiKey(key)
    }
}

impl FromStr for ApiKey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(ApiKey(s.to_string()))
    }
}

impl From<&str> for ApiKey {
    fn from(s: &str) -> Self {
        ApiKey(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(api_key) = req.headers().get_one("X-API-Key") {
            if let Some(api_key_config) = req.rocket().state::<ApiKey>() {
                if api_key == api_key_config.0 {
                    return Outcome::Success(ApiKey(api_key.to_string()));
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, "Api key missing"))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AdminConfig {
    #[serde(default, rename = "api_key")]
    api_key: Option<ApiKey>,
    #[serde(default, rename = "enable")]
    enabled: bool,
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
    open: String,
    #[serde(default = "default_status_display_closed", rename = "closed")]
    closed: String,
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
    text: StatusDisplay,
    #[serde(default, rename = "html")]
    html: StatusDisplay,
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

pub struct SpaceState {
    pub open: bool,
}

pub struct SpaceGuard(Arc<RwLock<SpaceState>>);

impl SpaceGuard {
    pub fn new() -> Self {
        let space = SpaceState { open: false };
        SpaceGuard(Arc::new(RwLock::new(space)))
    }

    pub async fn open(&self) {
        let mut space = self.0.write().await;
        space.open = true;
    }

    pub async fn close(&self) {
        let mut space = self.0.write().await;
        space.open = false;
    }

    pub async fn is_open(&self) -> bool {
        let space = self.0.read().await;
        space.open
    }
}

impl Default for SpaceGuard {
    fn default() -> Self {
        SpaceGuard::new()
    }
}

#[post("/admin/publish/space-open")]
pub async fn open_space(_api_key: ApiKey, space: &State<SpaceGuard>) {
    space.open().await;
}

#[post("/admin/publish/space-close")]
pub async fn close_space(_api_key: ApiKey, space: &State<SpaceGuard>) {
    space.close().await;
}

/// Minimalistic implementation of the index page
#[get("/")]
pub async fn index(
    space: &State<SpaceGuard>,
    displays: &State<StatusDisplayTypes>,
    template: &State<spaceapi_dezentrale::Status>,
) -> (ContentType, String) {
    let name = &template.space;
    let logo = &template.logo;
    let status = if space.is_open().await { &displays.text.open } else { &displays.text.closed };

    let html = format!(
        r#"<html>
        <body>
            <center>
                <img src="{logo}" alt="{name}"></img>
                <div>{status}</div>
                <div><a href="https://github.com/dezentrale/spaceapi-rs">{SOFTWARE} v{VERSION}</a></div>
            </center>
        </body>
    </html>
    "#
    );
    (ContentType::HTML, html)
}

#[get("/spaceapi/v14")]
pub async fn get_status_v14<'a>(
    space: &State<SpaceGuard>,
    template: &State<spaceapi_dezentrale::Status>,
) -> Json<spaceapi_dezentrale::Status> {
    let mut status = template.inner().clone();
    status.api_compatibility = Some(vec![spaceapi_dezentrale::ApiVersion::V14]);
    status.state = Some(spaceapi_dezentrale::State {
        open: Some(space.is_open().await),
        lastchange: Some(unix_timestamp()),
        ..spaceapi_dezentrale::State::default()
    });
    Json(status)
}

#[get("/status/text")]
pub async fn get_status_text<'a>(space: &State<SpaceGuard>, displays: &State<StatusDisplayTypes>) -> (ContentType, String) {
    let status = if space.is_open().await { displays.text.open.clone() } else { displays.text.closed.clone() };
    (ContentType::Text, status)
}

#[get("/status/html")]
pub async fn get_status_html<'a>(space: &State<SpaceGuard>, displays: &State<StatusDisplayTypes>) -> (ContentType, String) {
    let status = if space.is_open().await { displays.html.open.clone() } else { displays.html.closed.clone() };
    (ContentType::HTML, status)
}

/// OPTION fallback handler required for CORS
#[options("/<_..>")]
fn options_catch_all() {}

pub struct Cors;

/// Implementation for CORS
///
/// Inspired by
/// [Stackoverflow](https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs)
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Settings",
            kind: Kind::Response,
        }
    }

    async fn on_response<'a>(&self, _request: &'a Request<'_>, response: &mut Response<'a>) {
        // Allow GET from all locations
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(), "true"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_HEADERS.as_str(), "*"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_METHODS.as_str(), "*"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"));
    }
}

pub fn serve(config: SpaceConfig) -> Rocket<Build> {
    let mut routes = routes![get_status_v14, get_status_html, index, options_catch_all];

    if config.admin.enabled {
        routes.extend(routes![open_space, close_space]);
    }

    let rocket = rocket::build()
        .attach(Cors)
        // Add loaded template for spaceapi publishing
        .manage(config.publish)
        .manage(config.status_display)
        // Add Space state
        .manage(SpaceGuard::new())
        .mount("/", routes);

    if config.admin.enabled {
        // Add API-KEY for admin interface
        rocket.manage(config.admin.api_key.unwrap_or(ApiKey::generate()))
    } else {
        rocket
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::{
        http::{Header, Status},
        local::asynchronous::Client,
        tokio,
    };

    pub(crate) fn sample_config(admin_enabled: bool) -> SpaceConfig {
        let admin = if admin_enabled {
            AdminConfig {
                api_key: Some("sesame-open".into()),
                enabled: true,
            }
        } else {
            AdminConfig {
                api_key: None,
                enabled: false,
            }
        };

        SpaceConfig {
            publish: spaceapi_dezentrale::StatusBuilder::v14("test")
                .logo("some_logo")
                .url("http://localhost")
                .contact(Default::default())
                .location(Default::default())
                .build()
                .unwrap(),
            admin,
        }
    }

    pub(crate) async fn tester(config: SpaceConfig) -> Client {
        let rocket = serve(config).ignite().await.expect("A server");
        Client::tracked(rocket).await.expect("A client")
    }

    #[tokio::test]
    async fn check_space_name() {
        let client = tester(sample_config(false)).await;
        let response = client.get(uri!(get_status_v14())).dispatch().await;
        assert_eq!(Status::Ok, response.status());

        let response: spaceapi_dezentrale::Status = response.into_json().await.unwrap();
        assert_eq!("test", response.space);
    }

    fn admin_routes() -> Vec<String> {
        vec![uri!(open_space()).to_string(), uri!(close_space()).to_string()]
    }

    #[tokio::test]
    async fn check_enabled_admin_api_not_authorized_without_api_key() {
        let client = tester(sample_config(true)).await;
        for route in admin_routes() {
            let response = client.post(route).dispatch().await;
            assert_eq!(Status::Unauthorized, response.status());
        }
    }

    #[tokio::test]
    async fn check_enabled_admin_api_not_authorized_with_invalid_api_key() {
        let client = tester(sample_config(true)).await;

        for route in admin_routes() {
            let response = client
                .post(route)
                .header(Header::new("X-API-KEY", "sesame"))
                .dispatch()
                .await;
            assert_eq!(Status::Unauthorized, response.status());
        }
    }

    #[tokio::test]
    async fn check_enabled_admin_api_authorized_with_valid_api_key() {
        let client = tester(sample_config(true)).await;

        for route in admin_routes() {
            let response = client
                .post(route)
                .header(Header::new("X-API-KEY", "sesame-open"))
                .dispatch()
                .await;
            assert_eq!(Status::Ok, response.status());
        }
    }

    #[tokio::test]
    async fn check_disabled_admin_api() {
        let client = tester(sample_config(false)).await;

        for route in admin_routes() {
            let response = client.post(route).dispatch().await;
            assert_eq!(Status::NotFound, response.status());
        }
    }
}
