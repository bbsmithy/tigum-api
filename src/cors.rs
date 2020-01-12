use rocket::fairing::{Fairing, Info, Kind};
use rocket::{
    http::uri::Origin, http::ContentType, http::Header, http::Method, Data, Request, Response,
};

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn on_request(&self, request: &mut Request, _data: &Data) {
        if request.method() == Method::Options {
            let uri = Origin::parse("/").unwrap();
            request.set_uri(uri);
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // Add CORS headers to allow all origins to all outgoing requests
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type,X-User-ID",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response | Kind::Request,
        }
    }
}
