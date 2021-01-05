use std::net::{TcpStream, IpAddr};
use std::io;
use std::io::prelude::*;
use native_tls::{TlsConnector, TlsStream};
use std::collections::HashMap;
use miniurl::Url;
use std::io::{Read, Write};
use std::error::Error;
use std::fmt;

static GET: &str = "GET";
static POST: &str = "POST";
static PUT: &str = "PUT";
static UPDATE: &str = "UPDATE";

pub struct HttpClient {
    url: Url
}


// let client = util::http::HttpClient::new("localhost").unwrap();
// client.post("/test", "data")

impl HttpClient {
    // Take in a url and find domain using 
    pub fn new(url: &str) -> HttpClient {
        let parsed_url = Url::parse(url);
        HttpClient {
            url: parsed_url
        }
    }

    fn setup_request(&self) -> Result<TlsStream<TcpStream>, HttpClientError> {
        if let Some(h) = &self.url.host {
            println!("{:?}", &self.url);
            let connector = TlsConnector::new().unwrap();
            let stream = TcpStream::connect(format!("{}:{}", &h, &self.url.port)).unwrap();
            let tls_stream = connector.connect(&h, stream).unwrap();
            Ok(tls_stream)
        } else {
            Err(HttpClientError { reason: "Failed to parse out host of connection".to_string() })
        }
       
    }

    // Used for requests with bodys
    // fn build_header_body(&self, method: &str, body: &String) -> String {
    //     let mut headers = String::new();
    //     headers.push_str(&format!("{} / HTTP/1.1\r\n", method));
    //     headers.push_str(&format!("Host: {}:{}\r\n", &self.url.path, &self.url.port));
    //     headers.push_str(&format!("Connection: Close\r\n"));
    //     headers.push_str(&format!("Content-Length: {}\r\n", body.len()));
    //     // for (i,k) in &self.headers {
    //     //     headers.push_str(&format!("{}: {}\r\n",i,k));
    //     // }
    //     headers.push_str("\r\n");
    //     headers
    // }

    // Used for requests without bodys
    fn build_header(&self, method: &str) -> Result<String, HttpClientError> {
        if let Some(h) = &self.url.host {
            let mut headers = String::new();
            headers.push_str(&format!("{} / HTTP/1.0\r\n", method));
            headers.push_str(&format!("Host {}:{}\r\n", &h, &self.url.port));
            headers.push_str(&format!("Connection: Close\r\n"));
            // for (i,k) in &self.headers {
            //     headers.push_str(&format!("{}: {}\r\n",i,k));
            // }
            headers.push_str("\r\n");
            Ok(headers)
        } else {
            Err(HttpClientError { reason: "Failed to parse url path".to_string() })
        }
        
    }

    pub fn get(&self) {
        match self.setup_request() {
            Ok(mut stream) => {
                let headers = &self.build_header(GET);
                match headers {
                    Ok(hds) => {
                        println!("{}", hds);
                        stream.write_all(hds.as_bytes()).unwrap();
                        let mut res = vec![];
                        stream.read_to_end(&mut res).unwrap();
                        println!("{}", String::from_utf8_lossy(&res));
                    },
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }
        
    }

    // pub fn post(&self, body: String) {
    //     match self.setup_request() {
    //         Ok(mut stream) => {
    //             let headers = &self.build_header(POST, );
    //             println!("{}", headers);
    //             stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    //             let mut res = vec![];
    //             stream.read_to_end(&mut res).unwrap();
    //             println!("{}", String::from_utf8_lossy(&res));
    //             // stream.write_all(headers.as_bytes()).unwrap();
    //             // let mut res = vec![];
    //             // stream.read_to_end(&mut res).unwrap();
    //             // println!("{}", String::from_utf8_lossy(&res));
    //         },
    //         Err(err) => {
    //             println!("{}", err);
    //         }
    //     }
        
    // }

    fn send(stream: TlsStream<TcpStream>, headers: String, body: String) {
        // &stream.write(header.as_bytes())?;
        // if let Some(ref body) = self.body {
        //     stream.write(body.as_slice())?;
        // }
        // stream.flush()?;
        // let mut res :Vec<u8>= Vec::new();
        // stream.read_to_end(&mut res)?;
        // let back = Response::new(res)?;
        // Ok(back)
    }

}

#[derive(Debug)]
struct HttpClientError {
    reason: String
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.reason)
    }
}

impl Error for HttpClientError {
}