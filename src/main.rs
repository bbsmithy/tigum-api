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
        
        routes::handle_get_request(&mut stream);

    } else if buffer.starts_with(post) {

        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(patch){
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {

        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("test.json").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
