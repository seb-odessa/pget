extern crate reqwest;

use std::env;
use std::io::Read;
use std::fs::File;
use reqwest::Error;



fn load_cert() -> reqwest::Result<reqwest::Certificate> {
    let mut buf = Vec::new();
    File::open("root.der")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    reqwest::Certificate::from_der(&buf)
}

fn get(url: &str) -> Result<String, Error> {
    let proxy = reqwest::Proxy::http("https://128.199.126.183:3128")?;
    let cert = load_cert()?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .add_root_certificate(cert)
        .danger_disable_hostname_verification()
        .build()?;



    let mut response = client.get(url).send()?;
    assert!(response.status().is_success());

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();
    return Ok(content);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if !args.len() > 1 {
        let url = &args[1];
        println!("Get {}", &url);
        match get(&url) {
            Ok(content) => println!("{}", content),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("expected URL as argument");
    }

}
