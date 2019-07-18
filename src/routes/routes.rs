use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

pub fn ok_response(body:String) -> String {
    return format!("HTTP/1.1 200 OK\r\n\r\n{}", body);
}

pub fn not_found_response(body:String) -> String {
    return format!("HTTP/1.1 404 OK\r\n\r\n{}", body);
}


pub fn write_to_stream(stream: &mut TcpStream, response: &String){
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


pub fn handle_get_request(mut stream: TcpStream){
    let body = fs::read_to_string("get.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", &body);
    write_to_stream(&mut stream, &response)
}

pub fn handle_post_request(mut stream: TcpStream){
    let body = fs::read_to_string("post.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", &body);
    write_to_stream(&mut stream, &response)
}

pub fn handle_patch_request(mut stream: TcpStream){
    let body = fs::read_to_string("patch.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", &body);
    write_to_stream(&mut stream, &response)
}

pub fn handle_delete_request(mut stream: TcpStream){
    let body = fs::read_to_string("delete.html").unwrap();
    let response = ok_response(body);
    write_to_stream(&mut stream, &response)
}

pub fn handle_not_found(mut stream: TcpStream){
    let body = fs::read_to_string("404.html").unwrap();
    let response = not_found_response(body);
    write_to_stream(&mut stream, &response)
}