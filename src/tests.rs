extern crate reqwest;

use std::thread;
use std::io::Read;
use super::main;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[test]
fn basic() {
    let _t = thread::spawn(main);
    let mut resp = reqwest::get("http://localhost:8888").unwrap();
    assert!(resp.status().is_success());
    let mut content = String::new();
    resp.read_to_string(&mut content).unwrap();
    assert_eq!(Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::from_str(content.as_str()).unwrap());
}
