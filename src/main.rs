use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

mod routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let patch = b"PATCH / HTTP/1.1\r\n";
    let put = b"PUT / HTTP/1.1\r\n";
    let delete = b"DELETE / HTTP/1.1\r\n";

    

    if buffer.starts_with(get) {
        
        routes::handle_get_request(stream);

    } else if buffer.starts_with(post) {

        routes::handle_post_request(stream)

    } else if buffer.starts_with(patch){
        routes::handle_patch_request(stream)
    } else {
        routes::handle_delete_request(stream)
    }
    
}
