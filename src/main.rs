use std::fs;
use std::collections::HashMap;
use props_rs::{parse, to_map};
use serde_json::Value;
use regex::Regex;
use reqwest::StatusCode;

fn main() {
    let props = env("config.properties");
    let ip = fetch_ip(&props);
    let resolve = fetch_resolve(&props);
    println!("local: {}", ip);
    println!("domain: {}", resolve);
    if ip.eq(&resolve) {
        println!("status: UP_TO_DATE");
    }
    else {
        println!("status: OUTDATED");
        let success = fetch_update(&props, &ip);
        println!("updated: {}", success);
    }
}

fn fetch_ip(props: &HashMap<String, String>) -> String {
    let provider_ip = props.get("dns.provider.ip").take()
        .expect("Unable to retrieve property!");
    let resp = reqwest::blocking::get(provider_ip)
        .expect("Unable to resolve ip provider!");
    let body = resp.text()
        .expect("Unable to retrieve response text");
    body.trim().to_string()
}

fn fetch_resolve(props: &HashMap<String, String>) -> String {
    let provider_resolve = props.get("dns.provider.resolve").take()
        .expect("Unable to retrieve property!");
    let host = props.get("dns.host").take()
        .expect("Unable to retrieve property!");
    let domain = props.get("dns.domain").take()
        .expect("Unable to retrieve property!");
    let query = format!("{}?name={}.{}&type=A", provider_resolve, host, domain);
    let resp = reqwest::blocking::get(query)
        .expect("Unable to resolve ip provider!");
    let body = resp.text()
        .expect("Unable to retrieve response json");
    let json: Value = serde_json::from_str(&body)
        .expect("Unable to parse into json!");
    String::from(json["Answer"][0]["data"].as_str().take().expect("Unable to parse into string!"))
}

fn fetch_update(props: &HashMap<String, String>, ip: &String) -> bool {
    let provider_domain = props.get("dns.provider.domain").take()
        .expect("Unable to retrieve property!");
    let domain = props.get("dns.domain").take()
        .expect("Unable to retrieve property!");
    let host = props.get("dns.host").take()
        .expect("Unable to retrieve property!");
    let password = props.get("dns.provider.domain.password").take()
        .expect("Unable to retrieve property!");
    let query = format!("{}/update?domain={}&host={}&ip={}&password={}", provider_domain, domain, host, ip, password);
    let resp = reqwest::blocking::get(query)
        .expect("Unable to resolve ip provider!");
    if let StatusCode::OK = resp.status() {
        let body = resp.text()
            .expect("Unable to retrieve response text");
        let exp = Regex::new(r"<ErrCount>(\d+)</ErrCount>").expect("Unable to parse regex!");
        let caps = exp.captures(&body).expect("Unable to capture regex!");
        let status = caps.get(1).map_or("", |group| group.as_str());
        return status.eq("0")
    }
    false
}

fn env(file: &str) -> HashMap<String, String> {
    let content = fs::read_to_string(file)
        .expect("Unable to read file!");
    let parsed = parse(content.as_bytes())
        .expect("Unable to parse file!");
    to_map(parsed)
}
