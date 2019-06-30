use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;


pub fn handle_get_request(stream: &mut TcpStream){
    let body = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", body);

    println!("Handling get request in routes.rs");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}