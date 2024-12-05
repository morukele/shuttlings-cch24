use actix_web::{get, web, Responder};
use serde::Deserialize;
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[derive(Deserialize)]
struct Info {
    from: String,
    key: String,
}

#[get("/2/dest")]
async fn dest(info: web::Query<Info>) -> impl Responder {
    let from_ip = Ipv4Addr::from_str(&info.from)
        .expect("Failed to parse IpV4")
        .octets();
    let key_ip = Ipv4Addr::from_str(&info.key)
        .expect("Failed to parse IpV4")
        .octets();

    let mut dest = [0; 4];
    for (idx, (f, k)) in from_ip.iter().zip(key_ip).enumerate() {
        let (val, _overflow) = f.overflowing_add(k);
        dest[idx] = val
    }

    Ipv4Addr::from(dest).to_string()
}

#[derive(Deserialize)]
struct ReverseInfo {
    from: String,
    to: String,
}

#[get("/2/key")]
async fn key(info: web::Query<ReverseInfo>) -> impl Responder {
    let from_ip = Ipv4Addr::from_str(&info.from)
        .expect("Failed to parse IpV4 address")
        .octets();
    let to_ip = Ipv4Addr::from_str(&info.to)
        .expect("Failed to parse IpV4 address")
        .octets();

    let mut key = [0; 4];
    for (idx, x) in to_ip.iter().zip(from_ip).enumerate() {
        let (val, _overflow) = x.0.overflowing_sub(x.1);
        key[idx] = val
    }

    Ipv4Addr::from(key).to_string()
}

#[get("/2/v6/dest")]
async fn dest_v6(info: web::Query<Info>) -> impl Responder {
    let from_ip = Ipv6Addr::from_str(&info.from)
        .expect("Failed to parse IpV6")
        .octets();
    let key_ip = Ipv6Addr::from_str(&info.key)
        .expect("Failed to parse IpV6")
        .octets();

    let mut dest_v6 = [0; 16];
    for (idx, x) in from_ip.iter().zip(key_ip).enumerate() {
        let res = x.0 ^ x.1;
        dest_v6[idx] = res;
    }

    Ipv6Addr::from(dest_v6).to_string()
}

#[get("/2/v6/key")]
async fn key_v6(info: web::Query<ReverseInfo>) -> impl Responder {
    let from_ip = Ipv6Addr::from_str(&info.from)
        .expect("Failed to parse IpV6")
        .octets();
    let key_ip = Ipv6Addr::from_str(&info.to)
        .expect("Failed to parse IpV6")
        .octets();

    let mut key_v6 = [0; 16];
    for (idx, x) in from_ip.iter().zip(key_ip).enumerate() {
        let res = x.0 ^ x.1;
        key_v6[idx] = res;
    }

    Ipv6Addr::from(key_v6).to_string()
}
