use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Method, http::Header, http::ContentType, http::uri::Origin, Request, Response, Data};
use std::io::Cursor;


pub struct CorsFairing;

impl Fairing for CorsFairing {

    fn on_request(&self, request: &mut Request, _data: &Data){
        if request.method() == Method::Options {
            let uri = Origin::parse("/").unwrap();
            request.set_uri(uri);
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // Add CORS headers to allow all origins to all outgoing requests
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type,X-User-ID"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response | Kind::Request,
        }
    }
}