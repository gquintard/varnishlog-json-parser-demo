use std::io::{self, BufRead};
use std::net::IpAddr;
use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    url: String,
    headers: HashMap<String, Vec<String>>,
    method: String,
    proto: String,
    #[serde(rename = "hdrBytes")]
    hdr_bytes: usize,
    #[serde(rename = "bodyBytes")]
    body_bytes: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    headers: HashMap<String, Vec<String>>,
    status: u16,
    proto: String,
    reason: String,
    #[serde(rename = "hdrBytes")]
    hdr_bytes: usize,
    #[serde(rename = "bodyBytes")]
    body_bytes: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    timestamp: f64,
}

#[derive(Serialize, Deserialize, Debug)]
enum Side {
    #[serde(rename = "backend")]
    Backend,
    #[serde(rename = "client")]
    Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    #[serde(rename = "rAddr")]
    r_addr: IpAddr,
    #[serde(rename = "rPort")]
    r_port: u16,
    sock: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Backend {
    name: String,

    #[serde(rename = "rAddr")]
    r_addr: IpAddr,
    #[serde(rename = "rPort")]
    r_port: u16,
    #[serde(rename = "connReused")]
    conn_reused: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    #[serde(rename = "type")]
    typ_: String,
    id: String,
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    req: Request,
    resp: Response,
    timeline: Vec<Event>,// Timestamp
    side: Side,
    id: String,
    vcl: String,
    client: Option<Client>,
    backend: Option<Backend>,
    storage: Option<String>,
    error: Option<String>,
    logs: Vec<String>,
    links: Vec<Link>,
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let l: Log = serde_json::from_str(&line?)?;
        println!("{} {} {}", l.resp.status, l.req.method, l.req.url);
    }
    Ok(())
}
