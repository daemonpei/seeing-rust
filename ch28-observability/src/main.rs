//! Chapter 28: Logging, Configuration and Observability

use std::env;
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct Settings {
    service: String,
    port: u16,
    timeout_secs: u64,
}

impl Settings {
    fn from_env() -> Result<Self, String> {
        Ok(Self {
            service: env::var("SERVICE").unwrap_or_else(|_| "unnamed".into()),
            port: env::var("PORT")
                .ok()
                .unwrap_or_else(|| "8080".into())
                .parse()
                .map_err(|_| "PORT must be a number".to_string())?,
            timeout_secs: env::var("TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
        })
    }

    fn endpoint(&self) -> String {
        format!("{}:{}/health", self.service, self.port)
    }

    fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.timeout_secs)
    }
}

/// Deriving Debug would print the token in every log line.
struct ApiClient {
    base_url: String,
    token: String,
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("base_url", &self.base_url)
            .field(
                "token",
                &if self.token.is_empty() {
                    "[unset]"
                } else {
                    "[redacted]"
                },
            )
            .finish()
    }
}

pub trait Pingable {
    fn ping(&self) -> Result<(), String>;
}

struct Store {
    reachable: bool,
}

impl Pingable for Store {
    fn ping(&self) -> Result<(), String> {
        if self.reachable {
            Ok(())
        } else {
            Err("connection refused".into())
        }
    }
}

fn main() {
    // `try_from_default_env` reads RUST_LOG; the fallback is what you ship.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    info!(service = "inventory", port = 8080, "server starting");
    warn!(remaining = 3, "retry budget is low");
    debug!("this is invisible at RUST_LOG=info");
    error!(url = "https://api.example/health", "upstream failed");

    // ---- configuration, resolved once at startup ------------------------
    println!();
    match Settings::from_env() {
        Ok(settings) => {
            // Log the resolved settings once -- never the secrets.
            info!(
                ?settings,
                endpoint = %settings.endpoint(),
                timeout = ?settings.timeout(),
                "configuration loaded"
            );
        }
        Err(e) => error!(error = %e, "bad configuration"),
    }

    // ---- a span turns "which request was slow" into one line ------------
    println!();
    let client = ApiClient {
        base_url: "https://api.example".into(),
        token: "s3cr3t-should-never-print".into(),
    };
    info!(?client, "client configured");

    for path in ["/health", "/items", "/broken"] {
        if let Err(e) = check(&client, path) {
            warn!(error = %e, "check failed");
        }
    }

    // ---- liveness vs readiness ------------------------------------------
    println!();
    let store = Store { reachable: false };
    info!(
        liveness = health(),
        readiness = ?ready(&store),
        "probes evaluated"
    );

    // Metrics worth having before you need them:
    //   requests_total{route, status}
    //   request_duration_seconds
    //   in_flight_requests
    //   errors_total{kind}
    println!();
    info!("set RUST_LOG=debug or RUST_LOG=ch28_observability=trace to see more");
}

/// `skip(client)` keeps the client (and its token) out of the span fields.
#[instrument(skip(client), fields(url = %url))]
fn check(client: &ApiClient, url: &str) -> Result<u16, String> {
    let full = format!("{}{}", client.base_url, url);
    info!("checking");
    if url.contains("broken") {
        let e = format!("{full} is unreachable");
        error!(error = %e, "request failed");
        return Err(e);
    }
    let status = 200u16;
    info!(status, "checked");
    Ok(status)
}

/// Liveness: is the process alive? Keep it trivial.
fn health() -> &'static str {
    "ok"
}

/// Readiness: can it actually serve? Check the dependencies.
fn ready(store: &Store) -> Result<&'static str, &'static str> {
    match store.ping() {
        Ok(_) => Ok("ready"),
        Err(_) => Err("not ready"),
    }
}
