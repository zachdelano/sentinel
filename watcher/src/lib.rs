use rtsp_2::uri::request::URI;
use rtsp_2::request::Request;
use rtsp_2::uri::{Host,RTSP_DEFAULT_PORT};
use rtsp_2::client::Client;
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
use std::net::SocketAddr;
use rtsp_2::status::StatusCodeClass::ClientError;

// jank way of running this example:
// https://github.com/sgodwincs/rtsp-rs/blob/master/rtsp-2/examples/client.rs

// TODO:
// 1. Connect to RTSP stream
// 2. Parse the stream
// 3. Convert the parsed stream to H264 format
// 4. Perform ML on the stream
fn main() {
    // let uri_string = std::env::args()
    //     .nth(1)
    //     .unwrap_or(String::from("rtsp://192.168.1.109:5554"));
    // let uri = URI::parse_str(uri_string);
    let ipv4 = Ipv4Addr::new(192, 168, 1, 109);
    let host = Host::IPv4Address(ipv4); //uri.host().unwrap();
    // let addr: IpAddr = match host {
    //     Host::IPv4Address(ip) => IpAddr::V4(*ip),
    //     Host::IPv6Address(ip) => IpAddr::V6(*ip),
    //     _ => {
    //         eprintln!("Please provide an IP address. Hostname not supported.");
    //         std::process::exit(1);
    //     }
    // };
    let addr = IpAddr::V4(ipv4);
    let address = SocketAddr::new(addr, 5554);

    let client = Client::connect(address);

    // let client = Client


    // let uri = URI::try_from(uri_string.as_str()).expect("Invalid URI");
    // let host = uri.host().unwrap();
}
