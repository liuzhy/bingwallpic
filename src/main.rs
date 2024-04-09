use std::{io::{Read, Write}, net::TcpStream};
use native_tls::TlsConnector;
use serde_json::Value;
fn main() {
    let host = "www.bing.com";
    let port = "443";
    let api = "/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-us";
    let rep1 = get_body(get_https_response(host,api,port));

    let buf = String::from_utf8_lossy(&rep1);
    let v: Value = serde_json::from_str(&buf).unwrap();
    let imgurl = v["images"][0]["url"].as_str().unwrap();

    let start_index = imgurl.find("=").unwrap();
    let end_index = imgurl.find("&").unwrap();
    let img_name = &imgurl[start_index+1..end_index];

    let resp2 = get_body(get_https_response(host,imgurl,port));
    std::fs::write(img_name, &resp2).unwrap();
    println!("{} saved", img_name);
}

fn get_body(response: Vec<u8>) -> Vec<u8> {
    let double_crlf = b"\r\n\r\n";
    if let Some(index) = response.windows(double_crlf.len()).position(|window| window == double_crlf) {
        response[index + double_crlf.len()..].to_vec()
    } else {
        [].to_vec()
    }
}

fn get_https_response(host: &str, url: &str, port: &str) -> Vec<u8> {
    let stream = TcpStream::connect(format!("{}:{}",host,port)).unwrap();
    let connector = TlsConnector::new().unwrap();
    let mut socket = connector.connect(host, stream).unwrap();
    let header = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", 
        url, host
    );
    socket.write_all(header.as_bytes()).unwrap();

    let mut buf = Vec::new();
    let _ = socket.read_to_end(&mut buf).unwrap();
    buf
}
