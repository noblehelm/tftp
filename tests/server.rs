use std::thread;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::io;

use tftp::Server;
use tftp::packet::*;
use std::io::ErrorKind;

#[test]
fn test_serve_when_request_is_not_read_or_write() {
    let serve_dir = tempfile::tempdir().unwrap();
    let server_addr = "127.0.0.1:54321";
    let server = Server::new(&server_addr, serve_dir.path()).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:12345").unwrap();

    let op = vec![0, 5];
    let mut code = vec![0, 1];
    let mut message = b"file not found\0".to_vec();
    let mut bytes = op;
    bytes.append(&mut code);
    bytes.append(&mut message);

    let _ = socket
        .send_to(&bytes[..], &server_addr[..])
        .unwrap();

    assert!(server.serve().is_err());
}