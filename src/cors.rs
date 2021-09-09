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
                "https://tigum.io".to_string(),
            ],
        }
    }
}
impl Fairing for CorsFairing {

    fn on_request(&self, req: &mut Request, data: &Data) {
        if req.method() == Method::Options {
            let uri = Origin::parse("/").unwrap();
            req.set_uri(uri);
        }
    }

    fn on_response(&self, req: &Request, res: &mut Response) {
        let found_origin = req.headers().get_one("Origin");
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
        let jwt_set_cookie_header = res.headers().get_one("Set-Cookie");
        match jwt_set_cookie_header {
            Some(jwt_set_cookie_header) => { 
                let fixed_cookie = format!("{}; SameSite=None; Secure", jwt_set_cookie_header);
                res.remove_header("Set-Cookie");
                res.set_header(Header::new("Set-Cookie", fixed_cookie));
            },
            None => println!("{}", "No Set-Cookie header in res")
        }
        let allowed_origin_header = Header::new("Access-Control-Allow-Origin", allowed_origin);
        // Add CORS headers to allow all origins to all outgoing requests
        res.set_header(allowed_origin_header);
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, DELETE, PUT",
        ));
        res.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type,X-User-ID",
        ));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response | Kind::Request,
        }
    }
}
