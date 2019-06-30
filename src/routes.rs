use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

pub fn write_to_stream(stream: &mut TcpStream, response: &String){
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


pub fn handle_get_request(mut stream: TcpStream){
    let body = fs::read_to_string("get.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", body);
    write_to_stream(&mut stream, &response)
}

pub fn handle_post_request(mut stream: TcpStream){
    let body = fs::read_to_string("post.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", body);
    write_to_stream(&mut stream, &response)
}