use std::fs;
use ron::de::from_str;
use serde::Deserialize;
use serde_json::Value;
use regex::Regex;
use reqwest::StatusCode;

#[derive(Debug, Deserialize)]
struct Domain {
    domain: String,
    host: String,
    url: String,
    password: String,
}

impl ToString for Domain {
    fn to_string(&self) -> String {
        if self.host.is_empty() || self.host.eq("@") {
            format!("{}", self.domain)
        }
        else {
            format!("{}.{}", self.host, self.domain)
        }
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    domains: Vec<Domain>,
}

const IP_RESOLVER_URL: &str = "http://checkip.amazonaws.com";
const DOMAIN_RESOLVER_URL: &str = "https://dns.google.com/resolve";

fn main() {
    let config = read_config("config.ron")
        .expect("Unable to read config file!");
    let ip = resolve_ip();
    for domain in config.domains {
        let resolved_domain = resolve_domain(&domain);
        if !ip.eq(&resolved_domain) {
            if request_update(&domain, &ip) {
                println!("Updated {} to {}", domain.to_string(), ip);
            }
            else {
                println!("Failed to update {} to {}", domain.to_string(), ip);
            }
        }
    }
}

fn resolve_ip() -> String {
    let resp = reqwest::blocking::get(IP_RESOLVER_URL)
        .expect("Unable to resolve ip provider!");
    let body = resp.text()
        .expect("Unable to retrieve response text");
    body.trim().to_string()
}

fn resolve_domain(domain: &Domain) -> String {
    let query = format!(
        "{}?name={}&type=A",
        DOMAIN_RESOLVER_URL,
        domain.to_string(),
    );
    let resp = reqwest::blocking::get(query)
        .expect("Unable to resolve ip provider!");
    let body = resp.text()
        .expect("Unable to retrieve response json");
    let json: Value = serde_json::from_str(&body)
        .expect("Unable to parse into json!");
    String::from(json["Answer"][0]["data"]
        .as_str().take()
        .expect("Unable to parse into string!"))
}

fn request_update(domain: &Domain, ip: &String) -> bool {
    let query = format!(
        "{}/update?domain={}&host={}&ip={}&password={}",
        domain.url,
        domain.domain,
        domain.host,
        ip,
        domain.password,
    );
    let resp = reqwest::blocking::get(query)
        .expect("Unable to resolve ip provider!");
    if let StatusCode::OK = resp.status() {
        let body = resp.text()
            .expect("Unable to retrieve response text");
        let exp = Regex::new(r"<ErrCount>(\d+)</ErrCount>")
            .expect("Unable to parse regex!");
        let caps = exp.captures(&body)
            .expect("Unable to capture regex!");
        let status = caps.get(1)
            .map_or("", |group| group.as_str());
        return status.eq("0")
    }
    false
}

fn read_config(path: &str) -> Result<Config, ron::Error> {
    from_str(&fs::read_to_string(path)?)
}
