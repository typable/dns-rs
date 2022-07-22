use env_logger::Builder;
use env_logger::Target;
use log::LevelFilter;
use regex::Regex;
use serde_json::Value;
use std::process;

use dns_rs::error::Error;
use dns_rs::Config;
use dns_rs::Domain;
use dns_rs::Provider;
use dns_rs::Result;
use dns_rs::DOMAIN_RESOLVER;
use dns_rs::IP_RESOLVER;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    Builder::new()
        .format_timestamp_secs()
        .filter_module("dns_rs", LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    let config = Config::acquire().unwrap_or_else(|err| {
        error!("unable to load config file! Reason: {}", err);
        process::exit(1);
    });
    info!("resolving ip addr...");
    let ip_addr = resolve_ip_addr().await.unwrap_or_else(|err| {
        error!("unable to resolve ip addr! Reason: {}", err);
        process::exit(1);
    });
    info!("ip addr: {}", ip_addr);
    info!("checking domains");
    for domain in config.domains {
        let domain_id = domain.id();
        info!("[{}]", domain_id);
        info!("  resolving domain addr...");
        let domain_addr = match resolve_domain_addr(&domain).await {
            Ok(domain_addr) => domain_addr,
            Err(err) => {
                error!("  unable to resolve domain addr! Reason: {}", err);
                error!("  status: FAILED");
                continue;
            }
        };
        info!("  domain addr: {}", domain_addr);
        if !ip_addr.eq(&domain_addr) {
            warn!("  domain addr does not match with ip addr!");
            let update = domain.update.clone().unwrap_or(true);
            if !update {
                warn!("  updating disabled");
                warn!("  status: IGNORED");
                continue;
            }
            info!("  updating domain addr...");
            if let Err(err) = update_domain(&config.provider, &domain, &ip_addr).await {
                error!("  unable to update domain addr! Reason: {}", err);
                error!("  status: FAILED");
            } else {
                info!("  status: UPDATED");
            }
        } else {
            info!("  status: UP TO DATE");
        }
    }
    info!("done");
}

async fn resolve_ip_addr() -> Result<String> {
    let mut resp = surf::get(IP_RESOLVER).await?;
    let body = resp.body_string().await?;
    Ok(body.trim().to_string())
}

async fn resolve_domain_addr(domain: &Domain) -> Result<String> {
    let url = format!("{}?name={}&type=A", DOMAIN_RESOLVER, domain.id());
    let mut resp = surf::get(url).await?;
    let body: Value = resp.body_json().await?;
    let domain_addr = match body["Answer"][0]["data"].as_str().take() {
        Some(domain_addr) => domain_addr,
        None => return Err(Error::new("unable to determine domain addr!")),
    };
    Ok(domain_addr.to_string())
}

async fn update_domain(provider: &Provider, domain: &Domain, ip_addr: &str) -> Result<()> {
    let url = provider.build(&domain, &ip_addr);
    let mut resp = surf::get(url).await?;
    let body = resp.body_string().await?;
    let exp = Regex::new(r"<ErrCount>(\d+)</ErrCount>")?;
    let caps = match exp.captures(&body) {
        Some(caps) => caps,
        None => {
            return Err(Error::new(""));
        }
    };
    let status = caps.get(1).map_or("", |cap| cap.as_str());
    if !status.eq("0") {
        return Err(Error::new("updating the domain addr failed!"));
    }
    Ok(())
}
