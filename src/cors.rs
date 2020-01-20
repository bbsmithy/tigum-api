use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::uri::Origin, http::Header, http::Method, Data, Request, Response};

pub struct CorsFairing {
    allow_origins: Vec<String>,
}

impl CorsFairing {
    pub fn new() -> CorsFairing {
        CorsFairing {
            allow_origins: vec![
                "http://localhost:3000".to_string(),
                "https://devkeep.io".to_string(),
            ],
        }
    }
}

impl Fairing for CorsFairing {
    fn on_request(&self, request: &mut Request, _data: &Data) {
        if request.method() == Method::Options {
            let uri = Origin::parse("/").unwrap();
            request.set_uri(uri);
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let found_origin = request.headers().get_one("Origin");
        let allowed_origin = match found_origin {
            Some(origin) => {
                let string_origin = String::from(origin);
                if self.allow_origins.contains(&string_origin) {
                    string_origin
                } else {
                    "none".to_string()
                }
            }
            None => "none".to_string(),
        };

        let allowed_origin_header = Header::new("Access-Control-Allow-Origin", allowed_origin);

        // Add CORS headers to allow all origins to all outgoing requests
        response.set_header(allowed_origin_header);
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, DELETE, PUT",
        ));
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
