pub mod error;

use serde::Deserialize;
use std::fs;

use error::Error;

pub const APP_NAME: &str = "dns-rs";
pub const IP_RESOLVER: &str = "http://checkip.amazonaws.com";
pub const DOMAIN_RESOLVER: &str = "https://dns.google.com/resolve";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub provider: Provider,
    pub domains: Vec<Domain>,
}

impl Config {
    pub fn acquire() -> Result<Self> {
        let mut config_path = match dirs::config_dir() {
            Some(config_path) => config_path,
            None => return Err(Error::new("unable to determine config path!")),
        };
        config_path.push(APP_NAME);
        config_path.push("config.toml");
        let raw = fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&raw)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Provider {
    pub host: String,
    pub path: String,
    pub args: String,
}

impl Provider {
    pub fn build(&self, domain: &Domain, ip_addr: &str) -> String {
        let mut args = self.args.clone();
        args = args.replace("%h", &domain.host);
        args = args.replace("%s", &domain.subdomain.clone().unwrap_or("@".to_string()));
        args = args.replace("%i", ip_addr);
        args = args.replace("%p", &domain.password);
        format!("{}{}?{}", self.host, self.path, args)
    }
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub host: String,
    pub subdomain: Option<String>,
    pub password: String,
    pub update: Option<bool>,
}

impl Domain {
    pub fn id(&self) -> String {
        format!(
            "{}{}{}",
            self.subdomain.clone().unwrap_or_default(),
            if self.subdomain.is_some() { "." } else { "" },
            self.host
        )
    }
}
