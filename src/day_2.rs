use actix_web::{get, web, Responder};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
struct Info {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[get("/2/dest")]
async fn dest(info: web::Query<Info>) -> impl Responder {
    let from_ip = info.from;
    let key_ip = info.key;

    let mut dest = [0; 4];
    for (idx, (f, k)) in from_ip.octets().iter().zip(key_ip.octets()).enumerate() {
        dest[idx] = f.overflowing_add(k).0
    }

    Ipv4Addr::from(dest).to_string()
}

#[derive(Deserialize)]
struct ReverseInfo {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[get("/2/key")]
async fn key(info: web::Query<ReverseInfo>) -> impl Responder {
    let from_ip = info.from;
    let to_ip = info.to;

    let mut key = [0; 4];
    for (idx, x) in to_ip.octets().iter().zip(from_ip.octets()).enumerate() {
        key[idx] = x.0.overflowing_sub(x.1).0;
    }

    Ipv4Addr::from(key).to_string()
}

#[derive(Deserialize)]
struct InfoV6 {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[get("/2/v6/dest")]
async fn dest_v6(info: web::Query<InfoV6>) -> impl Responder {
    let from_ip = info.from;
    let key_ip = info.key;

    let mut dest_v6 = [0; 16];
    for (idx, x) in from_ip.octets().iter().zip(key_ip.octets()).enumerate() {
        dest_v6[idx] = x.0 ^ x.1;
    }

    Ipv6Addr::from(dest_v6).to_string()
}

#[derive(Deserialize)]
struct ReverseInfoV6 {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

#[get("/2/v6/key")]
async fn key_v6(info: web::Query<ReverseInfoV6>) -> impl Responder {
    let from_ip = info.from;
    let key_ip = info.to;

    let mut key_v6 = [0; 16];
    for (idx, x) in from_ip.octets().iter().zip(key_ip.octets()).enumerate() {
        key_v6[idx] = x.0 ^ x.1;
    }

    Ipv6Addr::from(key_v6).to_string()
}
