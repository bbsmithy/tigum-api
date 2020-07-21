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


        let headers = response.headers().iter();
        headers.for_each(|header| {
            println!("{}", header)
        });
        let jwt_set_cookie_header = response.headers().get_one("Set-Cookie");

        match jwt_set_cookie_header {
            Some(jwt_set_cookie_header) => { 
                let fixed_cookie = format!("{}; SameSite=None; Secure", jwt_set_cookie_header);
                println!("Fixed cookie {}", fixed_cookie);
                response.remove_header("Set-Cookie");
                response.set_header(Header::new("Set-Cookie", fixed_cookie));
                let set_cookie = response.headers().get_one("Set-Cookie").unwrap();
                println!("The new Set-Cookie header {}", set_cookie);
            },
            None => println!("{}", "No Set-Cookie header in response")
        }


        let allowed_origin_header = Header::new("Access-Control-Allow-Origin", allowed_origin);

        // response.set_header(Header::new("Set-Cookie", fixed_cookie));

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
